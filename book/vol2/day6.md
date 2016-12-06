# Day 6 - derive_builder

Today our focus is the [derive_builder](https://crates.io/crates/derive_builder)
crate. It provides a macro to automatically generate setter methods
for struct fields. And since all setters return the struct itself,
we can chain them in a so called *builder pattern*.

Builder and fluent interfaces
-----------------------------

[Builder pattern](https://en.wikipedia.org/wiki/Builder_pattern) (in the sense
of the original Design Patterns book) is an object that knows how to construct
other objects step by step. The builder approach aims to simplify object
creation. Instead of a long argument list in the constructor (or a method
 like `new()` in Rust), we initialize the object by calling subsequent
 builder methods.

Each builder method returns the builder itself. This allows for chaining
the calls right after each other `just().like().this()`. For example
[diesel](http://diesel.rs/) uses such method chaining to build database queries:

```rust
let versions = Version::belonging_to(krate)
    .select(id)
    .order(num.desc())
    .limit(5);
```

Fluent interfaces are useful in some contexts; they are not a silver bullet
(nothing is). Marco Pivetta wrote a good critique in his blog post
[Fluent interfaces are evil](https://ocramius.github.io/blog/fluent-interfaces-are-evil/).

Game config example
-------------------

Imagine we're writing a computer game. We need to store game
configuration such as screen resolution, save directory location etc. A lot
of modern games start via a separate launcher which tries to autodetect
these values and presents initial config to the actual game. Let's see
how we could use `derive_builder` in such a launcher.

```rust
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate derive_builder;
```

[custom_derive](https://crates.io/crates/custom_derive) is a helper crate that
allows deriving user-defined attributes. We'll see that later, but here's a
regular derive:

```rust
#[derive(Debug)]
struct Resolution {
    width: u32,
    height: u32,
}

impl Default for Resolution {
    fn default() -> Resolution {
        Resolution {
            width: 1920,
            height: 1080,
        }
    }
}
```

`Resolution` is just a pair of integers with a debugging representation
derived for us by the Rust compiler. We could also derive `Default`, but this
would use default values for both dimensions, giving us a 0px x 0px screen. Not
really useful...

```rust
custom_derive! {
    #[derive(Debug, Default, Builder)]
    struct GameConfig {
        resolution: Resolution,
        save_dir: Option<String>,
        autosave: bool,
        fov: f32,
        render_distance: u32,
    }
}
```

Here's the meat of the matter. `Debug` and `Default` are standard Rust traits
that the compiler knows how to derive. `Builder` on the other hand is actually
a `Builder!` macro, that `custom_derive!` knows how to call on our struct.

So this is a plain Rust struct inside some macro. What does it buy us?

```rust
let mut conf = GameConfig::default();
conf.save_dir("saves".to_string()).fov(70.0).render_distance(1000u32);
println!("{:?}", conf);
```

```text
$ cargo run
GameConfig { resolution: Resolution { width: 1920, height: 1080 }, save_dir: Some("saves"), autosave: false, fov: 70, render_distance: 1000 }
```

Note how the resolution and autosave flag are still correctly set from the
defaults, while we use generated setters for the rest of settings.
Another interesting thing happens when we set `save_dir`. The struct contains
an `Option<String>`, but we're passing a `String` into the setter. How does
that even compile?

Since [Rust 1.12](https://blog.rust-lang.org/2016/09/29/Rust-1.12.html),
`Option<T>` implements `From<T>`. And the generated setter signature
would look like:

```rust
pub fn save_dir<VALUE: Into<Option<String>>(&mut self, value: VALUE) -> &mut Self
```

`From` and `Into` traits complement each other, hence everything typechecks.

We can of course add `impl GameConfig` and implement our own methods - this is
still a regular Rust struct. The only thing `derive_builder` does is adding
public setter methods.

(Non)-consuming builders
------------------------

There's a small gotcha with `derive_builder`. We can't initialize config like
this:

```rust
let mut conf = GameConfig::default().fov(0.0);
```

This will fail to compile with an error like below:

```text
error: borrowed value does not live long enough
  --> src\day6.rs:34:50
   |
34 |     let mut conf = GameConfig::default().fov(0.0);
   |                    ---------------------         ^ temporary value dropped here while still borrowed
   |                    |
   |                    temporary value created here
...
37 | }
   | - temporary value needs to live until here
   |
   = note: consider using a `let` binding to increase its lifetime
```

`derive_builder` prefers a so called *non-consuming* builder approach, as
described in [The builder pattern](https://aturon.github.io/ownership/builders.html).
See [this issue](https://github.com/colin-kiegel/rust-derive-builder/issues/2)
for the rationale behind this choice.

Further reading
---------------

 - [The builder pattern](https://aturon.github.io/ownership/builders.html)
 - [FluentInterface by Martin Fowler](http://martinfowler.com/bliki/FluentInterface.html)
 - [Fluent interfaces](http://www.erikschierboom.com/2014/10/08/fluent-interfaces/) (in C#)
 - [Fluent interfaces are evil](https://ocramius.github.io/blog/fluent-interfaces-are-evil/)
 - [smelter](https://crates.io/crates/smelter) - a similar crate for deriving builders
