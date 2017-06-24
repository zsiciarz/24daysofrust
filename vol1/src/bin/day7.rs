#[macro_use]
extern crate itertools;

use itertools::Itertools;

fn main() {
    println!("24 days of Rust - itertools (day 7)");
    let words = "hello supercalifragilisticexpialidocious programmer".split(|c| c == ' ');
    words.foreach(|word| {
        println!("{} is {} characters long", word, word.len())
    });
    let even = (1..10).map(|x| x * 2);
    let odd = (1..5).map(|x| x * 2 + 1);
    println!("{:?}", even.interleave(odd).collect::<Vec<_>>());
    println!("{:?}", (1..10).intersperse(15).collect::<Vec<_>>());
    let numbers = 1..4;
    let chars = vec!['a', 'b', 'c'];
    for (i, c) in iproduct!(numbers, chars.iter()) {
        println!("{}: {}", i, c);
    }
}
