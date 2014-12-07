#![feature(phase)]

#[phase(plugin, link)]
extern crate itertools;

use itertools::Itertools;

fn main() {
    println!("24 days of Rust - itertools (day 7)");
    let mut words = "hello supercalifragilisticexpialidocious programmer".words();
    words.apply(|word| println!("{} is {} characters long", word, word.len()));
    let even = range(1, 10u).map(|x| x * 2);
    let odd = range(1, 5u).map(|x| x * 2 + 1);
    println!("{}", even.interleave(odd).collect::<Vec<_>>());
    let it = range(1, 10u);
    println!("{}", it.intersperse(15u).collect::<Vec<_>>());
}
