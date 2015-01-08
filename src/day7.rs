#[macro_use]
extern crate itertools;

use std::iter::AdditiveIterator;
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
    let numbers = range(1, 4u);
    let chars = vec!['a', 'b', 'c'];
    for (i, c) in iproduct!(numbers, chars.iter()) {
        println!("{}: {}", i, c);
    }
    let log = "GET / 4096\nGET /home/ 16301\nPOST /home/ 49\nGET / 4096\n";
    let lines = log.lines();
    let rows = icompr!(line.words().collect::<Vec<_>>() for line in lines);
    let bytes = icompr!(row[2] for row in rows if row[0] != "POST");
    let total = icompr!(b.parse::<uint>().unwrap() for b in bytes).sum();
    println!("Total GET throughput: {} bytes", total);
}
