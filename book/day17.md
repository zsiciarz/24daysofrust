# Day 17 - from_fn

> Relevancy: **outdated**

In Rust there is no concept of a constructor as a language feature, like for example in C++. However there is a strong convention (and mentioned in [the guidelines](http://aturon.github.io/ownership/constructors.html)) to use a static method called `new` as the constructor. This works well, but you can have only one function called `new` in the `impl` - there is no method overloading. So are we out of luck if we want to have different constructors? No! But arguably different purposes should imply different method names, so another convention is to prefix extra constructors with `with_` (such as [Vec::with_capacity](http://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity)) or `from_`, if the constructor does some kind of a conversion.

A few types in the standard library and some third-party crates provide a `from_fn` constructor. This curious method usually takes as arguments some sort of *dimensions* and a closure that will generate values.

Vectors
-------

One of the typical methods to create a vector is to call `collect()` on an iterator like below:

    :::rust
    let v = range(0, 10u).map(|x| x * 3).collect::<Vec<_>>();
    println!("{}", v);

We can shorten that a bit with the [from_fn](http://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_fn) method. It takes the expected vector size and a closure that uses current index to generate vector elements.

    :::rust
    let v = Vec::from_fn(10, |x| x * 3);

This is roughly analogous to the following C++ code:

    :::cpp
    vector<int> v;
    int x = 0;
    generate_n(back_inserter(v), 10, [&amp;i] () { return (x++) * 3; });

We can of course ignore closure's argument and do something else, like generate a vector of random elements:

    :::rust
    use std::rand;

    let v = Vec::from_fn(10, |_| rand::random::<uint>());

Matrices
--------

In the [nalgebra crate](https://siciarz.net/24-days-of-rust-nalgebra/) there is a `DMat` type representing a matrix which dimensions are known at runtime. We can build a matrix using the `from_fn` constructor too. Let's create a triangular matrix:

    :::rust
    extern crate nalgebra;

    use nalgebra::DMat;

    let mat: DMat<uint> = DMat::from_fn(7, 7, |i, j| if j <= i { 1 } else { 0 });
    println!("{}", mat);

The first two arguments to `from_fn` are numbers of rows and columns; this means the closure must take two arguments - indices of the current row and column. And here's our matrix:

    :::sh
    $ cargo run
    1 0 0 0 0 0 0
    1 1 0 0 0 0 0
    1 1 1 0 0 0 0
    1 1 1 1 0 0 0
    1 1 1 1 1 0 0
    1 1 1 1 1 1 0
    1 1 1 1 1 1 1

Images
------

Bur we're not limited to mathematical objects. For example the [image crate](https://siciarz.net/24-days-of-rust-image/) provides a buffer that can generate the image with the `from_fn` call.

    :::rust
    extern crate image;

    use image::Pixel;

    let buffer = image::ImageBuffer::from_fn(512, 512, |x, y| {
        Pixel::from_channels((x * y % 256) as u8, (y % 256) as u8, (x % 256) as u8, 255)
    });
    let img = image::DynamicImage::ImageRgba8(buffer);
    let out = File::create(&amp;Path::new("out_pattern.png")).unwrap();
    let _ = img.save(out, image::PNG);

We're working in two dimensions in the same manner as with the `DMat` type. And here's the generated image:

![pattern](//i.imgur.com/G3JuGR0.png)

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly.
</small>
