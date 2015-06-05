extern crate image;
extern crate rand;

use image::{FilterType, GenericImage, Pixel};

use rand::distributions::{Normal, IndependentSample};
use std::fs::File;

fn main() {
    println!("24 days of Rust - image (day 12)");
    let img = image::open("data/in.png").ok().expect("Opening image failed");
    let filtered = img.fliph();
    let mut out = File::create("out.png").unwrap();
    let _ = filtered.save(&mut out, image::PNG).ok().expect("Saving image failed");

    let kernel = [-1.0f32, -1.0, -1.0,
                  -1.0, 8.0, -1.0,
                  -1.0, -1.0, -1.0];
    let filtered = img.filter3x3(&kernel);
    let mut out = File::create("out_blur.png").unwrap();
    let _ = filtered.save(&mut out, image::PNG).ok().expect("Saving image failed");

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
    let mut out = File::create("out_noisy.png").unwrap();
    let _ = noisy.save(&mut out, image::PNG).ok().expect("Saving image failed");

    let thumbnail = img.resize(120, 120, FilterType::Lanczos3);
    let mut out = File::create("out_thumb.png").unwrap();
    let _ = thumbnail.save(&mut out, image::PNG).ok().expect("Saving image failed");
}
