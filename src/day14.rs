extern crate nalgebra;

use std::f64::consts::FRAC_PI_2;
use nalgebra::{Mat2, Pnt2, Rot2, Vec1, Vec2, Vec3};

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
}
