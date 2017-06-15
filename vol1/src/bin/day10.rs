extern crate num;
extern crate tau;

use num::complex::{Complex, Complex64};

fn main() {
    println!("24 days of Rust - tau (day 10)");
    println!("τ = {}", tau::TAU);
    let radius: f64 = 15.0;
    println!("circle circumference = τ * r = {}", tau::TAU * radius);
    let c: Complex64 = Complex::from_polar(&1.0, &tau::TAU);
    println!("Euler's identity: exp(i * τ) = {}", c);
    println!(
        "Trigonometry: sin(τ) = {}, cos(τ) = {}",
        tau::TAU.sin(),
        tau::TAU.cos()
    );
    println!("That other constant = {}", tau::TAU / 2.0);
}
