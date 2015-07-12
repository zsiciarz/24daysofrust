# Day 19 - rusti

> Relevancy: 1.3 nightly

A few days ago the [Rust subreddit](http://www.reddit.com/r/rust/comments/2phjon/rusti_reborn_my_unofficial_workinprogress_rust/) got excited about the rebirth of [rusti](https://github.com/murarth/rusti) - an interactive shell for Rust. Such interpreters (sometimes called REPLs - [read-eval-print loop](http://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)) are the bread and butter of dynamic languages like Lisp, Python or Ruby. But that is not a requirement - as Haskell demonstrates with its GHCi, you can have a REPL for a statically typed, compiled language too. `rusti` is an attempt to build such an interpreter for Rust.

`rusti` is still under development and currently installable only from source checkout. Clone the [repository](https://github.com/murarth/rusti) and start the interpreter with `cargo run`:

```sh
$ cargo run
    Running `target/rusti`
rusti=> 1 + 4
5
```

`rusti` can evaluate most of Rust code and display results. You can use crates and modules from the standard library as shown in a few examples below.

```sh
rusti=> use std::env;
rusti=> env::args().collect::<Vec<_>>()
["target/debug/rusti"]
rusti=> use std::iter::AdditiveIterator;
rusti=> (1..100).filter(|x| *x % 19 == 3).fold(0, |acc, x| acc + x)
303
```

One great thing about `rusti` is the `.type` command which shows the type of an expression. (It's quite similar to `:type` in GHCi.)

```sh
rusti=> .type Some("Hello world!".to_string())
Some("Hello world!".to_string()) = core::option::Option<collections::string::String>
rusti=> .type vec![1, 2, 3]
vec![1, 2, 3] = collections::vec::Vec<i32>
rusti=> .type std::io::stdin
std::io::stdin = fn() -> std::io::stdio::Stdin {std::io::stdio::stdin}
```

Unfortunately there is no code completion yet, but at least `readline` capabilities (such as line editing and history) are available.

There are a few other limitations, some of which are rather inconvenient at the moment (such as `let` declarations being *very* local), but the authors are aware of them. The real issue for me today is that it can't reference external crates. However [this is already being discussed](https://github.com/murarth/rusti/issues/2) and I hope it will happen.
