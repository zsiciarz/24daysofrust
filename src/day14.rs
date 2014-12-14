extern crate nalgebra;

use std::f64::consts::FRAC_PI_2;
use nalgebra::{Mat2, Rot2, Vec1, Vec2};

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
}
