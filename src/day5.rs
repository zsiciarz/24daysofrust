extern crate hyper;

use hyper::{HttpError, HttpResult, Url};
use hyper::client::Request;
use hyper::header::ContentLength;

fn get_content(url: &str) -> HttpResult<String> {
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Err(HttpError::HttpUriError),
    };
    let fresh_request = try!(Request::get(url));
    let streaming_request = try!(fresh_request.start());
    let mut response = try!(streaming_request.send());
    Ok(try!(response.read_to_string()))
}

fn post_query(url: &str, body: &str) -> HttpResult<String> {
    let url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Err(HttpError::HttpUriError),
    };
    let mut fresh_request = try!(Request::post(url));
    fresh_request.headers_mut().set(ContentLength(body.len()));
    let mut streaming_request = try!(fresh_request.start());
    try!(streaming_request.write_str(body));
    let mut response = try!(streaming_request.send());
    Ok(try!(response.read_to_string()))
}

fn main() {
    println!("24 days of Rust - hyper (day 5)");
    println!("{}", get_content("http://httpbin.org/status/200"));
    println!("{}", post_query("http://httpbin.org/post", "key=value"));
}
