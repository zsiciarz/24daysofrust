extern crate image;
extern crate nalgebra;

use std::f64::consts::FRAC_PI_2;
use std::io::File;
use std::num::FloatMath;
use image::{GenericImage, Pixel};
use nalgebra::{DVec, Mat2, Pnt2, Rot2, Vec1, Vec2, Vec3};

fn draw(v: &DVec<f64>, path: &Path) {
    let width = v.len() as u32;
    let height = 128u32;
    let white = Pixel::from_channels(255, 255, 255, 255);
    let buffer = image::ImageBuffer::from_fn(width, height, |_, _| white);
    let mut img = image::DynamicImage::ImageRgba8(buffer);
    let red = Pixel::from_channels(255, 0, 0, 255);
    for i in range(0u32, width) {
        let half = (height / 2) as f64;
        let y = half * (1.0 + v[i as uint]);
        let y = nalgebra::clamp(y as u32, 1u32, height);
        img.put_pixel(i, height - y, red);
    }
    let out = File::create(path).unwrap();
    let _ = img.save(out, image::PNG);
}

fn main() {
    println!("24 days of Rust - nalgebra (day 14)");
    let v = Vec2::new(0.0f64, 1.0);
    // 90 degrees clockwise
    //  0, 1
    // -1, 0
    let rot = Mat2::new(0.0f64, -1.0, 1.0, 0.0);
    println!("{}", rot * v);
    let angle = FRAC_PI_2;
    let rot = Rot2::new(Vec1::new(angle));
    println!("{}", rot * v);
    let point = Pnt2::new(4.0f64, 4.0);
    println!("Translate from {} to {}", point, nalgebra::translate(&v, &point));

    let v1 = Vec3::new(2.0f64, 2.0, 0.0);
    let v2 = Vec3::new(2.0f64, -2.0, 0.0);
    if nalgebra::approx_eq(&0.0f64, &nalgebra::dot(&v1, &v2)) {
        println!("v1 is orthogonal to v2");
    }

    println!("{}", nalgebra::cross(&v1, &v2));
    println!("{}", nalgebra::cross(&v2, &v1));

    let sine = DVec::from_fn(512, |i: uint| {
        let t = i as f64 / 16.0f64;
        t.sin()
    });
    draw(&sine, &Path::new("out_sine.png"));
}
