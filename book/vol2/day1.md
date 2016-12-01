# Day 1 - Cargo subcommands

Welcome to the second edition of **24 Days of Rust**!

Two years ago in December 2014 the
[first edition](https://zsiciarz.github.io/24daysofrust/)  was pretty well
received by the Rust community. That was a hard time - we were still before
the 1.0 release of the language. I had to keep up with updating my examples
day by day since almost every nightly broke *something*. Even so, I managed
to publish all 24 blogposts on schedule :-)

This year I'm back with another series of articles about Rust tools and
libraries. The languge is at version 1.13 as I'm writing this introduction.
It's definitely more stable and mature at this point than it was in
those savage pre-1.0 times. Some of my examples will probably still require a
nightly release, but I'll try to keep them updated when relevant features get
into stable.

With that, let's dive in into our first topic, which is....

Cargo subcommands
-----------------

By now you're probably familiar with commands like `cargo build`, `cargo run`,
`cargo test` etc. But these are just the built-in commands. Cargo is extensible,
allowing you to plug in third party subcommands. I'll briefly describe some of
those in this post. Read on!

Subcommand discovery
--------------------

First of all, what are those subcommands and how does Cargo know what code to
run? Turns out it's very simple. If you have an executable named
`cargo-embiggen` in your `PATH`, then it will be available as `cargo embiggen`
subcommand.

`cargo fmt`
-----------

If you write code in other languages, there are probably some coding style
guides you follow (or at least have heard of). In Python land there's
[PEP 8](https://www.python.org/dev/peps/pep-0008/), PHP has their coding
standards spread across [several](http://www.php-fig.org/psr/psr-1/)
[PSRs](http://www.php-fig.org/psr/psr-2/), and a lot of organizations have
their internal style guides for the languages they use. But having a coding
style is one thing, actually sticking to it is another. Wouldn't it be nice
to have some automated tool to enforce code formatting? Fortunately there's one
for Rust.

Enter [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt), which is the
officially supported code formatter for Rust. If you install it with
`cargo install rustfmt`, then a `cargo fmt` command becomes available. Run
it on your project to fix any code that doesn't necessarily follow best
formatting practices. For example, `rustfmt` can turn this:

```rust
fn make_it_happen<T, U>(x: &Option<T>, y: U) -> Result<U, String> where T: std::io::Write, U: std::io::Read
{
    match *x
    {
        Some(_) => {Ok(y)},
        None => Err("not happening".into())
    }
}
```

into this:

```rust
fn make_it_happen<T, U>(x: &Option<T>, y: U) -> Result<U, String>
    where T: std::io::Write,
          U: std::io::Read
{
    match *x {
        Some(_) => Ok(y),
        None => Err("not happening".into()),
    }
}
```

`rustfmt` fixed where clause, unnecessary braces and trailing commas. The
code is now consistent with current Rust style guidelines. If you use
one of popular text editors or IDEs, you can probably hook `rustfmt`/`cargo fmt`
on save, so all your Rust sources will be formatted automatically.

`cargo outdated`
----------------

This command is the equivalent of `npm outdated` or `pip list --outdated`.
Install it with `cargo install cargo-outdated`, and then run `cargo outdated`
in your project directory. For example in one of my projects:

```text
$ cargo outdated
Checking for SemVer compatible updates...Done
Checking for the latest updates...Done
The following dependencies have newer versions available:

    Name                   Project Ver  SemVer Compat  Latest Ver
    hound                     1.1.0          --          2.0.0
    rustfft                   1.0.0        1.0.1         1.0.1
```

As you can see, it tells me I can probably safely update the `rustfft`
crate to its latest version. On the other hand, `hound` might have introduced
breaking changes, so there's no [SemVer](http://semver.org/)-compatible version.

**Note**: always test your project after upgrading your dependencies, even if
it's only a patch version. SemVer is only a promise not to break your code,
not a strict guarantee.

`cargo clippy`
--------------

`clippy` (which takes its name from an obnoxious
[MS Office feature](https://en.wikipedia.org/wiki/Office_Assistant)) is
actually a very useful tool, compared to its namesake. It is a linter that
checks your code against common mistakes and suggests idiomatic improvements.
At this moment clippy has over
[170 different checks](https://github.com/Manishearth/rust-clippy/wiki),
so it's definitely worth trying out on your codebase.

If you install it with `cargo install clippy` (requires nightly), you can run it
as a Cargo subcommand - `cargo clippy`.

For example:

```rust
fn needs_improving(x: bool) {
    if x.clone() == true {
        println!("x is true");
    } else {
        if 42 > std::i32::MAX {
            panic!("WAT");
        }
    }
}
```

This code is well-formatted and compiles succesfully. However, there are a few
things that could be improved here and `clippy` tells you just that:

```text
warning: this `else { if .. }` block can be collapsed, #[warn(collapsible_if)] on by default
  --> src\example.rs:55:12
   |
55 |     } else {
   |            ^
   |
help: try
   |     } else if 42 > std::i32::MAX {
   |     panic!("WAT");
   | }
   = help: for further information visit https://github.com/Manishearth/rust-clippy/wiki#collapsible_if

warning: equality checks against true are unnecessary, #[warn(bool_comparison)] on by default
  --> src\example.rs:53:8
   |
53 |     if x.clone() == true {
   |        ^^^^^^^^^^^^^^^^^
   |
help: try simplifying it as shown:
   |     if x.clone() {
   = help: for further information visit https://github.com/Manishearth/rust-clippy/wiki#bool_comparison

warning: using `clone` on a `Copy` type, #[warn(clone_on_copy)] on by default
  --> src\example.rs:53:8
   |
53 |     if x.clone() == true {
   |        ^^^^^^^^^
   |
help: try removing the `clone` call
   |     if x == true {
   = help: for further information visit https://github.com/Manishearth/rust-clippy/wiki#clone_on_copy

warning: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false, #[warn(absurd_extreme_comparisons)] on by default
  --> src\example.rs:56:12
   |
56 |         if 42 > std::i32::MAX {
   |            ^^^^^^^^^^^^^^^^^^
   |
   = help: because std::i32::MAX is the maximum value for this type, this comparison is always false
   = help: for further information visit https://github.com/Manishearth/rust-clippy/wiki#absurd_extreme_comparisons
```

Note how each of these checks points you to clippy wiki for more explanation if
it's still unclear from the message. **Thanks clippy!**

Before I finish this article, I need to point out that there's unfortunately
no (un)official tutorial on how to write a custom Cargo subcommand.
That's an idea for a future blogpost :-)

Further reading
---------------

 * [Third party cargo subcommands](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands)
 * [Cargo subcommands at libs.rs](http://libs.rs/cargo-subcommands/)
 * [Pull request to add templating to Cargo](https://github.com/rust-lang/cargo/pull/3004)
 * [Clippy as a service](https://clippy.bashy.io/)
