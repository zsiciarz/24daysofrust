extern crate image;
extern crate nalgebra;

use image::Pixel;
use nalgebra::DMat;

use std::io::File;
use std::rand;

fn main() {
    println!("24 days of Rust - patterns (day 17)");
    let v = range(0, 10u).map(|x| x * 3).collect::<Vec<_>>();
    println!("{}", v);
    let v = Vec::from_fn(10, |x| x * 3);
    println!("{}", v);
    let v = Vec::from_fn(10, |_| rand::random::<uint>());
    println!("{}", v);
    let mat = DMat::from_fn(7, 7, |i, j| if j <= i { 1 } else { 0 });
    println!("{}", mat);
    let buffer = image::ImageBuffer::from_fn(512, 512, |x, y| {
        Pixel::from_channels((x * y % 256) as u8, (y % 256) as u8, (x % 256) as u8, 255)
    });
    let img = image::DynamicImage::ImageRgba8(buffer);
    let out = File::create(&Path::new("out_pattern.png")).unwrap();
    let _ = img.save(out, image::PNG);
}
