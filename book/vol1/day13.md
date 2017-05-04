# Day 13 - uuid

> Relevancy: 1.9 stable

Writing the [Postgres chapter](day11.md) reminded me of one more cool thing in PostgreSQL - native support for the [UUID](http://www.postgresql.org/docs/9.4/static/datatype-uuid.html) type. The popular web framework Django [will soon get a UUID field](https://github.com/django/django/commit/ed7821231b7dbf34a6c8ca65be3b9bcbda4a0703), Rails [already support it](http://edgeguides.rubyonrails.org/active_record_postgresql.html#uuid). So what's the big deal?

Simply put, using UUIDs can improve distributed systems by removing the need for a centralized ID generation. Let me quote [Andrzej Krzywda's justification](http://andrzejonsoftware.blogspot.com/2013/12/decentralise-id-generation.html):

> For years, I have been thinking that centralised id generation is the natural way. Seems obvious. After I learnt mote about UUID, I start seeing more and more good things about it. It allows for less coupling between components/services.  You no longer need to 'ask' for the id. In case of JavaScript frontends it solves the problem of whether to block the UI, when we create a new resource.

For me, looking from the web development perspective, this is an area of experimentation as I'm not using UUIDs in any of my public projects (yet). However most of the languages I use have a function or library to generate UUIDs and Rust is no exception here. The [uuid](https://crates.io/crates/uuid) crate was once in the standard library, but [got pulled out](https://github.com/rust-lang/rust/issues/8784) during the cleanup.

As usual we'll start by adding the dependency to `Cargo.toml`:

```ini
[dependencies.uuid]
version = "~0.2.0"
features = ["v4"]
```

Let's generate a few UUIDs and display them in the most familar form:

[include:1-3](../../vol1/src/bin/day13.rs)
[include:7-9](../../vol1/src/bin/day13.rs)

The example output would look like:

```sh
$ cargo run
982bb2a5-9e32-450a-9736-bb2dd00d0b4a
9b44c913-c28f-4f5a-be56-2aec4aaa5384
86f137e7-2d8e-4224-9f7a-1724a9503e41
59db4c26-46f7-49b9-9942-636bdc0c1f89
49326e6f-f9c3-40b1-8041-f7dadd503c81
0eabbd4d-13e8-4a76-9892-119096cdab72
6f530ac3-4605-459a-9df7-73026f15c361
17fccb8e-f5e2-4281-ba75-82a50d7f007e
3c7c02b2-4a50-4a56-8a21-bcde9d46aff0
5a405806-f967-4228-a20a-2afaec18e501
```

We can use the `parse_str()` method to check if a string represents a valid UUID and convert it to an `Uuid` value.

[include:10-13](../../vol1/src/bin/day13.rs)

(Web developers might recognize that specific value.) Here's the output:

```sh
$ cargo run
Err(InvalidLength(35))
Err(InvalidCharacter('x', 0))
Err(InvalidGroupLength(0, 7, 8))
Ok(Uuid { bytes: [210, 124, 219, 110, 174, 109, 17, 207, 150, 184, 68, 69, 83, 84, 0, 0] })
```

See also
--------

 * [Use UUIDs as Keys](http://blog.joevandyk.com/2013/08/14/uuids-as-keys/)
 * [A Better ID Generator for PostgreSQL](http://rob.conery.io/2014/05/29/a-better-id-generator-for-postgresql/)
 * [How to start using UUID in ActiveRecord with PostgreSQL](http://blog.arkency.com/2014/10/how-to-start-using-uuid-in-activerecord-with-postgresql/)
 * The [postgres](https://crates.io/crates/uuid) crate supports UUID fields
