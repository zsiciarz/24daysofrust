# Day 10 - nom, part 1

It's entirely possible that you're walking a happy road of a programmer
who never had to write a parser by hand. That's not my case unfortunately.
I remember incomprehensible indexing of hideous arrays of characters,
a maddening cascade of unmaintainable if-else statements, and futile,
indescribable attempts to abstract away parts of this unspeakable monstrosity.

If the above paragraph was hard to parse, good! Putting some Lovecraftian
adjectives into a description of events can be a way of coping with terrible
experiences. Those dark, eldritch (sorry, couldn't resist one more) days are
fortunately over. With
[parser combinators](https://en.wikipedia.org/wiki/Parser_combinator) we
can write composable and fast parsers. Rust adds another adjective here:
*safe*. [`nom`](https://crates.io/crates/nom) is a parser combinator library
that works by generating parsing code at compile time with a bunch of macros.
It also tries to avoid allocation and work through input bytes without copying.

I decided to split `nom` article into two parts.
Today we're focused on parsing text (well, bytes that contain text),
while the next article in the series will cover binary parsing.

This is my first hands-on experience with parser combinators - I'm learning
`nom` as I write this. Feel free to let me know if the examples here could
be more nom-idiomatic.

Toy HTTP parser
---------------

We'll start our adventure with `nom` by writing a simple parser for HTTP
headers. Even though HTTP itself is a plain text protocol, it's a complex
beast.

**Note**: this is by no means a complete, correct HTTP parser. Its purpose
is only illustrative. In fact, we're only parsing *the first line* of
the HTTP header. Parsing the rest of header fields is left as an exercise
for the reader.

Let's take a look at the first line of HTTP header.

```rust
let first_line = b"GET /home/ HTTP/1.1\r\n";
```

According to [RFC 2616](https://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html),
which is the definition of the HTTP protocol, the first line of the request
starts with method name. It is followed by an URI (`/` in our example) and
finally protocol version. Finally it's terminated by CRLF characters.

```rust
named!(parse_method_v1, alt!(tag!("GET") | tag!("POST")));
println!("{:?}", parse_method_v1(&first_line[..]));
```


We can create a new parser using `named!` macro. It will create a function that
takes a byte slice (`&[u8]`) and returns an
[`IResult`](http://rust.unhandledexpression.com/nom/enum.IResult.html).

The `tag!` macro matches the exact characters given. `alt!` tries to match
any of sub-parsers separated by `|`, returning the result of the first one
that matches. (Note: we're leaving out other HTTP methods for simplicity.)

```text
$ cargo run
Done([32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10], [71, 69, 84])
```

`Done` is a variant of `IResult` returned when the parser succeeds. We're
interested in the second field, which contains the byte sequence matched by the parser.
(We can check that the byte sequence `[71, 69, 84]` is equivalent to `GET`.)
But we'd like to see some text, not raw bytes. We can do this in `nom` using
`map!` or `map_res!` combinators.

```rust
named!(parse_method_v2<&[u8], &str>,
       alt!(map_res!(tag!("GET"), str::from_utf8) | map_res!(tag!("POST"), str::from_utf8)));
println!("{:?}", parse_method_v2(&first_line[..]));
```

`map_res!` takes an existing parser (like `tag!("GET")`) and applies a function
to its result. This function must return a `Result`, like
[`from_utf8`](https://doc.rust-lang.org/stable/std/str/fn.from_utf8.html) in our
example. There's also a version for functions returning `Option` - `map_opt!`,
and one for functions returning "plain" values - `map!`. If you need your
parser to produce something else than byte slices, you'll probably use these
macros a lot.

> Note that we had to add type parameters to our parser inside angle brackets.
> First is the input type - a byte array slice, followed by output type. If you
> omit these types as we did in `parse_method_v1`, the default output type is
> also a byte slice.

```rust
#[derive(Debug)]
enum Method {
    GET,
    POST,
}

named!(parse_method<&[u8], Method>,
       alt!(map!(tag!("GET"), |_| Method::GET) | map!(tag!("POST"), |_| Method::POST)));
```

Here we're using a custom enum for HTTP methods. The parser uses `map!` to
discard matched bytes (we already know we matched either `GET` or `POST`) and
return a respective `Method` variant. We could also implement
[`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) for our enum.

The power of parser combinator libraries such as `nom` lies in the `combinator`
part of the name. We can build very complex parsing machinery from small pieces.
Let's reuse our `parse_method` function to actually parse entire first line of
HTTP header.

```rust
#[derive(Debug)]
struct Request {
    method: Method,
    url: String,
    version: String,
}

named!(parse_request<&[u8], Request>, ws!(do_parse!(
    method: parse_method >>
    url: map_res!(take_until!(" "), str::from_utf8) >>
    tag!("HTTP/") >>
    version: map_res!(take_until!("\r"), str::from_utf8) >>
    (Request { method: method, url: url.into(), version: version.into() })
)));
println!("{:?}", parse_request(&first_line[..]));
```

```text
$ cargo run
Done([], Request { method: GET, url: "/home/", version: "1.1" })
```

The `do_parse!` macro is new in nom 2.0. It's used to chain parsers one after
another. If an intermediate value is required, we can give it a name, for
example in `method: parse_method` the `method` name will hold the result of
`parse_method` parser. Subsequent parsers are put together with `>>` and
the final result can refer to named intermediate results. When we run the code,
we notice it consumed all bytes and succesfully parsed a `Request`.

What if our input doesn't conform to the specification?

```rust
let bad_line = b"GT /home/ HTTP/1.1\r\n";
println!("{:?}", parse_request(&bad_line[..]));
```

```text
$ cargo run
Error(Alt)
```

This time our parser returns an `Error` variant, indicating which of the
subparsers failed (`Alt` points to `alt!` macro). There is an
[extensive guide to error management in nom](http://rust.unhandledexpression.com/nom/error_management.html)
if we wanted to improve the experience.

See you tomorrow for part 2 on binary parsing!

Further reading
---------------

 - [Making a new parser from scratch](http://rust.unhandledexpression.com/nom/making_a_new_parser_from_scratch.html)