#![feature(core, str_words)]

#[macro_use]
extern crate itertools;

use std::iter::AdditiveIterator;
use itertools::Itertools;

fn main() {
    println!("24 days of Rust - itertools (day 7)");
    let mut words = "hello supercalifragilisticexpialidocious programmer".words();
    words.apply(|word| println!("{} is {} characters long", word, word.len()));
    let even = (1us..10).map(|x| x * 2);
    let odd = (1us..5).map(|x| x * 2 + 1);
    println!("{:?}", even.interleave(odd).collect::<Vec<_>>());
    println!("{:?}", (1..10).intersperse(15).collect::<Vec<_>>());
    let numbers = 1..4;
    let chars = vec!['a', 'b', 'c'];
    for (i, c) in iproduct!(numbers, chars.iter()) {
        println!("{}: {}", i, c);
    }
    /*
    let log = "GET / 4096\nGET /home/ 16301\nPOST /home/ 49\nGET / 4096\n";
    let lines = log.lines();
    let rows = icompr!(line.words().collect::<Vec<_>>() for line in lines);
    let bytes = icompr!(row[2] for row in rows if row[0] != "POST");
    let total = icompr!(b.parse::<uint>().unwrap() for b in bytes).sum();
    println!("Total GET throughput: {} bytes", total);
    */
}
