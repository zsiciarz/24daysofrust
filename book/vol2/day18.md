# Day 18 - error_chain

If you have a background in Python, Java or C++, you're probably used to
raising exceptions if something goes wrong. Rust doesn't have exceptions.
The official Rust book has a
[comprehensive chapter on error handling](https://doc.rust-lang.org/book/error-handling.html),
but the TL;DR is we should probably use the `Result` type. We can match
on its variants to handle both the happy path and error cases in a very
explicit, if not verbose, way. To address the verbosity, there was a `try!`
macro that cut down on a lot of pattern matching boilerplate. And as of now
we have an even simpler syntax - the `?` operator. But when there are many
error types, possibly coming from different libraries, making them compose well
still requires a lot of code: `From` and `Error` implementations and such.

The [`error_chain`](https://crates.io/crates/error-chain) crate was created
to avoid all that remaining boilerplate. Let's see it in action!

Results and Errors and ErrorKinds, oh my!
-----------------------------------------

We will build a simple command line utility called `json2cron`.
It's going to read a JSON file with a schedule of commands to run, convert
it into a format that `cron` understands and finally feed that into `crontab`.
(One could argue that it's better to learn `crontab` syntax rather than
write custom JSON, but I'm not going to discuss the usefulness of our tool
here.)

But first let's start with adding `error_chain` to our project.

```rust
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use errors::*;
```

With only these few lines of code, we have now:

 - custom `Error` and `ErrorKind` types
 - a `Result` type wrapping the standard library `Result` with a fixed
   error type - the custom `Error` mentioned above
 - a `ResultExt` trait that adds a `chain_err()` method to standard library
   `Result`s

Our `main()` function will follow the template recommended by Brian Anderson in
the [getting started with `error_chain`](http://brson.github.io/2016/11/30/starting-with-error-chain)
blog post.

```rust
fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        std::process::exit(1);
    }
}
```

This will print the entire error chain and possibly a backtrace of the original
error. We're also good command line citizens and return with a non-zero exit
code.

With `main()` out of the way, we can now focus on the `run()` function.

Different errors in json2cron
-----------------------------

A lot of things can go wrong. File I/O, JSON parsing, even calling `crontab`
can fail. And if Murphy is right, *if anything can go wrong, it will*.
But we're prepared for that, we're using Rust! If you've seen the
[*if programming languages were people*](http://leftoversalad.com/c/015_programmingpeople/)
cartoon, you'll remember Rust being portrayed as a knight with three shields
and a witty message. I like to think of `error_chain` as a squire loyal to the
Rust knight.

```rust
fn run() -> Result<()> {
    let schedule = load_schedule("data/schedule.json").chain_err(|| "failed to load schedule")?;
    if schedule.rules.is_empty() {
        bail!("the schedule is empty");
    }
    update_crontab(&schedule).chain_err(|| "failed to update crontab")
}
```

We're doing three things in the `run()` function. First we need to load the
schedule from a JSON file, using `chain_err()` to add some more context to any
errors that `load_schedule()` may return. Next, we check business logic rules,
such as requiring a non-empty schedule. The `bail!` macro does an early return,
converting its argument into an `Err` with our custom error inside. Finally
we try to update crontab with a new schedule.

The `Schedule` type is not important here. It's just a struct that is
JSON-serializable (with `serde`) and implements the `Display` trait. You can
find the entire source code for this example in the
[24daysofrust repository](https://github.com/zsiciarz/24daysofrust).

```rust
#![feature(proc_macro)]
#[macro_use]

extern crate serde_derive;
extern crate serde_json;
extern crate tempfile;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn load_schedule<P: AsRef<Path>>(path: P) -> Result<Schedule> {
    let file = File::open(path).chain_err(|| "failed to open input file")?;
    serde_json::from_reader(&file).chain_err(|| "failed to read JSON")
}

fn update_crontab(schedule: &Schedule) -> Result<()> {
    let mut file =
        tempfile::NamedTempFile::new().chain_err(|| "failed to create a temporary file")?;
    let schedule_str = format!("{}", schedule);
    file.write_all(&schedule_str.into_bytes()[..]).chain_err(|| "failed to write schedule")?;
    let path = file.path().to_str().ok_or("temporary path is not UTF-8")?;
    Command::new("crontab").arg(path)
        .spawn()
        .chain_err(|| "failed to run crontab command")?;
    Ok(())
}
```

We're using several APIs here that return `Result`s. Thanks to the `ResultExt`
trait implementation generated by `error_chain!`, all of them have a
`chain_err()` method, even though they may come from external crates.
Note the `ok_or("...")?` call. `Path::to_str()` returns an `Option` and
not a `Result`, but we can fix that with `Option::ok_or()`. And thanks to the
`?` operator we can still return immediately in case of an error.

Let's raise our shields, run the program a few times and see what happens. What
if the JSON file is missing?

```text
$ cargo run
error: failed to load schedule
caused by: failed to open input file
caused by: The system cannot find the file specified. (os error 2)
```

Let's fix that part by adding the file, but make it invalid JSON.

```text
$ cargo run
error: failed to load schedule
caused by: failed to read JSON
caused by: expected `:` at line 2 column 11
```

We can see that the general message (*failed to load schedule*) didn't change,
but the actual *reason* why it failed is different. Yay, error chaining!

And what if the `crontab` command fails for some reason?

```text
$ cargo run
error: failed to update crontab
caused by: failed to run crontab command
caused by: The system cannot find the file specified. (os error 2)
```

The actual operating system error isn't really helpful here, but it boils
down to the `crontab` command being unavailable on Windows.

Embrace Open Source
-------------------

If you want to find real-life examples of how a Rust crate works (which is
what I did with `error_chain` before writing this article), I have a tip
for you. The page for each crate on crates.io has a `Dependent crates` link,
which leads to
[a listing like this](https://crates.io/crates/error-chain/reverse_dependencies).
From there it's just a matter of opening several browser tabs with repositories
for interesting crates and using code search to find the APIs you want. A few
examples:

 - [`chain_err` usage in cargo-chrono](https://github.com/nikomatsakis/cargo-chrono/search?utf8=%E2%9C%93&q=chain_err) -
   where errors from filesystem, CSV parser or regex engine can work together
 - [`foreign_links` and custom errors in conserve](https://github.com/sourcefrog/conserve/blob/d00c060faae1d45312c102cb51a67b85ca5195c7/src/errors.rs)
 - [a lot of `foreign_links` in rq](https://github.com/dflemstr/rq/blob/4c04fee237c87815009377a2d20bf7a9596478b5/src/error.rs)

Further reading
---------------

 - [Starting a new Rust project right, with error-chain](http://brson.github.io/2016/11/30/starting-with-error-chain)
 - [Stroustrup's Rule and Layering Over Time](https://thefeedbackloop.xyz/stroustrups-rule-and-layering-over-time/)
 - [Error handling](https://doc.rust-lang.org/book/error-handling.html) in The Rust Programming Language
 - [Error handling](http://rust-lang.github.io/book/ch09-00-error-handling.html) in the upcoming second edition of the above book