# Day 9 - anymap

> Relevancy: 1.6 stable

In this article we will focus on the [anymap](https://crates.io/crates/anymap) crate by [Chris Morgan](http://chrismorgan.info/) of `rust-http` and `teepee` fame. This crate provides the `AnyMap` type - a slightly peculiar, interesting container.

The `AnyMap` type is different from a regular map. For example, a `HashMap` in Rust is a generic type parametrized by `K` - the type of keys and `V` - the type of values stored in map. (There's also a hasher parameter, but it's not relevant here.) Meanwhile, `AnyMap` itself is **not** a generic type. It uses a `HashMap` internally but we don't need to know that; conceptually, `AnyMap` maps from *types* to values. This means that for each and every type there can be at most one value contained in the mapping.

You may ask - why would I ever need something that holds just one value of each type? A lot of programs use some kind of a map from (usually) strings to some arbitrary values, for example to store configuration data, process environment etc. Let me quote Chris on that:

> It’s typically something like map[string]interface{} and is accessed with arbitrary strings which may clash and type assertions which are a little unwieldy and must be used very carefully. (Personally I would consider that it is just asking for things to blow up in your face.) In a language like Go, lacking in generics, this is the best that can be done; such a thing cannot possibly be made safe without generics.

We can use `AnyMap` together with the [newtype idiom](http://aturon.github.io/features/types/newtype.html) to create a strongly typed configuration holder.

```rust
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
    let mut config = AnyMap::new();
    config.insert(HostAddress::DomainName("siciarz.net".to_owned()));
    config.insert(Port(666));
    config.insert(ConnectionLimit(32));
    println!("{:?}", config.get::<HostAddress>());
    println!("{:?}", config.get::<Port>());
    assert!(config.get::<String>().is_none());
    assert!(config.get::<u32>().is_none());
}
```

The output:

```sh
$ cargo run
Some(DomainName("siciarz.net"))
Some(Port(666))
```

Here the `Port` and `ConnectionLimit` types are abstractions over the underlying integer (with no overhead at runtime!). It is also impossible to mix these two - they are totally different types (not aliases to `u32`) and you can't pass a `Port` value where a `ConnectionLimit` is expected. This fact suggests that we can use these as separate entries in the `AnyMap`. And that is correct, as shown in the example above. It's also worth noting that inserting a value wrapped in a newtype does not make the original type appear in the mapping (seems obvious, I guess).

When we insert another value of a type that already exists in the `AnyMap`, the previous value gets overwritten. Even if this is another enum variant - as enum variants are values grouped under one type - and remember we think of `AnyMap` as mapping from *types* to values.

```rust
config.insert(HostAddress::Ip(Ipv4Addr::new(127, 0, 0, 1)));
println!("{:?}", config.get::<HostAddress>());
```

```sh
$ cargo run
Some(Ip(127.0.0.1))
```

Generic types are considered different for every type parameter, so for example every `Option`-al type gets a separate entry in the `AnyMap`.

```rust
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
```
