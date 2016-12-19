#[macro_use]
extern crate nom;

use std::str;

#[derive(Debug)]
enum Method {
    GET,
    POST,
}

#[derive(Debug)]
struct Request {
    method: Method,
    url: String,
    version: String,
}

named!(parse_method_v1, alt!(tag!("GET") | tag!("POST")));

named!(parse_method_v2<&[u8], &str>,
       alt!(map_res!(tag!("GET"), str::from_utf8) | map_res!(tag!("POST"), str::from_utf8)));

named!(parse_method<&[u8], Method>,
       alt!(map!(tag!("GET"), |_| Method::GET) | map!(tag!("POST"), |_| Method::POST)));

named!(parse_request<&[u8], Request>, ws!(do_parse!(
    method: parse_method >>
    url: map_res!(take_until!(" "), str::from_utf8) >>
    tag!("HTTP/") >>
    version: map_res!(take_until!("\r"), str::from_utf8) >>
    (Request { method: method, url: url.into(), version: version.into() })
)));

fn main() {
    println!("24 Days of Rust vol. 2 - nom, part 1");
    let first_line = b"GET /home/ HTTP/1.1\r\n";
    println!("{:?}", parse_method_v1(&first_line[..]));
    println!("{:?}", parse_method_v2(&first_line[..]));
    println!("{:?}", parse_method(&first_line[..]));
    println!("{:?}", parse_request(&first_line[..]));
    let bad_line = b"GT /home/ HTTP/1.1\r\n";
    println!("{:?}", parse_request(&bad_line[..]));
}
