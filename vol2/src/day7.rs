#![feature(test)]
#![feature(plugin)]
#![plugin(phf_macros)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate phf;

extern crate test;

#[derive(Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

mod lookups;

use lookups::{find_color, find_color_lazy_static, find_color_phf};

fn main() {
    println!("24 Days of Rust vol. 2 - static");
    println!("{:?}", find_color("black"));
    println!("{:?}", find_color_lazy_static("fuchsia"));
    println!("{:?}", find_color_phf("ROSE"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_match_lookup(b: &mut Bencher) {
        b.iter(|| find_color("White"))
    }

    #[bench]
    fn bench_lazy_static_map(b: &mut Bencher) {
        b.iter(|| find_color_lazy_static("White"))
    }

    #[bench]
    fn bench_phf_map(b: &mut Bencher) {
        b.iter(|| find_color_phf("White"))
    }
}
