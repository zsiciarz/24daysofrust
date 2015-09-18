# Day 6 - working with JSON

> Relevancy: 1.1 stable (macros only on nightly)

[JSON](http://en.wikipedia.org/wiki/JSON) is a workhorse data format of the modern Web. Originating from the JavaScript world, it gained a lot of traction and at the moment it's usually the first choice of a Web developer for a data interchange format. Not only Web - once JavaScript-only, JSON support is now ubiquitous. A lot of languages ship with JSON parsers in the standard libraries, and when it's not the case, surely someone has already built a third party library. In case of Rust, JSON support comes out in the [rustc_serialize::json](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html) module.

**Note**: in this article I deliberately do not focus on the Web, APIs, requests and so on. I mentioned JSON in a [previous post about hyper](http://siciarz.net/24-days-of-rust-hyper/) but now I don't care where did the JSON-encoded data come from or what to do with it later. Here I'm just going to show you a few practical tips.

Slightly offtopic: I'm writing this blogpost while getting ready to coach a group of fantastic women at [Django Girls](http://djangogirls.org/) in Łódź, Poland. If you're a woman interested in learning programming (or know such girls) check out if there's a Django Girls workshop near you! The workshops focus on basic Python, Django and web technologies, but the general idea is to get the attendees genuinely interested in programming and empowered to create.

That said, now back to JSON and Rust.

Primitives
----------

A lot of Rust types serialize to JSON just as you would expect. Note that `encode()` immutably borrows its argument:

```rust
extern crate rustc_serialize;

use rustc_serialize::json;

fn main() {
    println!("{:?}", json::encode(&42));
    println!("{:?}", json::encode(&vec!["to", "be", "or", "not", "to", "be"]));
    println!("{:?}", json::encode(&Some(true)));
}
```

`Option<T>` maps to the encoding of `value` itself if it is a `Some(value)` while `None` maps to `null`.

Automatic (de)serialization
---------------------------

In the [chapter on CSV](day3.md) I mentioned the `RustcEncodable` and `RustcDecodable` traits. Here's an example with a nested struct:

```rust
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

let user = User {
    name: "Zbyszek".to_owned(),
    post_count: 100u32,
    likes_burgers: true,
    avatar: Some(Photo {
        url: "http://lorempixel.com/160/160/".to_owned(),
        dimensions: (160u32, 160u32),
    }),
};
println!("{:?}", json::encode(&user));
// Ok("{\"name\":\"Zbyszek\",\"post_count\":100,\"likes_burgers\":true,\"avatar\":{\"url\":\"http://lorempixel.com/160/160/\",\"dimensions\":[160,160]}}")
```

Pretty printing
---------------

The `json::encode()` doesn't care for readability of it's output. Although the JSON it emits is correct and machine-readable, there are no newlines or indents making it hard for a human to debug. Pretty-printing is a little bit more complex than just one function call, but not too complicated:

```rust
use rustc_serialize::json::{self, Encoder};
let mut encoded = String::new();
{
    let mut encoder = Encoder::new_pretty(&mut encoded);
    user.encode(&mut encoder).ok().expect("JSON encode error");
}
println!("{}", encoded);
```

Decoding
--------

```rust
let incoming_request = "{\"name\":\"John\",\"post_count\":2,\"likes_burgers\":false,\"avatar\":null}";
let decoded: User = json::decode(incoming_request).unwrap();
println!("My name is {} and I {} burgers",
    decoded.name, if decoded.likes_burgers { "love" } else { "don't like" });
assert!(decoded.avatar.is_none());
```

As you cen see, decoding is also pretty easy. But what happens if we don't know all of the fields in advance? We can use another function in the `json` module - `from_str()`. The difference between `from_str()` and `decode()` is that the latter may return some struct implementing `RustcDecodable` while the former returns a [Json](http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/enum.Json.html) value. This type has a few methods of its own, including `find()`. See the example below:

```rust
let new_request = "{\"id\":64,\"title\":\"24days\",\"stats\":{\"pageviews\":1500}}";
if let Ok(request_json) = json::Json::from_str(new_request) {
    if let Some(ref stats) = request_json.find("stats") {
        if let Some(ref pageviews) = stats.find("pageviews") {
            println!("Pageviews: {}", pageviews);
        }
    }
}
```

We're using the [if let](http://doc.rust-lang.org/book/if-let.html) language construct which often simplifies pattern matches where we care for only one branch and do nothing if the expression doesn't match.

The json! macro
---------------

> **Note:** syntax extensions work only on nightly

There's one more thing I wanted to show you today. You can embed JSON-like literals directly in your Rust code with the help of the [json_macros](https://crates.io/crates/json_macros) crate. This is a compiler extension that allows for some nice syntactic sugar like below:

```rust
#![feature(plugin)]
#![plugin(json_macros)]

let config = json!({
    "hostname": "localhost",
    "port": 6543,
    "allowed_methods": ["get", "post"],
    "limits": {
        "bandwidth": (500 * 16),
        "rate": null
    }
});
println!("Configuration: {}", config);
println!("Bandwidth: {}", config.search("bandwidth").unwrap());
```
