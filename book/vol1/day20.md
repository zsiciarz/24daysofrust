# Day 20 - zeromq

> Relevancy: 1.9 stable

[ZeroMQ](http://zeromq.org/) is a language-independent messaging solution. It's not a full-fledged system such as for example [RabbitMQ](http://www.rabbitmq.com/), basically it's just a transport layer. From the programmer's perspective working with it doesn't differ much from ordinary sockets, but there's a lot of power hidden underneath. The [rust-zmq](https://github.com/erickt/rust-zmq) crate is a Rust binding to the C library. There used to be a working native binding ([zeromq](https://github.com/zeromq/zmq.rs)), but it's now undergoing a redesign and rewrite.

Operational patterns
--------------------

The [ZeroMQ guide](http://zguide.zeromq.org/page:all#Messaging-Patterns) lists several messaging patterns such as request-response, pub-sub, pipeline etc. Different patterns suit different needs for distributed systems; for example the request-response pattern is commonly used for remote procedure calls. This is the only mode of operation implemented in the `zeromq` crate at the moment, but hopefully more will be added soon.

Before we start implementing the client and server, let's prepare some boilerplate code. We will decide whether to run our demo program as client or server based on the commandline argument.

[include:2-2](../../src/day20.rs)
[include:5-5](../../src/day20.rs)
[include:44-58](../../src/day20.rs)

Client
------

[include:9-20](../../src/day20.rs)

A ZeroMQ request starts with opening a `REQ` socket. The sockets send and receive `Message` objects. You can use any encoding you like, ZeroMQ doesn't enforce anything. It can be JSON, [msgpack](https://github.com/mneumann/rust-msgpack), [protobuf](https://github.com/stepancheg/rust-protobuf), whatever - as long as you push some bytes over the wire, ZeroMQ is happy.

Note that we're using the [try! macro](http://doc.rust-lang.org/std/result/#the-try!-macro) for error handling.

Server
------

We're going to build a simple echo server that repeats the incoming message in the response.

[include:23-33](../../src/day20.rs)

The server opens a `REP` socket and then loops infinitely and echoes back incoming message data. In my first implementation I forgot to send the response and got weird socket errors - turns out a response is necessary in a request/response mode, who would have thought...

Let's start the server now:

```sh
$ cargo run -- server
ZeroMQ server listening on tcp://127.0.0.1:25933
```

And if we fire up the client in a new tab we should see a roundtrip message:

```sh
$ cargo run -- client
ZeroMQ client connecting to tcp://127.0.0.1:25933
-> "Hello world!"
<- "Hello world!"
```

See also
--------

* [ZeroMQ an introduction](http://nichol.as/zeromq-an-introduction)
* [rust-zmq](https://github.com/erickt/rust-zmq) - Rust bindings to the C ZeroMQ library
* [ØMQ Messaging Patterns](http://learning-0mq-with-pyzmq.readthedocs.org/en/latest/pyzmq/patterns/patterns.html)
