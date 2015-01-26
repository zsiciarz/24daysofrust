extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate url;

use hyper::{Client, HttpResult};
use rustc_serialize::{Encodable, json};
use url::form_urlencoded;

fn get_content(url: &str) -> HttpResult<String> {
    let mut client = Client::new();
    let mut response = try!(client.get(url).send());
    Ok(try!(response.read_to_string()))
}

type Query<'a> = Vec<(&'a str, &'a str)>;

fn post_query(url: &str, query: Query) -> HttpResult<String> {
    let mut client = Client::new();
    let body = form_urlencoded::serialize(query.into_iter());
    let mut response = try!(client.post(url).body(&body[]).send());
    Ok(try!(response.read_to_string()))
}

fn post_json<'a, T>(url: &str, payload: &T) -> HttpResult<String>
    where T: Encodable {
    let mut client = Client::new();
    let body = json::encode(payload).unwrap();
    let mut response = try!(client.post(url).body(&body[]).send());
    Ok(try!(response.read_to_string()))
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
