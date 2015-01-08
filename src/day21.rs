#![allow(unused_imports, unused_variables, unused_mut)]

extern crate crypto;
extern crate "rustc-serialize" as serialize;

use crypto::aes::{self, KeySize};
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use crypto::symmetriccipher::SynchronousStreamCipher;

use serialize::base64::{STANDARD, ToBase64};
use serialize::hex::ToHex;

use std::iter::repeat;
use std::rand::{OsRng, Rng};

fn main() {
    println!("24 days of Rust - rust-crypto (day 21)");
    let input = "Hello world!";
    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
    let mut bytes: Vec<u8> = repeat(0u8).take(sha.output_bytes()).collect();
    sha.result(bytes.as_mut_slice());
    println!("{}", bytes.to_base64(STANDARD));

    let mut gen = OsRng::new().ok().expect("Failed to get OS random generator");
    let mut key: Vec<u8> = repeat(0u8).take(16).collect();
    gen.fill_bytes(key.as_mut_slice());
    let mut nonce: Vec<u8> = repeat(0u8).take(16).collect();
    gen.fill_bytes(nonce.as_mut_slice());
    println!("Key: {}", key.to_base64(STANDARD));
    println!("Nonce: {}", nonce.to_base64(STANDARD));
    let mut cipher = aes::ctr(KeySize::KeySize128, key.as_slice(), nonce.as_slice());
    let secret = "I like Nickelback";
    let mut output: Vec<u8> = repeat(0u8).take(secret.len()).collect();
    cipher.process(secret.as_bytes(), output.as_mut_slice());
    println!("Ciphertext: {}", output.to_base64(STANDARD));

    let mut hmac_key: Vec<u8> = repeat(0u8).take(32).collect();
    gen.fill_bytes(hmac_key.as_mut_slice());
    let message = "Ceterum censeo Carthaginem esse delendam";
    println!("Message: {}", message);
    println!("HMAC key: {}", hmac_key.to_base64(STANDARD));
    let mut hmac = Hmac::new(Sha256::new(), hmac_key.as_slice());
    hmac.input(message.as_bytes());
    println!("HMAC digest: {}", hmac.result().code().to_hex());
}
