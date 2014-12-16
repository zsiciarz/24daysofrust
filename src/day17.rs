extern crate image;

use image::Pixel;

use std::io::File;

fn main() {
    println!("24 days of Rust - patterns (day 17)");
    let buffer = image::ImageBuffer::from_fn(512, 512, |x, y| {
        Pixel::from_channels((x * y % 256) as u8, (y % 256) as u8, (x % 256) as u8, 255)
    });
    let img = image::DynamicImage::ImageRgba8(buffer);
    let out = File::create(&Path::new("out_pattern.png")).unwrap();
    let _ = img.save(out, image::PNG);
}
