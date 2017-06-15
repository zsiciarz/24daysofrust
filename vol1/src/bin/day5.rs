extern crate hyper;
extern crate rustc_serialize;
extern crate url;

use std::io::Read;
use hyper::Client;
use rustc_serialize::{Encodable, json};
use url::form_urlencoded;

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;
    Ok(buf)
}

type Query<'a> = Vec<(&'a str, &'a str)>;

fn post_query(url: &str, query: Query) -> hyper::Result<String> {
    let client = Client::new();
    let body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(query.iter())
        .finish();
    let mut response = client.post(url).body(&body[..]).send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;
    Ok(buf)
}

fn post_json<T>(url: &str, payload: &T) -> hyper::Result<String>
where
    T: Encodable,
{
    let client = Client::new();
    let body = json::encode(payload).unwrap();
    let mut response = client.post(url).body(&body[..]).send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;
    Ok(buf)
}

#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: usize,
}

fn main() {
    println!("24 days of Rust - hyper (day 5)");
    println!("{:?}", get_content("http://httpbin.org/status/200"));
    let query = vec![("key", "value"), ("foo", "bar")];
    println!("{}", post_query("http://httpbin.org/post", query).unwrap());
    let movie = Movie {
        title: "You Only Live Twice".to_string(),
        bad_guy: "Blofeld".to_string(),
        pub_year: 1967,
    };
    println!("{}", post_json("http://httpbin.org/post", &movie).unwrap());
}
