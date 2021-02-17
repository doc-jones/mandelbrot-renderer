# Building a (Parallel) Mandelbrot Renderer in Rust 

The Mandelbrot set is a set of complex numbers that, when [rendered][mandelbrot-image], produces really aesthetic
recursive fractals. Writing a program that renders these fractals is what is often jokingly
referred to as an "embarrassingly parallel problem". 

But before we get to the whole "making it parallel" part, we need the core logic of our 
renderer in place first! 

## Implementing a Single-Threaded Renderer

The object of our renderer is to output an image of a Mandelbrot fractal, given some inputs
such as the file name of the image, the dimensions of the image, etc.

Implementing our renderer will require working with complex numbers, but since the details of
how we need to manipulate these complex numbers is out of scope for this project, we're
going to gloss over these details and use the provided functions that handle those details for us.

- [ ] Survey the functionality that's already provided to us, and understand it at much as we 
need to to build more functionality on top of them (or more if you're so inclined).

A command to run our Mandelbrot renderer might look like this:
```
cargo run mandelbrot.png 1000x750 -1.20,0.35 -1,0.20
```

The first argument to our program is the name of the image file we want to output our image to.
The second is a string denoting the dimensions of the image. The third and forth arguments are
points on the complex plane which designate the area in which our rendered Mandelbrot fractal 
covers. 

Our program thus needs to be able to parse these arguments.

- [ ] Implement logic to parse a command to our renderer with the specified arguments.

Once that's done, we'll need to figure out how to render the correct pixels in the Mandelbrot 
fractal to a pixel buffer.

- [ ] Render the image by writing the pixels to a buffer.

Finally, once all the pixels have been written to our buffer, the last step is to write the
contents of the buffer to an image file.

- [ ] Output the buffer contents to a PNG file.

## Implementing a Multithreaded Renderer

Now that we have successfully implemented a single-threaded Mandelbrot renderer, let's make
it multithreaded!

- [ ] Make our Mandelbrot renderer multithreaded!

[mandelbrot-image]: https://en.wikipedia.org/wiki/Mandelbrot_set#/media/File:Mandel_zoom_00_mandelbrot_set.jpg
