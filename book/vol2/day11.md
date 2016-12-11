# Day 11 - nom, part 2

We learned the basic concepts of `nom`
[yesterday](https://siciarz.net/24-days-rust-nom-part-1/)
when we wrote a parser for HTTP headers. HTTP is by its nature a text protocol.
`nom` however always works on bytes (byte array slices, denoted in Rust with
`&[u8]`). This makes it perfectly suitable for parsing binary data as well.
There's already a selection of parsers using `nom` for binary formats such as
[Redis dump files](https://github.com/badboy/rdb-rs),
[MySQL protocol](https://github.com/blackbeam/rust-mysql-simple/blob/master/src/parser.rs)
or  [tar archives](https://crates.io/crates/tar-parser). Today we are going
to build a simplified WebSocket frame parser.

WebSockets
----------

We touched upon HTTP yesterday. It is not a particularily network-efficient
protocol (at least until version 2 came to being). To access a single
resource, the following sequence of events must happen:

 - client opens a connection
 - client sends a request to the server
 - server sends back a response to the client
 - connection is closed

The communication is single-directional and connections aren't persistent
(there are some tricks, but this isn't the place to discuss them).

[WebSockets](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API)
on the other hand provide persistent, bi-directional connections between
browsers and servers. This not only improves performance, but also allows
for interactions that weren't possible over HTTP, such as real-time data
streaming from the server to all connected clients.

Binary parsing
--------------

In contrast with HTTP 1, WebSocket is a binary protocol. It is defined in
 [RFC 6455](https://tools.ietf.org/html/rfc6455), there's also a
[simpler explanation at Mozilla Development Network](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers).
The protocol is frame-based, so it makes sense to parse it frame-by-frame.

A WebSocket frame looks like this:

```text
+-+-+-+-+-------+-+-------------+-------------------------------+
|F|R|R|R| opcode|M| Payload len |    Extended payload length    |
|I|S|S|S|  (4)  |A|     (7)     |             (16/63)           |
|N|V|V|V|       |S|             |   (if payload len==126/127)   |
| |1|2|3|       |K|             |                               |
+-+-+-+-+-------+-+-------------+ - - - - - - - - - - - - - - - +
|     Extended payload length continued, if payload len == 127  |
+ - - - - - - - - - - - - - - - +-------------------------------+
|                               |Masking-key, if MASK set to 1  |
+-------------------------------+-------------------------------+
| Masking-key (continued)       |          Payload Data         |
+-------------------------------- - - - - - - - - - - - - - - - +
:                     Payload Data continued ...                :
+ - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
|                     Payload Data continued ...                |
+---------------------------------------------------------------+
```

Let's express that in Rust:

```rust
#[derive(Debug)]
enum OpCode {
    Continue,
    Text,
    Binary,
    Close,
    Ping,
    Pong,
    Reserved,
}

impl From<u8> for OpCode {
    fn from(opcode: u8) -> OpCode {
        match opcode {
            0 => OpCode::Continue,
            1 => OpCode::Text,
            2 => OpCode::Binary,
            8 => OpCode::Close,
            9 => OpCode::Ping,
            10 => OpCode::Pong,
            _ => OpCode::Reserved,
        }
    }
}

#[derive(Debug)]
struct WebSocketFrame {
    fin: bool,
    opcode: OpCode,
    mask: bool,
    length: u64,
    masking_key: u16,
    payload: Vec<u8>,
}
```

We're using a custom enum to make possible opcodes more strongly typed. The
conversion from a raw byte is done by implementing the
[`From<T>`](https://doc.rust-lang.org/std/convert/trait.From.html) trait.
This is idiomatic Rust - prefer implementing standard traits rather than
writing custom functions for conversions which cannot fail. Note that `from()`
consumes its argument.

And now for the parser itself:

```rust
named!(parse_frame<&[u8], WebSocketFrame>, do_parse!(
    first_byte: bits!(tuple!(take_bits!(u8, 1), take_bits!(u8, 3), take_bits!(u8, 4))) >>
    mask_and_length: bits!(tuple!(take_bits!(u8, 1), take_bits!(u8, 7))) >>
    extended_length: be_u64 >>
    length: value!(match mask_and_length.1 {
        127u8 => extended_length,
        126u8 => extended_length & 0xFFFF_0000_0000_0000u64 >> 24,
        _ => mask_and_length.1 as u64
    }) >>
    masking_key: be_u16 >>
    payload: take!(length) >>
    (WebSocketFrame {
        fin: first_byte.0 == 1,
        opcode: OpCode::from(first_byte.2),
        mask: mask_and_length.0 == 1,
        length: length,
        masking_key: masking_key,
        payload: payload.into(),
     })
));
```

That is a quite complex parser, but in the end we'll get a nice `WebSocketFrame`
instead of some chunks of bytes. Most of the complexity comes from the fact
that payload length in WebSockets is, well, complicated. But let's start
with the first byte:

```rust
first_byte: bits!(tuple!(take_bits!(u8, 1), take_bits!(u8, 3), take_bits!(u8, 4))) >>
// ...
    fin: first_byte.0 == 1,
    opcode: OpCode::from(first_byte.2),
// ...
```

We read the first byte in three groups of 1, 3 and 4 bits respectively.
Later on we convert tuple elements to types of `WebSocketFrame` fields, using
our `From` implementation for `OpCode`. Since the second group of bits is
reserved but unused in the current version of the protocol, we just ignore it.

```rust
// ...
mask_and_length: bits!(tuple!(take_bits!(u8, 1), take_bits!(u8, 7))) >>
extended_length: be_u64 >>
length: value!(match mask_and_length.1 {
    127u8 => extended_length,
    126u8 => extended_length & 0xFFFF_0000_0000_0000u64 >> 24,
    _ => mask_and_length.1 as u64
}) >>
// ...
payload: take!(length) >>
// ...
```

We split the second byte into mask and length tuple in a similar manner as
above. There's a helper `value!` macro in `nom` that we can use to compute
an intermediate value that we will refer to later in the parser chain. The
`match` expression implements the logic to calculate payload length and
afterwards we just take as many bytes as necessary.

**Note**: in a real WebSocket frame the payload would be XOR-ed with
masking key if the mask bit is set. I think this isn't our parser's
responsibility. A higher-level API should take the parsed frame,
decide whether it's text or binary and process payload as required, possibly
decoding the bytes from UTF-8.

So let's run our parser on an example WebSocket frame, represented as a vector
of raw bytes.

```rust
let frame = vec![0b10000001, 0b10000011, 0b00000000, 0b00000000,
                 0b00000000, 0b00000000, 0b00000000, 0b00000000,
                 0b00000000, 0b00000000, 0b00010010, 0b10111001,
                 0b00000001, 0b00000010, 0b00000011]; // [1, 2, 3]
println!("{:?}", parse_frame(&frame[..]));
```

```text
$ cargo run
Done([], WebSocketFrame { fin: true, opcode: Text, mask: true, length: 3, masking_key: 4793, payload: [1, 2, 3] })
```

Reflections after using `nom`
-----------------------------

As I [mentioned yesterday](https://siciarz.net/24-days-rust-nom-part-1/),
`nom` is my first step in the land of parser combinators. All in all,
I find the experience quite enjoyable. Initially I was a little afraid
of macros everywhere, but after a few tries I got the HTTP example working
and grasped the idea of building up parsers from smaller pieces. This helps
to keep code readable. It's also easier to test small, single-responsibility
parsers. I need to learn more about providing good error messages from
parsers, but there's a number of topic guides in
[the nom docs](http://rust.unhandledexpression.com/nom/).
`nom` seems to be gaining traction in the Rust community, so I encourage
you to check it out the next time you're thinking about writing a parser!

Further reading
---------------

 - [Convenient and idiomatic conversions in Rust](https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust)
 - [RFC 6455](https://tools.ietf.org/html/rfc6455)
 - [Websockets 101](http://lucumr.pocoo.org/2012/9/24/websockets-101/)
 - [websocket](https://crates.io/crates/websocket) and [ws](https://crates.io/crates/ws) - two Rust crates for WebSockets