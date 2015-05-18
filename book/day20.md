# Day 20 - zeromq

> Relevancy: **outdated**

[ZeroMQ](http://zeromq.org/) is a language-independent messaging solution. It's not a full-fledged system such as for example [RabbitMQ](http://www.rabbitmq.com/), basically it's just a transport layer. From the programmer's perspective working with it doesn't differ much from ordinary sockets, but there's a lot of power hidden underneath. The [zeromq](https://github.com/zeromq/zmq.rs) crate is a native Rust implementation and while still lacking a lot of features, it is already usable today.

Operational patterns
--------------------

The [ZeroMQ guide](http://zguide.zeromq.org/page:all#Messaging-Patterns) lists several messaging patterns such as request-response, pub-sub, pipeline etc. Different patterns suit different needs for distributed systems; for example the request-response pattern is commonly used for remote procedure calls. This is the only mode of operation implemented in the `zeromq` crate at the moment, but hopefully more will be added soon.

Before we start implementing the client and server, let's prepare some boilerplate code. We will decide whether to run our demo program as client or server based on the commandline argument.

    :::rust
    extern crate zeromq;

    use zeromq::{Context, Msg, SocketType};

    fn main() {
        let args = std::os::args();
        if args.len() < 2 {
            println!("Usage: {} (client|server)", args[0]);
            return;
        }
        let ctx = Context::new();
        let addr = "tcp://127.0.0.1:25933";
        if args[1] == "client" {
            println!("ZeroMQ client connecting to {}", addr);
            // TODO
        }
        else {
            println!("ZeroMQ server listening on {}", addr);
            // TODO
        }
    }


Client
------

    :::rust
    let mut sock = ctx.socket(SocketType::REQ);
    let _ = sock.connect(addr);
    let payload = "Hello world!".to_string();
    println!("-> {}", payload);
    let mut msg = box Msg::new(payload.len());
    msg.data = payload.into_bytes();
    let _ = sock.msg_send(msg);
    if let Ok(msg) = sock.msg_recv() {
        let contents = String::from_utf8(msg.data).ok().expect("Not a UTF-8 string");
        println!("<- {}", contents);
    }

A ZeroMQ request starts with opening a `REQ` socket. The sockets send and receive `Msg` objects, so most of the client code is just encoding and storing the payload in the `msg.data` attribute (which is just a vector of bytes - `Vec<u8>`). You can use any encoding you like, ZeroMQ doesn't enforce anything. It can be JSON, [msgpack](https://github.com/mneumann/rust-msgpack), [protobuf](https://github.com/stepancheg/rust-protobuf), whatever - as long as you push some bytes over the wire, ZeroMQ is happy.

Server
------

We're going to build a simple echo server that repeats the incoming message in the response.

    :::rust
    let mut sock = ctx.socket(SocketType::REP);
    let _ = sock.bind(addr);
    loop {
        if let Ok(msg) = sock.msg_recv() {
            let mut response = box Msg::new(msg.data.len());
            response.data = msg.data;
            let _ = sock.msg_send(response);
        }
    }

The server opens a `REP` socket and then loops infinitely and echoes back incoming message data. In my first implementation I forgot to send the response and got weird socket errors - turns out a response is necessary in a request/response mode, who would have thought...

Let's start the server now:

    :::sh
    $ cargo run -- server
    ZeroMQ server listening on tcp://127.0.0.1:25933

And if we fire up the client in a new tab we should see a roundtrip message:

    :::sh
    $ cargo run -- client
    ZeroMQ client connecting to tcp://127.0.0.1:25933
    -> Hello world!
    <- Hello world!

See also
--------

* [ZeroMQ an introduction](http://nichol.as/zeromq-an-introduction)
* [rust-zmq](https://github.com/erickt/rust-zmq) - Rust bindings to the C ZeroMQ library
* [ØMQ Messaging Patterns](http://learning-0mq-with-pyzmq.readthedocs.org/en/latest/pyzmq/patterns/patterns.html)

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly.
</small>
