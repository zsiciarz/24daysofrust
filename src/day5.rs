extern crate hyper;

use hyper::Url;
use hyper::client::Request;

fn main() {
    println!("24 days of Rust - hyper (day 5)");
    let url = match Url::parse("http://httpbin.org/status/200") {
        Ok(url) => url,
        Err(_) => panic!("Uh oh."),
    };
    println!("> get: {}", url);
    let fresh_request = match Request::get(url) {
        Ok(request) => request,
        Err(_) => panic!("Whoops."),
    };
    let streaming_request = match fresh_request.start() {
        Ok(request) => request,
        Err(_) => panic!("Noooo."),
    };
    let mut response = match streaming_request.send() {
        Ok(response) => response,
        Err(_) => panic!("So close..."),
    };
    println!("< status code: {}", response.status);
    let content = match response.read_to_string() {
        Ok(content) => content,
        Err(_) => panic!("I give up."),
    };
    println!("{}", content);
}
