extern crate serialize;

use serialize::{Decodable, Encodable, json};
use serialize::json::PrettyEncoder;

#[deriving(Decodable, Encodable)]
struct Photo {
    url: String,
    dimensions: (uint, uint),
}

#[deriving(Decodable, Encodable)]
struct User {
    name: String,
    post_count: uint,
    likes_burgers: bool,
    avatar: Option<Photo>,
}


fn main() {
    println!("24 days of Rust - json (day 6)");
    println!("{}", json::encode(&42i));
    println!("{}", json::encode(&vec!["to", "be", "or", "not", "to", "be"]));
    let user = User {
        name: "Zbyszek".to_string(),
        post_count: 100u,
        likes_burgers: true,
        avatar: Some(Photo {
            url: "http://lorempixel.com/160/160/".to_string(),
            dimensions: (160u, 160u),
        }),
    };
    println!("{}", json::encode(&user));
    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut encoder = PrettyEncoder::new(&mut buffer);
        user.encode(&mut encoder).ok().expect("JSON encode error");
    }
    let encoded = String::from_utf8(buffer).unwrap();
    println!("{}", encoded);
}
