# Day 5 - hyper

The state of HTTP libraries in Rust is a constant flux. See [Are we web yet?](http://arewewebyet.com/) for an overview of the current affairs. There's `rust-http` which although usable (for example [Nickel](http://nickel.rs/) builds on top of that) is not developed anymore. [Teepee](http://teepee.rs/), it's conceptual successor, is in the words of it's author *not even vaguely usable*. Meanwhile a new library emerged during the last few months: [hyper](https://github.com/hyperium/hyper), which will be the subject of this blogpost.

I'm going to focus on using `hyper` only as a client, although the library contains also a server implementation. However with the advance of Rust web frameworks building on top of HTTP libraries, the programmers will focus less on developing servers and more on the clients. Consuming web APIs is a lot more common than writing new shiny servers. How can `hyper` help us?

Basic requests
--------------

Let's start from the usual dependency definition in `Cargo.toml`.

    :::rust
    [dependencies.hyper]
    git = "https://github.com/hyperium/hyper"

Note: at the time of writing this post the version hosted on crates.io does not compile with the latest nightly. That's why the dependency points to a git repository.

When you run `cargo build`, Cargo will download a few other required crates (for URL handling, mimetype support, OpenSSL bindings etc.) and hopefully compile `hyper` afterwards. Time for our first request!

    :::rust
    extern crate hyper;

    use hyper::Url;
    use hyper::client::Request;

    fn main() {
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

That was... *verbose*. I could just use `unwrap()` everywhere, but that would be handwaving and in poor taste. Sprinkling your code with `panic!` is not a sign of good style too. However, there are so many things that can go wrong during an HTTP request/response cycle! But there seems to be a pattern. Can we do better?

    :::rust
    fn get_content(url: &amp;str) -> HttpResult<String> {
        let url = match Url::parse(url) {
            Ok(url) => url,
            Err(_) => return Err(HttpError::HttpUriError),
        };
        let fresh_request = try!(Request::get(url));
        let streaming_request = try!(fresh_request.start());
        let mut response = try!(streaming_request.send());
        Ok(try!(response.read_to_string()))
    }

    fn main() {
        println!("{}", get_content("http://httpbin.org/status/200"));
    }

We refactored the request cycle into a separate function. But look how the code got simpler, thanks to the [try! macro](http://doc.rust-lang.org/std/result/#the-try!-macro). There's no explicit matching on the `HttpResult` variants and the first `try!` that fails will return from the function with some kind of an HTTP error. Unfortunately we have to do it explicitly in case of `Url::parse()`. It would be possible to use `try!` there too if the following code compiled:

    :::rust
    extern crate hyper;
    extern crate url;

    use std::error::FromError;
    use hyper::HttpError;
    use url::ParseError;

    impl FromError<ParseError> for HttpError {
        fn from_error(err: ParseError) -> HttpError {
            HttpError::HttpUriError
        }
    }

This is the current mechanism for [interoperation between errors](https://github.com/rust-lang/rfcs/blob/master/text/0201-error-chaining.md).
Unfortunately we get scolded by the compiler saying `error: cannot provide an extension implementation where both trait and type are not defined in this crate`. But if that `impl` was bundled with hyper... *gently prodding the maintainers* :-)

POST and query parameters
-------------------------

Sending POST requests with hyper is only a little bit more complicated. We'll write a wrapper function again, this time taking an additional argument of type `Query`.

    :::rust
    extern crate url;

    use hyper::header::ContentLength;
    use url::form_urlencoded;

    type Query<'a> = Vec<(&amp;'a str, &amp;'a str)>;

    fn post_query(url: &amp;str, query: Query) -> HttpResult<String> {
        let url = match Url::parse(url) {
            Ok(url) => url,
            Err(_) => return Err(HttpError::HttpUriError),
        };
        let body = form_urlencoded::serialize(query.into_iter());
        let mut fresh_request = try!(Request::post(url));
        fresh_request.headers_mut().set(ContentLength(body.len()));
        let mut streaming_request = try!(fresh_request.start());
        try!(streaming_request.write_str(body[]));
        let mut response = try!(streaming_request.send());
        Ok(try!(response.read_to_string()))
    }

    let query = vec![("key", "value"), ("foo", "bar")];
    println!("{}", post_query("http://httpbin.org/post", query));

The main differences from `get_content()` are more mutable variables and the serialization machinery. Once we've built a raw request body (like `key=value&amp;foo=bar`), we can take advantage of `Writer` implementation for `Request<Streaming>` and call `write_str()` with a slice of the raw body. We also have to set the `Content-Length` header, which is strongly typed in hyper.

Sending JSON
------------

Our `post_query` function can be easily changed to borrow a struct, serialize it to JSON and send it over the wire.

    :::rust
    extern crate serialize;

    use serialize::{Encodable, json};
    use std::io::IoError;

    fn post_json<'a, T>(url: &amp;str, payload: &amp;T) -> HttpResult<String>
            where T: Encodable<json::Encoder<'a>, IoError> {
        let body = json::encode(payload);
        // rest of the code as before
    }

This function is generic in its `payload` argument, accepting anything that implements the `Encodable` trait. We use a `where` clause to specify trait bounds which are slightly complex in this case. We can use the function as follows:

    :::rust
    #[deriving(Encodable)]
    struct Movie {
        title: String,
        bad_guy: String,
        pub_year: uint,
    }

    let movie = Movie {
        title: "You Only Live Twice".to_string(),
        bad_guy: "Blofeld".to_string(),
        pub_year: 1967,
    };
    println!("{}", post_json("http://httpbin.org/post", &amp;movie));

Where to go from here?
----------------------

As you may have noticed, hyper's HTTP client API is not as high-level as for example [requests](http://docs.python-requests.org/en/latest/). However the library is still under active development and the API may or may not change. Secondly, there are a few projects that aim to wrap hyper behind a simpler facade - I mention two of these at the end of this blog post. The future of HTTP in Rust will change for sure, but I'm hopeful!

See also
--------

* [HTTP library requirements](https://github.com/servo/servo/wiki/HTTP-library-requirements) from the Servo project
* [rust-request](https://github.com/jgillich/rust-request) - a HTTP client library written on top of hyper
* [rest_client](https://github.com/gtolle/rest_client) - another HTTP client built with hyper
* [Improved error handling in Rust](http://lucumr.pocoo.org/2014/11/6/error-handling-in-rust/) by Armin Ronacher

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly.
</small>
