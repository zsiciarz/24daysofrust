# Day 19 - rusti

> Relevancy: **outdated**

A few days ago the [Rust subreddit](http://www.reddit.com/r/rust/comments/2phjon/rusti_reborn_my_unofficial_workinprogress_rust/) got excited about the rebirth of [rusti](https://github.com/murarth/rusti) - an interactive shell for Rust. Such interpreters (sometimes called REPLs - [read-eval-print loop](http://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)) are the bread and butter of dynamic languages like Lisp, Python or Ruby. But that is not a requirement - as Haskell demonstrates with its GHCi, you can have a REPL for a statically typed, compiled language too. `rusti` is an attempt to build such an interpreter for Rust.

`rusti` is still under development and currently installable only from source checkout. Clone the [repository](https://github.com/murarth/rusti) and start the interpreter with `cargo run`:

    :::sh
    $ cargo run
        Running `target/rusti`
    rusti=> 1u + 4
    5

You can use it as a calculator but remember about type annotations, just like in regular Rust code. Otherwise you'll get an error like:

    :::sh
    rusti=> 1 + 4
    <anon>:13:16: 13:25 error: unable to infer enough type information to locate the impl of the trait `core::fmt::Show` for the type `_`; type annotations required
    <anon>:13 println!("{}", { 1 + 4 });

`rusti` can evaluate most of Rust code and display results. You can use crates and modules from the standard library as shown in a few examples below.

    :::sh
    rusti=> use std::os;
    rusti=> os::args()
    [target/rusti]
    rusti=> use std::iter::AdditiveIterator;
    rusti=> range(1, 1000u).filter(|x| *x % 19 == 3).sum()
    26341
    rusti=> #![feature(phase)]
    rusti=> #[phase(plugin)] extern crate regex_macros;
    rusti=> extern crate regex;
    rusti=> let re = regex!(r"\b[a-z]{6}:[0-9]{3}\b"); re.is_match("qwerty:123")
    true

One great thing about `rusti` is the `.type` command which shows the type of an expression. (It's quite similar to `:type` in GHCi.)

    :::sh
    rusti=> .type Some("Hello world!".to_string())
    Some("Hello world!".to_string()) = core::option::Option<collections::string::String>
    rusti=> .type vec![1u, 2, 3]
    vec![1u, 2, 3] = collections::vec::Vec<uint>
    rusti=> .type std::io::stdin
    std::io::stdin = fn() -> std::io::stdio::StdinReader

Unfortunately there is no code completion yet, but at least `readline` capabilities (such as line editing and history) are available.

There are a few other limitations, some of which are rather inconvenient at the moment (such as `let` declarations being *very* local), but the authors are aware of them. The real issue for me today is that it can't reference external crates. However [this is already being discussed](https://github.com/murarth/rusti/issues/2) and I hope it will happen.

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly.
</small>
