# Day 12 - image

> Relevancy: 1.1 stable

The [image](https://crates.io/crates/image) crate is a library under development (well, not unlike the rest of the Rust ecosystem before 1.0) to read, manipulate and write images. It is part of the effort to develop an open source game engine in pure Rust - [Piston](http://www.piston.rs/), but of course the `image` crate can be used on its own.

At the moment `image` supports reading and writing JPG, GIF and PNG images, while a few other formats are read-only (TIFF, WEBP).

Basics
------

Let's start from something simple - read a JPEG image, flip it horizontally and save as PNG.

```rust
extern crate image;

use std::fs::File;

fn main() {
    let img = image::open("data/in.png").ok().expect("Opening image failed");
    let filtered = img.fliph();
    let mut out = File::create("out.png").unwrap();
    let _ = filtered.save(&mut out, image::PNG).ok().expect("Saving image failed");
}
```

We used the `open()` function to create the `img` variable (which is a [DynamicImage](http://www.piston.rs/image/image/enum.DynamicImage.html) value). This is a wrapper for the `load()` function, which can read images from anything that implements the `Read` trait. However, there's no symmetrical shortcut to write an image, so we take advantage of the fact that `File` implements `Write`. In between, the `fliph()` method of the image does what it says on the cover. There are other transformations available, such as:

 * `fliph()`
 * `flipv()`
 * `rotateN()` where N is 90, 180 or 270
 * `blur(sigma)`
 * `invert()`
 * `grayscale()`

and a few others. All these return a new image, except `invert()`.

Edge detection
--------------

The `image` API lets us run arbitrary 3x3 [convolution filters](http://www.roborealm.com/help/Convolution.php). We can use it to create a very basic edge detection filter.

```rust
let kernel = [-1.0f32, -1.0, -1.0,
              -1.0, 8.0, -1.0,
              -1.0, -1.0, -1.0];
let filtered = img.filter3x3(&kernel);
```

To honour the image processing tradition, our input image is [a photo of Lena Söderberg](http://en.wikipedia.org/wiki/Lenna). Here's the result:

![edgy Lena](//i.imgur.com/D1mMwhK.jpg)

Directly manipulating pixels
----------------------------

A typical example of looping over image pixels is to add some noise to the image. The noise does not depend on surrounding pixels, so the inner loop is very simple - generate a Gaussian noise sample and add it to the current pixel.

```rust
extern crate rand;

use image::{GenericImage, Pixel};
use rand::distributions::{Normal, IndependentSample};

let (width, height) = img.dimensions();
let mut rng = rand::thread_rng();
let normal = Normal::new(15.0, 15.0);
let mut noisy = img.brighten(-25);
for x in 0..(width) {
    for y in 0..(height) {
        let offset = normal.ind_sample(&mut rng) as u8;
        let px = img.get_pixel(x, y).map(|v| if v <= 255 - offset { v + offset } else { 255 });
        noisy.put_pixel(x, y, px);
    }
}
```

We need to use the `GenericImage` and `Pixel` traits to introduce some extra methods we're going to use later.

Each run of the inner loop generates a sample from the normal distribution. We then pick one pixel from the original image with `get_pixel()`, add the offset to every channel (that's what the `map()` method does for types implementing `Pixel` trait) and store the pixel in the new image.

![noisy Lena](//i.imgur.com/Zu7jnIK.jpg)

Thumbnails
----------

To create a thumbnail from the image, use it's `resize()` method. It takes three arguments: width and height of the thumbnail (but the original aspect ratio will be preserved, so one of these dimensions might be ignored) and a variant of the `FilterType` enum. This value dictates what interpolation to use when resizing. See for example the [GIMP documentation](http://docs.gimp.org/en/gimp-tools-transform.html) to learn more about various methods. I personally like Lanczos interpolation, unless the result looks really bad.

```rust
let thumbnail = img.resize(120, 120, FilterType::Lanczos3);
```

One more thing - if your code using the `image` crate seems to be pretty slow, double check that you run in release mode (with compiler optimizations). For Cargo, that means `cargo run --release`. In my case the change from the default (no optimization) to release mode resulted in an 8-10x increase in speed.
