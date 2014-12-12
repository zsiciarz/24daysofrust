extern crate image;

use image::{GenericImage, Pixel};

use std::io::File;
use std::rand;
use std::rand::distributions::{Normal, IndependentSample};

fn main() {
    println!("24 days of Rust - image (day 12)");
    let img = image::open(&Path::new("data/in.jpg")).ok().expect("Opening image failed");
    let filtered = img.fliph();
    let out = File::create(&Path::new("out.jpg")).unwrap();
    let _ = filtered.save(out, image::JPEG).ok().expect("Saving image failed");

    let kernel = [-1.0f32, -1.0, -1.0,
                  -1.0, 8.0, -1.0,
                  -1.0, -1.0, -1.0];
    let filtered = img.filter3x3(&kernel);
    let out = File::create(&Path::new("out.jpg")).unwrap();
    let _ = filtered.save(out, image::JPEG).ok().expect("Saving image failed");

    let (width, height) = img.dimensions();
    let mut rng = rand::task_rng();
    let normal = Normal::new(15.0, 15.0);
    let mut noisy = img.brighten(-25);
    for x in range(0, width) {
        for y in range(0, height) {
            let offset = normal.ind_sample(&mut rng) as u8;
            let px = img.get_pixel(x, y).map(|v| v + offset);
            noisy.put_pixel(x, y, px);
        }
    }
    let out = File::create(&Path::new("out.jpg")).unwrap();
    let _ = noisy.save(out, image::JPEG).ok().expect("Saving image failed");
}
