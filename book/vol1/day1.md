# Day 1 - Cargo and crates.io

Inspired by Ollie Charles and his excellent [24 days of Hackage](https://ocharles.org.uk/blog/pages/2013-12-01-24-days-of-hackage.html) series, I'm going to try and introduce you to a number of Rust language features, useful libraries and cool projects built with Rust. In fact this is a learning opportunity for me too - as much as I love Rust, I'm just diving in. If you think I'm wrong or know an interesting library you want me to write about, feel free to comment!

So let's start! As in the Haskell series by Ollie, where he [introduced Cabal](https://ocharles.org.uk/blog/posts/2012-12-01-24-days-of-hackage.html), the first post will briefly cover package management. For those of you coming from Python, Ruby or Node this may be a bit familiar. C++ does not have a dedicated package manager, so hopefully this is one of the selling points of Rust for people with such background.

Cargo
-----

**Cargo** is the package manager for Rust. It comes installed along with the compiler if you use the `rustup.sh` script. Cargo builds your code and manages its dependencies. It can also generate a basic project structure for you, if you're just starting a new one:

```sh
$ cargo new myproject --bin # or --lib, if you're creating a library
```

Cargo will then generate some initial sources and a `Cargo.toml` file (sometimes called a *manifest*). This is where you describe your project's metadata, such as name, version, etc. It's also the right place to declare any possible dependencies your project might have. See for example [Cargo.toml](https://github.com/zsiciarz/euler.rs/blob/7e7f93c395a8eb010221015fa3585d8c70663cd7/Cargo.toml) from one of my toy projects:

```ini
[package]

name = "euler"
version = "0.0.1"
authors = ["Zbigniew Siciarz <zbigniew@siciarz.net>"]

[dependencies]
getopts = "~0.2.14"
num = "~0.1.36"
permutohedron = "~0.2.2"
primal = "~0.2.3"
```

Cargo can also run a test suite, generate documentation or upload your crate to the repository, but that's the topic for later.

Crates and dependencies
-----------------------

If you don't know, Rust calls its compilation unit (be it a library or an executable) a **crate**. Your Rust program usually lives in one crate, but it can use other crates as dependencies. See the [Rust Guide on crates](http://doc.rust-lang.org/guide.html#crates-and-modules) for details on the difference between crates and modules.

Executing `cargo build` compiles your crate and the resulting binary can be found inside the `target/` directory. For executables there's a nice shortcut - `cargo run` - that automatically runs your program after it's compiled.

You probably noticed a `[dependencies]` section in the manifest file above. You guessed it - this is where you declare which external libraries you'll use. But `Cargo.toml` is not enough - it just tells Cargo to link your project with these crates. In order to use their APIs in your code, put `extern crate` imports in your project's crate root (that is usually the `main.rs` or `lib.rs` file), so that the compiler can resolve function names, types etc.

```rust
// main.rs
extern crate num;

fn main () {
    // ... use stuff from num library
}
```

**Note**: At the moment Cargo supports only source dependencies; it downloads the source code for every crate your project depends on, compiles them locally on your machine and finally links with your main crate.

What I personally like about Cargo is that it's very simple to build both a library and an executable using that library. It's just a matter of having two sections in the manifest. For example, this is what [rust-cpuid](https://github.com/zsiciarz/rust-cpuid) does:

```ini
[lib]
name = "cpuid"

[[bin]]
name = "cpuid"
```

Then I just need to put `extern crate cpuid` in my `main.rs` file (this is the crate root for the executable) and Cargo will link it with the library it built alongside.

**Update**: As [Steve Klabnik](http://www.reddit.com/r/rust/comments/2nybtm/24_days_of_rust_cargo_and_cratesio/cmip7xw) noted, these two sections are redundant as they're the defaults. If you have both `lib.rs` and `main.rs` in the source directory, Cargo will build the library and executable just the same.

crates.io
---------

[crates.io](https://crates.io/) is the central package repository for Cargo. (Python folks - think of PyPI.) Cargo pulls dependencies from there, although it can also install them from Git or local filesystem. Once you've built an awesome library and want to share it with the rest of the world, crates.io is the place to go. You'll need an account on the site (currently GitHub-based social login only, but this may change). The relevant command is `cargo publish`, documented in the [section on publishing crates](http://doc.crates.io/crates-io.html#publishing-crates). Once you've done it, congratulations, you are now a contributor to the Rust ecosystem!
