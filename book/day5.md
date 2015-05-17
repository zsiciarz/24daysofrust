# Day 5 - hyper

The state of HTTP libraries in Rust was a constant flux before 1.0. However it appears that a specific package won the hearts of Rust programmers: [hyper](https://crates.io/crates/hyper), which will be the subject of this chapter.

I'm going to focus on using `hyper` only as a client, although the library contains also a server implementation. However with the advance of Rust web frameworks building on top of HTTP libraries, the programmers will focus less on developing servers and more on the clients. Consuming web APIs is a lot more common than writing new shiny servers. How can `hyper` help us?

Basic requests
--------------

Let's start from the usual dependency definition in `Cargo.toml`.

```ini
[dependencies]
hyper = "~0.5"
```

When you run `cargo build`, Cargo will download a few other required crates (for URL handling, mimetype support, OpenSSL bindings etc.) and hopefully compile `hyper` afterwards. Time for our first request!

```rust
extern crate hyper;

use std::io::Read;
use hyper::{Client};

fn main() {
    let mut client = Client::new();
    let url = "http://httpbin.org/status/201";
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    println!("buf: {}", buf);
}
```

That was... *verbose*. I could just use `unwrap()` everywhere, but that would be handwaving and in poor taste. Sprinkling your code with `panic!` is not a sign of good style too. However, there are so many things that can go wrong during an HTTP request/response cycle! But there seems to be a pattern. Can we do better?

```rust
fn get_content(url: &str) -> hyper::error::Result<String> {
    let mut client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

fn main() {
    println!("{}", get_content("http://httpbin.org/status/200"));
}
```

We refactored the request cycle into a separate function. But look how the code got simpler, thanks to the [try! macro](http://doc.rust-lang.org/std/result/#the-try!-macro). There's no explicit matching on the `Result` variants and the first `try!` that fails will return from the function with some kind of an HTTP error.

POST and query parameters
-------------------------

Sending POST requests with hyper is only a little bit more complicated. We'll write a wrapper function again, this time taking an additional argument of type `Query`.

```rust
extern crate url;

use url::form_urlencoded;

type Query<'a> = Vec<(&'a str, &'a str)>;

fn post_query(url: &str, query: Query) -> hyper::error::Result<String> {
    let mut client = Client::new();
    let body = form_urlencoded::serialize(query);
    let mut response = try!(client.post(url).body(&body[..]).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

let query = vec![("key", "value"), ("foo", "bar")];
println!("{}", post_query("http://httpbin.org/post", query).unwrap());
```

The main difference from `get_content()` is the serialization machinery coming from the [url](https://crates.io/crates/url) crate. Once we've built a raw request body (like `key=value&foo=bar`), we pass it to the `body()` method and the rest is identical to the GET example above.

Sending JSON
------------

Our `post_query` function can be easily changed to borrow a struct, serialize it to JSON and send it over the wire.

```rust
extern crate rustc_serialize;

use rustc_serialize::{Encodable, json};

fn post_json<'a, T>(url: &str, payload: &T) -> hyper::error::Result<String>
    where T: Encodable {
    let body = json::encode(payload).unwrap();
    // rest of the code as before
}
```

This function is generic in its `payload` argument, accepting anything that implements the `Encodable` trait. We can use the function as follows:

```rust
#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: usize,
}

let movie = Movie {
    title: "You Only Live Twice".to_string(),
    bad_guy: "Blofeld".to_string(),
    pub_year: 1967,
};
println!("{:?}", post_json("http://httpbin.org/post", &movie).unwrap());
```

See also
--------

* [HTTP library requirements](https://github.com/servo/servo/wiki/HTTP-library-requirements) from the Servo project
* [rust-request](https://github.com/jgillich/rust-request) - a HTTP client library written on top of hyper
* [rest_client](https://github.com/gtolle/rest_client) - another HTTP client built with hyper
* [Improved error handling in Rust](http://lucumr.pocoo.org/2014/11/6/error-handling-in-rust/) by Armin Ronacher
