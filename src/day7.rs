#![feature(phase)]

#[phase(plugin, link)]
extern crate itertools;

use itertools::Itertools;

fn main() {
    println!("24 days of Rust - itertools (day 7)");
    let mut words = "hello supercalifragilisticexpialidocious programmer".words();
    words.apply(|word| println!("{} is {} characters long", word, word.len()));
}
