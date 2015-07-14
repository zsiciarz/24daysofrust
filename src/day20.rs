extern crate zmq;

use zmq::{Context, Message};

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
        let mut sock = ctx.socket(zmq::REQ).unwrap();
        let _ = sock.connect(addr);
        let payload = "Hello world!";
        println!("-> {:?}", payload);
        let mut msg = Message::new().unwrap();
        sock.send(payload.as_bytes(), 0).unwrap();
        sock.recv(&mut msg, 0).unwrap();
        let contents = msg.as_str().unwrap();
        println!("<- {:?}", contents);
    }
    else {
        println!("ZeroMQ server listening on {}", addr);
        let mut sock = ctx.socket(zmq::REP).unwrap();
        let _ = sock.bind(addr);
        let mut msg = Message::new().unwrap();
        loop {
            if let Ok(_) = sock.recv(&mut msg, 0) {
                sock.send_str(msg.as_str().unwrap(), 0).unwrap();
            }
        }
    }
}
