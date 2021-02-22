use num::Complex;
// use std::str::FromStr;
​
/// Determine if `c` is in the Mandelbrot set or not, based in part
/// on the `limit` parameter, which specifies how many "attempts"
/// the program gets to figure it out.
/// 
/// If `c` is not a member, returns `Some(i)` where `i` is the number
/// of iterations it took for `c` to leave the circle of radius 2 centered
/// at the origin. If `c` is a member of the set, i.e., if we reached the 
/// iteration limit without being able to prove that `c` is _not_ a member,
/// return `None`
fn in_mandelbrot_set(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
​
    for i in 0..limit {
        z = z * z + c; // where is this coming from? Black box magic maths of mandelbrots.
​
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
​
    None
}
​
//Break up (Parse) command line str into peices we can use in parse_bounds 
//Todo
​
//Only dealing with the `1000x750` portion of `cargo run mandelbrot.png 1000x750 -1.20,0.35 -1,0.20`
// parse the str bounds provided from command line into usizes that can be passed to next functions
fn parse_bounds(bounds: &str) -> Option<(usize, usize)> {
    if let Some(index) = bounds.find('x') {
        match (&bounds[..index].parse::<usize>(), &bounds[index+1..].parse::<usize>()) {
            (Ok(l), Ok(r)) => Some((*l, *r)),
            _ => None,
        }
    } else {
        None
    }
}
​
// fn parse_pair<T: FromStr>(pair: &str, separator: char) -> Option<(T, T)> {
//     if let Some(index) = pair.find(separator) {
//         match (T::from_str(&pair[..index]), T::from_str(&pair[index+1..])) {
//             (Ok(l), Ok(r)) => Some((l, r)),
//             _ => None,
//         }
//     } else {
//         None
//     }
// }
​
// fn parse_complex(pair: &str) -> Option<Complex<f64>> {
//     if let Some((re, im)) = parse_pair(pair, ',') {
//         Some(Complex { re, im })
//     } else {
//         None
//     }
// }
​
// decimal<,>decimal
fn parse_complex_bounds(bounds: &str) -> Option<Complex<f64>> {
    if let Some(index) = bounds.find(',') {
        match(&bounds[..index].parse::<f64>(), &bounds[index+1..].parse::<f64>()) {
            (Ok(l), Ok(r)) => Some(Complex { re: *l, im: *r }),
            _ => None,
        }
    } else {
        None
    }
}
​
/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),   // size of the image 
    pixel: (usize, usize),    // coordinates of the pixel 
    upper_left: Complex<f64>, // upper left corner of the complex plane
    lower_right: Complex<f64> // lower right corner of the complex plane
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
​
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}
​
/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
/// A buffer of pixels can be represented as a Vec<u8>s
/// Each u8 in the vector is a "grayscale" pixel 
/// A value of 0 is completely white; A value of 255 is completely black
/// 
/// The `bounds` arguments gives the width and height of the `pixels` buffer,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
/// 
/// The color gradient of any one pixel is determined by subtracting the
/// number of iterations it took to determine if the number is in the 
/// Mandelbrot set from the `limit` (the max number of iterations used to 
/// determine if the number is a member of the Mandelbrot set).
///
/// We can have the render return the `Vec<u8>` 
/// We can pass in the `Vec<u8>` from outside the function using an `&mut` 
fn render(
    pixel_buffer: &mut [u8],
    bounds: (usize, usize), //determines image size
    upper_left: Complex<f64>, 
    lower_right: Complex<f64>
) {
    assert!(pixel_buffer.len() == bounds.0 * bounds.1);
​
    // Iterate through all the buffer pixels
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            // convert each pixel to a complex with the `pixel_to_point` fn 
            // take the resulting complex and pass it to the `in_mandelbrot_set` fn 
            // that will determine how much we need to color the current pixel 
            // (if at all)
            let c = pixel_to_point(bounds, (col, row) , upper_left, lower_right);
            let pixel = match in_mandelbrot_set(c, 255) {
                Some(i) => 255 - i as u8,
                None => 0
            };
            //index in pixel_buffer is determined from row * bounds.0 + col
            let index = row * bounds.0 + col;
            pixel_buffer[index] = pixel;
        }
    }
}
​
/// Write the contents of the `pixel` buffer, whose dimensions are given
/// by `bounds`, to the specified file.
use image::{ save_buffer, ColorType };
​
fn write_image(
    filename: &str,
    bounds: (usize, usize),
    pixel_buffer: &[u8],
) {
    save_buffer(filename, pixel_buffer, bounds.0, bounds.1, ColorType::L8).unwrap()
}
​
// Write the main function that 
// 1. Accept arguments from the user 
// 2. Call the parse functions on those arguments
// 3. Pass those args to the render function 
// 4. Call `write_image` to write the image 

fn main() {
    // Accept arguments from the user 
        let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Enter attributes for mandelbrot filename widthxheight upper_left lower_right"
        ).unwrap();

        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        ).unwrap();

        std::process::exit(1);
    }


    // Call the paser function on those arguments
    let bounds = parse_bounds(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex_bounds(&args[3]).expect("error parsing upper left point");
    let lower_right = parse_complex_bounds(&args[4]).expect("error parsing lower right point");
    let mut pixel_buffer = vec![0; bounds.0 * bounds.1];


    // Pass the parsed args to the render function
    render(pixel_buffer, bounds, upper_left, lower_right);

    // Call write_image to write the image
     write_image(&args[1], &pixel_buffer, bounds).expect("error creating the image file");
​
}​

#[test]
fn test_pixel_to_point() {
    let answer = pixel_to_point(
        (100, 100), 
        (25, 75),
        Complex { re: -1.0, im:  1.0 },
        Complex { re:  1.0, im: -1.0 }
    );
​
    let expected =  Complex { re: -0.5, im: -0.5 };
​
    assert_eq!(answer, expected);
}
​
// #[test]
// fn test_parse_pair() {
//     assert_eq!(parse_pair::<i32>("", ','), None);
//     assert_eq!(parse_pair::<i32>(",", ','), None);
//     assert_eq!(parse_pair::<usize>("1000x750", 'x'), Some((1000, 750)));
//     assert_eq!(parse_pair("0.5x0.1", 'x'), Some((0.5, 0.1)));
// }




