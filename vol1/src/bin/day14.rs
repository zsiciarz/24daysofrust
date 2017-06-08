extern crate image;
extern crate nalgebra;

use std::f64::consts::{PI, FRAC_PI_2};
use std::fs::File;
use std::path::Path;
use image::{GenericImage, Pixel, Rgba};
use nalgebra::{DVector, Matrix2, Point2, Rotation2, Vector2, Vector3};

fn draw(v: &DVector<f64>, path: &Path) {
    let width = v.len() as u32;
    let height = 128u32;
    let white: Rgba<u8> = Pixel::from_channels(255, 255, 255, 255);
    let buffer = image::ImageBuffer::from_pixel(width, height, white);
    let mut img = image::DynamicImage::ImageRgba8(buffer);
    let red = Pixel::from_channels(255, 0, 0, 255);
    for i in 0u32..(width) {
        let half = (height / 2) as f64;
        let y = half * (1.0 + v[i as usize]);
        let y = nalgebra::clamp(y as u32, 1u32, height);
        img.put_pixel(i, height - y, red);
    }
    let mut out = File::create(path).unwrap();
    let _ = img.save(&mut out, image::PNG);
}

fn main() {
    println!("24 days of Rust - nalgebra (day 14)");
    let v = Vector2::new(0.0f64, 1.0);
    // 90 degrees clockwise
    //  0, 1
    // -1, 0
    let rot = Matrix2::new(0.0f64, -1.0, 1.0, 0.0);
    println!("{}", rot * v);
    let angle = FRAC_PI_2;
    let rot = Rotation2::new(angle);
    println!("{}", rot * v);
    let point = Point2::new(4.0f64, 4.0);
    println!("Translate from {} to {}", point, point + v);

    let v1 = Vector3::new(2.0f64, 2.0, 0.0);
    let v2 = Vector3::new(2.0f64, -2.0, 0.0);
    println!("Dot product: {}", v1.dot(&v2));

    println!("{}", v1.cross(&v2));
    println!("{}", v2.cross(&v1));

    const SIZE: usize = 512;
    let sine = DVector::from_fn(SIZE, |i: usize, _| {
        let t = i as f64 / 16.0f64;
        t.sin()
    });
    draw(&sine, Path::new("out_sine.png"));

    let window = DVector::from_fn(SIZE, |i: usize, _| {
        0.54f64 - 0.46 * (PI * 2.0 * (i as f64) / (SIZE - 1) as f64).cos()
    });
    draw(&window, Path::new("out_window.png"));

    draw(&sine.component_mul(&window), Path::new("out_windowed.png"));
}
