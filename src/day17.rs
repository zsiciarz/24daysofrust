extern crate image;
extern crate nalgebra;
extern crate rand;

use image::Pixel;
use nalgebra::DMatrix;

use std::fs::File;
use std::path::Path;

fn main() {
    println!("24 days of Rust - patterns (day 17)");
    let v = (0..10).map(|x| x * 3).collect::<Vec<_>>();
    println!("{:?}", v);
    let v = (0..10).map(|_| rand::random::<u32>()).collect::<Vec<_>>();
    println!("{:?}", v);
    let mat: DMatrix<u32> = DMatrix::from_fn(7, 7, |i, j| if j <= i { 1 } else { 0 });
    println!("{:?}", mat);
    let buffer = image::ImageBuffer::from_fn(512u32, 512u32, |x: u32, y: u32| {
        Pixel::from_channels((x * y % 256) as u8, (y % 256) as u8, (x % 256) as u8, 255)
    });
    let img = image::DynamicImage::ImageRgba8(buffer);
    let mut out = File::create(&Path::new("out_pattern.png")).unwrap();
    let _ = img.save(&mut out, image::PNG);
}
