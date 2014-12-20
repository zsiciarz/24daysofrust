extern crate zeromq;

use zeromq::{Context, Msg, SocketType};

fn main() {
    println!("24 days of Rust - zeromq (day 20)");
    let args = std::os::args();
    if args.len() < 2 {
        println!("Usage: {} (client|server)", args[0]);
        return;
    }
    let ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
    if args[1] == "client" {
        println!("ZeroMQ client connecting to {}", addr);
        let mut sock = ctx.socket(SocketType::REQ);
        let _ = sock.connect(addr);
        let payload = "Hello world!".to_string();
        let mut msg = box Msg::new(payload.len());
        msg.data = payload.into_bytes();
        let _ = sock.msg_send(msg);
    }
    else {
        println!("ZeroMQ server listening on {}", addr);
        let mut sock = ctx.socket(SocketType::REP);
        let _ = sock.bind(addr);
        loop {
            if let Ok(msg) = sock.msg_recv() {
                let contents = String::from_utf8(msg.data).ok().expect("Not a UTF-8 string");
                println!("{}", contents);
                let reply = "Why yes, hello!".to_string();
                let mut msg = box Msg::new(reply.len());
                msg.data = reply.into_bytes();
                let _ = sock.msg_send(msg);
            }
        }
    }
}
