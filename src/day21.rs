extern crate crypto;
extern crate "rustc-serialize" as serialize;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use serialize::base64::{STANDARD, ToBase64};

fn main() {
    println!("24 days of Rust - rust-crypto (day 21)");
    let input = "Hello world!";
    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
    let mut bytes = Vec::from_elem(sha.output_bytes(), 0u8);
    sha.result(bytes.as_mut_slice());
    println!("{}", bytes.to_base64(STANDARD));
}
