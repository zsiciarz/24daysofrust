# Day 14 - nalgebra

> Relevancy: **outdated**

The [nalgebra](https://crates.io/crates/nalgebra) crate provides a wide set of mathematical primitives for linear algebra, computer physics, graphic engines etc. I'm not going to dive deep into the underlying math, there are a lot of tutorials and courses, some of them specifically targeted at the programmers. My goal for today is just a brief showcase of what we can do in Rust with `nalgebra`.

Basic vector and matrix operations
----------------------------------

    :::rust
    extern crate nalgebra;

    use nalgebra::{Mat2, Rot2, Vec2};

    fn main() {
        println!("24 days of Rust - nalgebra (day 14)");
        let v = Vec2::new(0.0f64, 1.0);
        // 90 degrees clockwise
        //  0, 1
        // -1, 0
        let rot = Mat2::new(0.0f64, -1.0, 1.0, 0.0);
        println!("{}", rot * v);
    }

<!-- -->

    :::sh
    $ cargo run
    Vec2 { x: 1, y: 0 }

In `nalgebra` there are several statically sized vector and square matrix types (for dimensions up to 6). The standard mathematical operators are overloaded, so all allowed kinds of vector/matrix multiplication should just work. In the example above we defined the [rotation matrix](http://en.wikipedia.org/wiki/Rotation_matrix) ourselves, but there is a nice shortcut: the `RotN` type.

    :::rust
    use std::f64::consts::FRAC_PI_2;
    use nalgebra::{Rot2, Vec1};

    let angle = FRAC_PI_2;
    let rot = Rot2::new(Vec1::new(angle));
    println!("{}", rot * v);

The output is the same but this time we tell Rust *what* to do, not *how* to do it. Note that we need to wrap the `angle` in a single-element vector.

We can use vectors to translate (move) points.

    :::rust
    let point = Pnt2::new(4.0f64, 4.0);
    println!("Translate from {} to {}", point, nalgebra::translate(&amp;v, &amp;point));

A number of other operations are also exposed as top-level functions, such as `transform()`, `rotate()` along with their inverse counterparts.

Dot and cross product
---------------------

    :::rust
    use nalgebra::Vec3;

    let v1 = Vec3::new(2.0f64, 2.0, 0.0);
    let v2 = Vec3::new(2.0f64, -2.0, 0.0);
    if nalgebra::approx_eq(&amp;0.0f64, &amp;nalgebra::dot(&amp;v1, &amp;v2)) {
        println!("v1 is orthogonal to v2");
    }

    println!("{}", nalgebra::cross(&amp;v1, &amp;v2));
    println!("{}", nalgebra::cross(&amp;v2, &amp;v1));

The output is:

    :::sh
    $ cargo run
    v1 is orthogonal to v2
    Vec3 { x: 0, y: 0, z: -8 }
    Vec3 { x: 0, y: 0, z: 8 }

[Dot product](http://en.wikipedia.org/wiki/Dot_product) can be used to check if two vectors are orthogonal to each other. That happens if their dot product is equal to 0. As floating point comparisons are [sometimes suprising](http://www.parashift.com/c++-faq/floating-point-arith.html), we should use the `approx_eq()` function.

The [cross product](http://en.wikipedia.org/wiki/Cross_product) of two vectors is always perpendicular (sometimes we say **normal**) to *both* of them. As you can see from the example it is also not commutative. Normal vectors are very important in computer graphics for [calculating light and shading](http://www.opengl-tutorial.org/beginners-tutorials/tutorial-8-basic-shading/) of the scene.

Dynamic vectors
---------------

All of the `nalgebra` types we've seen so far have their higher-dimensional variants up to `Vec6`/`Mat6` etc. But what if we want to go further? Very high number of dimensions is common for example in digital signal processing. In `nalgebra` there is a `DVec` type for that purpose.

    :::rust
    const SIZE: uint = 512;
    let sine = DVec::from_fn(SIZE, |i: uint| {
        let t = i as f64 / 16.0f64;
        t.sin()
    });
    draw(&amp;sine, &amp;Path::new("out_sine.png"));

    let window = DVec::from_fn(SIZE, |i: uint| {
        0.54f64 - 0.46 * (PI_2 * (i as f64) / (SIZE - 1) as f64).cos()
    });
    draw(&amp;window, &amp;Path::new("out_window.png"));

    draw(&amp;(sine * window), &amp;Path::new("out_windowed.png"));

We can use the `from_fn()` mwthod to create a vector by generating each element in a closure. The `window` variable is a [Hamming window](http://en.wikipedia.org/wiki/Window_function#Hamming_window); such window functions are a common preprocessing step in DSP.

The `draw()` function borrows a `DVec` and a `Path` and plots a simple representation of the vector to a PNG file using the [image crate](https://siciarz.net/24-days-of-rust-image/). The code is [on GitHub](https://github.com/zsiciarz/24daysofrust/blob/86aef8e34a1b7681a189ffbf0abc50a1f413da23/src/day14.rs#L10) for anybody interested. Here's the output of all three steps of the above code:

![sine, window, windowed sine](//i.imgur.com/mQKFms3.png)

See also
--------

* [A First Course in Linear Algebra](http://linear.ups.edu/html/fcla.html)
* [Linear algebra for game developers](http://blog.wolfire.com/2009/07/linear-algebra-for-game-developers-part-1/)
* [kiss3d](https://github.com/sebcrozet/kiss3d) - a simple 3D graphics engine for Rust

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly and nalgebra 0.1.0.
</small>
