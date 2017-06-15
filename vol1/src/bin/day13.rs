extern crate uuid;

use uuid::Uuid;

fn main() {
    println!("24 days of Rust - uuid (day 13)");
    for _ in 0..10 {
        println!("{}", Uuid::new_v4().hyphenated().to_string());
    }
    println!(
        "{:?}",
        Uuid::parse_str("d27cdb6e-ae6d-11cf-96b8-44455354000")
    );
    println!(
        "{:?}",
        Uuid::parse_str("x27cdb6e-ae6d-11cf-96b8-444553540000")
    );
    println!(
        "{:?}",
        Uuid::parse_str("d27cdb6-eae6d-11cf-96b8-444553540000")
    );
    println!(
        "{:?}",
        Uuid::parse_str("d27cdb6e-ae6d-11cf-96b8-444553540000")
    );
}
