#![feature(plugin)]

#![plugin(clippy)]

extern crate image;
extern crate nalgebra;

use std::f64::consts::{PI, FRAC_PI_2};
use std::fs::File;
use std::path::Path;
use image::{GenericImage, Pixel, Rgba};
use nalgebra::{DVec, Mat2, Pnt2, Rot2, Vec1, Vec2, Vec3};

fn draw(v: &DVec<f64>, path: &Path) {
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
    let v = Vec2::new(0.0f64, 1.0);
    // 90 degrees clockwise
    //  0, 1
    // -1, 0
    let rot = Mat2::new(0.0f64, -1.0, 1.0, 0.0);
    println!("{:?}", rot * v);
    let angle = FRAC_PI_2;
    let rot = Rot2::new(Vec1::new(angle));
    println!("{:?}", rot * v);
    let point = Pnt2::new(4.0f64, 4.0);
    println!("Translate from {:?} to {:?}", point, nalgebra::translate(&v, &point));

    let v1 = Vec3::new(2.0f64, 2.0, 0.0);
    let v2 = Vec3::new(2.0f64, -2.0, 0.0);
    if nalgebra::approx_eq(&0.0f64, &nalgebra::dot(&v1, &v2)) {
        println!("v1 is orthogonal to v2");
    }

    println!("{:?}", nalgebra::cross(&v1, &v2));
    println!("{:?}", nalgebra::cross(&v2, &v1));

    const SIZE: usize = 512;
    let sine = DVec::from_fn(SIZE, |i: usize| {
        let t = i as f64 / 16.0f64;
        t.sin()
    });
    draw(&sine, &Path::new("out_sine.png"));

    let window = DVec::from_fn(SIZE, |i: usize| {
        0.54f64 - 0.46 * (PI * 2.0 * (i as f64) / (SIZE - 1) as f64).cos()
    });
    draw(&window, &Path::new("out_window.png"));

    draw(&(sine * window), &Path::new("out_windowed.png"));
}
