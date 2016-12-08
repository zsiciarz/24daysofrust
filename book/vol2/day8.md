# Day 8 - serde

Two years ago I wrote an article about
[working with JSON](https://siciarz.net/24-days-of-rust-working-json/) in Rust.
JSON (de)serialization support was then baked in the standard library. However,
at that time Rust was at version 0.13 and a lot of things happened since then.
Mainly, the [`rustc-serialize`](https://crates.io/crates/rustc-serialize) crate
got pulled out of the core libraries, but kept its close relation to the
`rustc` compiler itself. (Hence the slightly awkward name.)

Meanwhile, a new contender arose: `serde`. It is also a generic serialization
framework for Rust. It's more modern, actively maintained and gets lots
of love from the community. There's a selection of supported data formats,
including JSON, YAML, MessagePack as well as several others. Even the
[official docs for rustc-serialize](https://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/)
say (emphasis mine):

> While this library is the standard way of working with JSON in Rust,
> there is a next-generation library called **Serde** that's in the works
> (it's **faster**, overcomes some design limitations of rustc-serialize and
> **has more features**). You might **consider using it** when starting a new
> project or evaluating Rust JSON performance.

From `struct` to JSON
---------------------

Remember our game configuration struct from
[a few days ago](https://siciarz.net/24-days-rust-derive_builder/)? Let's
revisit it now.

```rust
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug, Default)]
struct GameConfig {
    save_dir: Option<String>,
    autosave: bool,
    fov: f32,
    render_distance: u32,
}

let config = GameConfig::default();
let json = serde_json::to_string(&config).expect("Couldn't serialize config");
println!("{}", json);
```

```text
$ cargo run
{"save_dir":null,"autosave":false,"fov":0.0,"render_distance":0}
```

The `proc_macro` feature gate allows us to use the new macro system (more on that
in a moment). Macros located in `serde_derive` transparently implement custom
`derive`-d traits. Our struct is a regular Rust data type, nothing fancy here
apart from `Serialize, Deserialize` in the derive-able trait list. These
come from `serde` and apply to things that can be (de)serialized.

> **Aside**: We're using nightly Rust (1.15 as of today) to take advantage of new macro
> system. Historically, `serde` used a compiler plugin to generate boilerplate
> code for dealing with serialization of custom types. Compiler plugins,
> however, are inherently unstable; when the compiler internals change,
> the plugins should be updated as well. This is part of the motivation behind
> [macros 1.1 RFC](https://github.com/rust-lang/rfcs/blob/master/text/1681-macros-1.1.md).
> This RFC even mentions `serde` directly.
>
> So should we still use nightly for convenient (de)serialization? At this
> moment, yes. However, custom `derive` attributes **will** be
> [stabilized](https://github.com/rust-lang/rust/issues/35900)! So in a few
> releases from now, we'll be able to use `serde`, `diesel` and a few other
> crates on stable Rust with the same ease as today on nightly.

If we would like the output to be more readable, we can replace the call to
`to_string()` with `to_string_pretty()` and that's it.

```rust
let json = serde_json::to_string_pretty(&config).expect("Couldn't serialize config");
println!("{}", json);
```

```text
$ cargo run
{
  "save_dir": null,
  "autosave": false,
  "fov": 0.0,
  "render_distance": 0
}
```

From YAML to `struct`
---------------------

[YAML](https://en.wikipedia.org/wiki/YAML) is another popular format for
human-readable configuration. We can use
[serde_yaml](https://crates.io/crates/serde_yaml) to read and write YAML
with `serde`.

```rust
extern crate serde_yaml;

#[derive(Deserialize, Debug)]
struct Task {
    name: String,
    command: String,
}

#[derive(Deserialize, Debug)]
struct Play {
    #[serde(rename="hosts")]
    host_list: String,
    tasks: Vec<Task>,
}

type Playbook = Vec<Play>;

let yaml = include_str!("../data/playbook.yml");
println!("{}", yaml);
let playbook = serde_yaml::from_str::<Playbook>(&yaml);
println!("{:?}", playbook);
```

In this example we're reading a minimal [Ansible](https://www.ansible.com/)
playbook. Ansible is an infrastructure automation tool which stores its
configuration in YAML files, called *playbooks* by convention. If we model
our playbook elements (such as tasks and plays) as Rust structs that derive
`Deserialize`, reading a playbook is straightforward. One thing to note is
that field names do not need to agree exactly between our struct and the data
format. We can use `#[serde(rename=<NAME>)]` to tell `serde` to serialize
the field under a different name. Here, a `hosts` key in the playbook
will be mapped to `host_list` field of the struct.

```text
$ cargo run
---
- hosts: all
  tasks:
    - name: tell the time
      command: date
Ok([Play { host_list: "all", tasks: [Task { name: "tell the time", command: "date" }] }])
```

This is a typical pattern when working with `serde`: define the types, either
derive or implement `Deserialize`/`Serialize` manually, and that's all :) And
if we mess up our struct or try to read some malformed data, `serde` and
related crates have rather good and informative error types.

Further reading
---------------

 - [Serde By Example: Part 1 - Serialization](http://src.codes/serde-by-example-part-1-serialization.html)
 - [serde.rs](https://serde.rs/) - a very thorough documentation
 - [Serializers - Are We Web Yet?](http://www.arewewebyet.org/topics/serializer/)
 - [Macros 1.1 RFC](https://github.com/rust-lang/rfcs/blob/master/text/1681-macros-1.1.md)
 - [Tracking issue for (de)serialize_with helpers](https://github.com/serde-rs/serde/issues/553)