#[cfg(target_family = "unix")]
extern crate zmq;

#[cfg(target_family = "unix")]
use zmq::{Context, Message, Error};


#[cfg(target_family = "unix")]
fn run_client(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REQ)?;
    sock.connect(addr)?;
    let payload = "Hello world!";
    println!("-> {:?}", payload);
    let mut msg = Message::new()?;
    sock.send(payload.as_bytes(), 0)?;
    sock.recv(&mut msg, 0)?;
    let contents = msg.as_str().unwrap();
    println!("<- {:?}", contents);
    Ok(())
}

#[cfg(target_family = "unix")]
fn run_server(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;
    let mut msg = Message::new()?;
    loop {
        if sock.recv(&mut msg, 0).is_ok() {
            sock.send(msg.as_str().unwrap().as_bytes(), 0)?;
        }
    }
}

#[cfg(target_family = "windows")]
fn main() {
    println!("24 days of Rust - zmq (day 20)");
    println!("This example does not work on Windows :(");
}

#[cfg(target_family = "unix")]
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
    } else {
        println!("ZeroMQ server listening on {}", addr);
        run_server(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
    }
}
