extern crate anymap;

use anymap::AnyMap;

fn main() {
    println!("24 days of Rust - anymap (day 9)");
    let mut config = AnyMap::new();
    config.insert("localhost");
    println!("{}", config.get::<&str>());
    println!("{}", config.get::<uint>());
}
