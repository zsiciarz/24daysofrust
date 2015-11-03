# Day 17 - from_fn

> Relevancy: 1.4 stable

In Rust there is no concept of a constructor as a language feature, like for example in C++. However there is a strong convention (and mentioned in [the guidelines](http://aturon.github.io/ownership/constructors.html)) to use a static method called `new` as the constructor. This works well, but you can have only one function called `new` in the `impl` - there is no method overloading. So are we out of luck if we want to have different constructors? No! But arguably different purposes should imply different method names, so another convention is to prefix extra constructors with `with_` (such as [Vec::with_capacity](http://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity)) or `from_`, if the constructor does some kind of a conversion.

A few types in the standard library and some third-party crates provide a `from_fn` constructor. This curious method usually takes as arguments some sort of *dimensions* and a closure that will generate values.

Matrices
--------

In the [nalgebra crate](https://siciarz.net/24-days-of-rust-nalgebra/) there is a `DMat` type representing a matrix which dimensions are known at runtime. We can build a matrix using the `from_fn` constructor too. Let's create a triangular matrix:

```rust
extern crate nalgebra;

use nalgebra::DMat;

let mat: DMat<u32> = DMat::from_fn(7, 7, |i, j| if j <= i { 1 } else { 0 });
println!("{:?}", mat);
```

The first two arguments to `from_fn` are numbers of rows and columns; this means the closure must take two arguments - indices of the current row and column. And here's our matrix:

```sh
$ cargo run
1 0 0 0 0 0 0
1 1 0 0 0 0 0
1 1 1 0 0 0 0
1 1 1 1 0 0 0
1 1 1 1 1 0 0
1 1 1 1 1 1 0
1 1 1 1 1 1 1
```

Images
------

Bur we're not limited to mathematical objects. For example the [image crate](day12.md) provides a buffer that can generate the image with the `from_fn` call.

```rust
extern crate image;

use image::Pixel;

let buffer = image::ImageBuffer::from_fn(512u32, 512u32, |x: u32, y: u32| {
    Pixel::from_channels((x * y % 256) as u8, (y % 256) as u8, (x % 256) as u8, 255)
});
let img = image::DynamicImage::ImageRgba8(buffer);
let mut out = File::create(&Path::new("out_pattern.png")).unwrap();
let _ = img.save(&mut out, image::PNG);
```

We're working in two dimensions in the same manner as with the `DMat` type. And here's the generated image:

![pattern](//i.imgur.com/G3JuGR0.png)
