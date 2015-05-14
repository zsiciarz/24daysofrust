# Day 6 - working with JSON

[JSON](http://en.wikipedia.org/wiki/JSON) is a workhorse data format of the modern Web. Originating from the JavaScript world, it gained a lot of traction and at the moment it's usually the first choice of a Web developer for a data interchange format. Not only Web - once JavaScript-only, JSON support is now ubiquitous. A lot of languages ship with JSON parsers in the standard libraries, and when it's not the case, surely someone has already built a third party library. In case of Rust, JSON support comes out of the box in the [serialize::json](http://doc.rust-lang.org/serialize/json/) module.

**Note**: in this article I deliberately do not focus on the Web, APIs, requests and so on. I mentioned JSON in a [previous post about hyper](http://siciarz.net/24-days-of-rust-hyper/) but now I don't care where did the JSON-encoded data come from or what to do with it later. Here I'm just going to show you a few practical tips - Rust has already a [quite extensive documentation](http://doc.rust-lang.org/serialize/json/) on the subject.

Slightly offtopic: I'm writing this blogpost while getting ready to coach a group of fantastic women at [Django Girls](http://djangogirls.org/) in Łódź, Poland. If you're a woman interested in learning programming (or know such girls) check out if there's a Django Girls workshop near you! The workshops focus on basic Python, Django and web technologies, but the general idea is to get the attendees genuinely interested in programming and empowered to create.

That said, now back to JSON and Rust.

Primitives
----------

A lot of Rust types serialize to JSON just as you would expect. Note that `encode()` immutably borrows its argument:

    :::rust
    extern crate serialize;

    use serialize::json;

    fn main() {
        println!("{}", json::encode(&amp;42i));
        println!("{}", json::encode(&amp;vec!["to", "be", "or", "not", "to", "be"]));
        println!("{}", json::encode(&amp;Some(true)));
    }

`Option<T>` maps to the encoding of `value` itself if it is a `Some(value)` while `None` maps to `null`.

Automatic (de)serialization
---------------------------

In my earlier [article on CSV](https://siciarz.net/24-days-of-rust-csv/) I mentioned the `Encodable` and `Decodable` traits. Here's an example with a nested struct:

    :::rust
    use serialize::{Decodable, Encodable, json};

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

    let user = User {
        name: "Zbyszek".to_string(),
        post_count: 100u,
        likes_burgers: true,
        avatar: Some(Photo {
            url: "http://lorempixel.com/160/160/".to_string(),
            dimensions: (160u, 160u),
        }),
    };
    println!("{}", json::encode(&amp;user));
    // {"name":"Zbyszek","post_count":100,"likes_burgers":true,"avatar":{"url":"http://lorempixel.com/160/160/","dimensions":[160,160]}}

Pretty printing
---------------

The `json::encode()` doesn't care for readability of it's output. Although the JSON it emits is correct and machine-readable, there are no newlines or indents making it hard for a human to debug. Pretty-printing is a little bit more complex than just one function call, but not too complicated:

    :::rust
    use serialize::json::PrettyEncoder;

    let mut buffer: Vec<u8> = Vec::new();
    {
        let mut encoder = PrettyEncoder::new(&amp;mut buffer);
        user.encode(&amp;mut encoder).ok().expect("JSON encode error");
    }
    let encoded = String::from_utf8(buffer).unwrap();
    println!("{}", encoded);

You'll need a `PrettyEncoder` which requires a [Writer](http://doc.rust-lang.org/std/io/trait.Writer.html). Fortunately for us, `Vec<u8>` (a vector of bytes) implements this trait. Note the explicit scope in the code above. The encoder mutably borrows the buffer and if I didn't do the scope trick, compilation would fail on the `from_utf8` call. (You can't immutably borrow a value which is mutably borrowed at that time.) So in the code above mutable borrow ends when the encoder goes out of scope.

    :::rust
    let decoded: User = json::decode(incoming_request).unwrap();
    println!("My name is {} and I {} burgers",
        decoded.name, if decoded.likes_burgers { "love" } else { "don't like" });
    assert!(decoded.avatar.is_none());

As you cen see, decoding is also pretty easy. But what happens if we don't know all of the fields in advance? We can use another function in the `json` module - `from_str()`. The difference between `from_str()` and `decode()` is that the latter may return some struct implementing `Decodable` while the former returns a [Json](http://doc.rust-lang.org/serialize/json/enum.Json.html) value. This type has a few methods of its own, including `find()`. See the example below:

    :::rust
    let new_request = "{\"id\":64,\"title\":\"24days\",\"stats\":{\"pageviews\":1500}}";
    if let Ok(request_json) = json::from_str(new_request) {
        if let Some(ref stats) = request_json.find("stats") {
            if let Some(ref pageviews) = stats.find("pageviews") {
                println!("Pageviews: {}", pageviews);
            }
        }
    }

We're using the [if let](https://github.com/rust-lang/rfcs/pull/160) language construct which often simplifies pattern matches where we care for only one branch and do nothing if the expression doesn't match. However, at the moment it is hidden behind a feature gate, so you'll need to add a `#![feature(if_let)]` line in your crate root.

The json! macro
---------------

There's one more thing I wanted to show you today. You can embed JSON-like literals directly in your Rust code with the help of the [json_macros](https://github.com/tomjakubowski/json_macros) crate. This is a compiler extension that allows for some nice syntactic sugar like below:

    :::rust
    #![feature(phase)]
    #[phase(plugin)]
    extern crate json_macros;

    let config = json!({
        "hostname": "localhost",
        "port": 6543,
        "allowed_methods": ["get", "post"],
        "limits": {
            "bandwidth": (500 * 16u),
            "rate": null
        }
    });
    println!("Configuration: {}", config);
    println!("Bandwidth: {}", config.search("bandwidth").unwrap());

See also
--------

 * JSON serialization in Rust, [part 1](http://valve.github.io/blog/2014/08/25/json-serialization-in-rust-part-1/) and [part 2](http://valve.github.io/blog/2014/08/26/json-serialization-in-rust-part-2/) - worth reading for an in-depth look on how to implement `Encodable`/`Decodable` yourself

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly and json_macros 0.0.4.
</small>
