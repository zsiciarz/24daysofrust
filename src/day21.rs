#![allow(unused_imports, unused_variables, unused_mut)]

extern crate crypto;
extern crate "rustc-serialize" as serialize;

use crypto::aes::{mod, KeySize};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::symmetriccipher::SynchronousStreamCipher;

use serialize::base64::{STANDARD, ToBase64};

use std::rand::{OsRng, Rng};

fn main() {
    println!("24 days of Rust - rust-crypto (day 21)");
    let input = "Hello world!";
    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
    let mut bytes = Vec::from_elem(sha.output_bytes(), 0u8);
    sha.result(bytes.as_mut_slice());
    println!("{}", bytes.to_base64(STANDARD));

    let mut gen = OsRng::new().ok().expect("Failed to get OS random generator");
    let mut key = Vec::from_elem(16, 0u8);
    gen.fill_bytes(key.as_mut_slice());
    let mut nonce = Vec::from_elem(16, 0u8);
    gen.fill_bytes(nonce.as_mut_slice());
    println!("Key: {}", key.to_base64(STANDARD));
    println!("Nonce: {}", nonce.to_base64(STANDARD));
    let mut cipher = aes::ctr(KeySize::KeySize128, key.as_slice(), nonce.as_slice());
    let secret = "I like Nickelback";
    let mut output = Vec::from_elem(secret.len(), 0u8);
    //cipher.process(secret.as_bytes(), output.as_mut_slice());
    //println!("{}", output.to_base64(STANDARD));
}
