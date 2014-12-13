extern crate uuid;

use uuid::Uuid;

fn main() {
    println!("24 days of Rust - uuid (day 13)");
    for _ in range(0, 10u) {
        println!("{}", Uuid::new_v4().to_hyphenated_string());
    }
}
