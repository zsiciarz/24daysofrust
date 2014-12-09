extern crate tau;

use std::num::FloatMath;

fn main() {
    println!("24 days of Rust - tau (day 9)");
    println!("τ = {}", tau::TAU);
    println!("cos(τ) = {}", tau::TAU.cos());
}
