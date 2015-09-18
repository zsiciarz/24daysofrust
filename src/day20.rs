#![feature(plugin)]

#![plugin(clippy)]

extern crate zmq;

use zmq::{Context, Message, Error};

fn run_client(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let mut sock = try!(ctx.socket(zmq::REQ));
    try!(sock.connect(addr));
    let payload = "Hello world!";
    println!("-> {:?}", payload);
    let mut msg = try!(Message::new());
    try!(sock.send(payload.as_bytes(), 0));
    try!(sock.recv(&mut msg, 0));
    let contents = msg.as_str().unwrap();
    println!("<- {:?}", contents);
    Ok(())
}

fn run_server(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let mut sock = try!(ctx.socket(zmq::REP));
    try!(sock.bind(addr));
    let mut msg = try!(Message::new());
    loop {
        if let Ok(_) = sock.recv(&mut msg, 0) {
            try!(sock.send_str(msg.as_str().unwrap(), 0));
        }
    }
    Ok(())
}


fn main() {
    println!("24 days of Rust - zmq (day 20)");
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} (client|server)", args[0]);
        return;
    }
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
    if args[1] == "client" {
        println!("ZeroMQ client connecting to {}", addr);
        run_client(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
    }
    else {
        println!("ZeroMQ server listening on {}", addr);
        run_server(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
    }
}
