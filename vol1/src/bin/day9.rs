extern crate anymap;

use std::net::Ipv4Addr;
use anymap::AnyMap;

#[derive(Debug)]
enum HostAddress {
    DomainName(String),
    Ip(Ipv4Addr),
}

#[derive(Debug)]
struct Port(u32);

#[derive(Debug)]
struct ConnectionLimit(u32);

fn main() {
    println!("24 days of Rust - anymap (day 9)");
    let mut config = AnyMap::new();
    config.insert(HostAddress::DomainName("siciarz.net".to_string()));
    config.insert(Port(666));
    config.insert(ConnectionLimit(32));
    println!("{:?}", config.get::<HostAddress>());
    println!("{:?}", config.get::<Port>());
    assert!(config.get::<String>().is_none());
    assert!(config.get::<u32>().is_none());
    config.insert(HostAddress::Ip(Ipv4Addr::new(127, 0, 0, 1)));
    println!("{:?}", config.get::<HostAddress>());
    if !config.contains::<Option<f32>>() {
        println!("There's no optional 32-bit float in the configuration...");
    }
    let dummy: Option<f32> = None;
    config.insert(dummy);
    if config.contains::<Option<f32>>() {
        println!("There's an optional 32-bit float in the configuration...");
    }
    if !config.contains::<Option<f64>>() {
        println!("...but not an optional 64-bit float.");
    }
}
