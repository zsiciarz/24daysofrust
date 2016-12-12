# Day 12 - clap

[`clap`](https://crates.io/crates/clap) is a fantastic Rust library
for **C**ommand **L**ine **A**rgument **P**arsing. It's both
easy to use and powerful - in the spirit of Rust philosophy - you get what
you pay for. Simple CLI options are simple to define, while complex schemes
(think `git` level of complex) are absolutely doable.

`clap` is also one of the best examples of what I would call
**developer marketing** in the Rust community. It has a
[beautiful and informative](https://clap.rs/) homepage, an extensive
[README](https://github.com/kbknapp/clap-rs) (including changelog - see note
below), a bunch of
[good examples](https://github.com/kbknapp/clap-rs/tree/master/examples), even
[video tutorials](https://www.youtube.com/playlist?list=PLza5oFLQGTl2Z5T8g1pRkIynR3E0_pc7U)!
Hats off to Kevin and all `clap` contributors, you're doing a great job!

**Note**: Rust crate authors, please, *please* add changelogs/release notes to
your libraries. Coming from Python where it's customary, it struck me that
a lot of libraries do not document their changes aside from the commit log.
(Oops, I'm [guilty of this myself](https://github.com/zsiciarz/rust-cpuid)...)

Simple arguments
----------------

`clap` allows specifying our command line opions in a number of ways. We can
use regular Rust method calls, macros or YAML configuration. I prefer the
first approach - a builder pattern with method chaining - and this is what I'll
be using throughout this article.

```rust
extern crate clap;

use std::process;
use clap::{Arg, ArgMatches, App, SubCommand};

fn main() {
    let matches = App::new("24daysofrust")
        .version("0.1")
        .author("Zbigniew Siciarz")
        .about("learn you some Rust!")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("verbosity level"))
        .get_matches();
    if let Err(e) = run(matches) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```

Before specyfing our first argument, we use `version()`, `author()` and
`about()` to give some general information about our program to `clap`.
If we stopped there, `clap` already knows enough to automatically handle two
common arguments: `--help` and `--version` (or their short forms `-h` and `-V`).

```text
cargo run -- -h
24daysofrust 0.1
Zbigniew Siciarz
learn you some Rust!

USAGE:
    example.exe

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

$ cargo run -- -V
24daysofrust 0.1
```

Awesome! But we're not stopping there. We want to be able to print out some
logs from our program. It's customary to use `-v` or (`--verbose`) to
control the verbosity level of program output. The `Arg::with_name()` call
creates a simple named argument. This struct has a selection of useful
methods such as:

 - `short()` to provide a short, one-letter form
 - `required()` to indicate whether the argument is optional or required
 - `takes_value()` and `default_value()` to read values from arguments like
   `--option=foo`
 - `multiple()` to allow for repeated occurences; here we can call our program
   with `-v` for verbose output, or `-vv` to be even more verbose

The main function uses a pattern I've noticed in a few applications or blog
posts from the community. We process configuration and pass options to a
`run()` function, which does all the work. This function returns a `Result`,
allowing the use of `?` directly in `run()` (we can't do that in `main()`).

```rust
#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

fn run(matches: ArgMatches) -> Result<(), String> {
    let min_log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        2 | _ => slog::Level::Trace,
    };
    let drain = slog::level_filter(min_log_level, slog_term::streamer().build()).fuse();
    let logger = slog::Logger::root(drain, o!());
    trace!(logger, "app_setup");
    // setting up app...
    debug!(logger, "load_configuration");
    trace!(logger, "app_setup_complete");
    // starting processing...
    info!(logger, "processing_started");
    // ...
}
```

We're using [`slog`](https://siciarz.net/24-days-rust-structured-logging/) here
to configure logging level. The more verbose output is requested, the lower
level is set in the `level_filter`. Let's see that in action:

```text
$ cargo run
Dec 12 20:17:01.284 INFO processing_started

$ cargo run -- -v
Dec 12 20:17:17.510 DEBG load_configuration
Dec 12 20:17:17.511 INFO processing_started

$ cargo run -- -vv
Dec 12 20:17:28.763 TRCE app_setup
Dec 12 20:17:28.764 DEBG load_configuration
Dec 12 20:17:28.765 TRCE app_setup_complete
Dec 12 20:17:28.765 INFO processing_started
```

With two (or more) occurences of `v` in the arguments, our app spews out a lot
of debugging and trace information. These are filtered out if we don't want
the verbose output. Perfect!

Subcommands
-----------

A lot of CLI programs like `apt`, `git` or `cargo` group several specialized
tasks under one executable. These subcommands (eg. `apt install`,
`git checkout` or `cargo run`) behave like separate apps and can have their own
options and even nested subcommands. Defining an interface like that is
super easy with `clap`.

```rust
#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};

arg_enum! {
    #[derive(Debug)]
    enum Algorithm {
        SHA1,
        SHA256,
        Argon2
    }
}

let matches = App::new("24daysofrust")
    // ...
    .subcommand(SubCommand::with_name("analyse")
        .about("Analyses the data from file")
        .arg(Arg::with_name("input-file")
            .short("i")
            .default_value("default.csv")
            .value_name("FILE")))
    .subcommand(SubCommand::with_name("verify")
        .about("Verifies the data")
        .arg(Arg::with_name("algorithm")
            .short("a")
            .help("Hash algorithm to use")
            .possible_values(&Algorithm::variants())
            .required(true)
            .value_name("ALGORITHM")))
    // ...
    .get_matches();
```

Our program has two subcommands: `analyse` and `verify`. The first one needs
an input file, but can use a default location. The second has one argument
which is required and doesn't have a default value. By the way, this example
shows how to use enums as arguments with a helper `arg_enum!` macro.

Let's see what should we add to `run()` to delegate work to subcommands:

```rust
fn run(matches: ArgMatches) -> Result<(), String> {
    // ...
    match matches.subcommand() {
        ("analyse", Some(m)) => run_analyse(m, &logger),
        ("verify", Some(m)) => run_verify(m, &logger),
        _ => Ok(()),
    }
}

fn run_analyse(matches: &ArgMatches, parent_logger: &slog::Logger) -> Result<(), String> {
    let logger = parent_logger.new(o!("command" => "analyse"));
    let input = matches.value_of("input-file").unwrap();
    debug!(logger, "analysis_started"; "input_file" => input);
    // ...
    Ok(())
}

fn run_verify(matches: &ArgMatches, parent_logger: &slog::Logger) -> Result<(), String> {
    let logger = parent_logger.new(o!("command" => "verify"));
    let algorithm = value_t!(matches.value_of("algorithm"), Algorithm).unwrap();
    debug!(logger, "verification_started"; "algorithm" => format!("{:?}", algorithm));
    // ...
    Ok(())
}
```

We're matching on the return value of `subcommand()` to dispatch control to
appropriate functions. Calling `value_of(...).unwrap()` in both examples is
safe. `clap` will use default value for `--input-file` if it's not provided,
while it won't allow skipping the required `--algorithm` argument.

Errors and typo corrections
---------------------------

`clap` gives a precise error when the user didn't supply the required argument
or used an unrecognized one. Every error message ends with a `USAGE` section,
so the user can immediately see an example of a correct invocation.

```text
$ cargo run --bin=example -- verify
error: The following required arguments were not provided:
    -a <ALGORITHM>

USAGE:
    clap verify -a <ALGORITHM>
```

If you're a fast typer like me, chances are you make typos and spelling
mistakes all the time. Great news - `clap` can spot these and suggest correct
spelling! And it's built in, you don't have to configure anything else
other than your regular arguments and subcommands.

```text
$ cargo run --bin=example -- analyze
error: The subcommand 'analyze' wasn't recognized
        Did you mean 'analyse'?

If you believe you received this message in error, try re-running with 'example -- analyze'

USAGE:
    example [FLAGS] [SUBCOMMAND]

For more information try --help
```

This may be slightly annoying to my American readers, sorry about that ;-)

And more
--------

We barely scratched the surface of `clap`. We can have related argument groups
using `group()`, aliases (so that you Americans can have your spelling as well),
complex relationships between arguments such as
`required_unless()`/`overrides_with()`/`conflicts_with()`, value validation etc.
[The docs for `Arg`](https://docs.rs/clap/2.19.2/clap/struct.Arg.html) provide
clean, useful examples to all these scenarios.

And if you're tired of writing arguments by hand, `clap` can generate
completions for popular shells. This includes even Windows Powershell.

Further reading
---------------

 - [clap.rs](https://clap.rs/) - just look at that homepage!
 - [clap introduced to /r/rust subreddit (2015)](https://www.reddit.com/r/rust/comments/2xineh/clap_a_simple_getoptlike_argument_parser_first/)
 - [Why use clap instead of docopt or getopts?](https://github.com/kbknapp/clap-rs/wiki/FAQ#why-use-clap-instead-of-docopt-or-getopts)
 - [What are good habits for designing command line arguments?](http://softwareengineering.stackexchange.com/questions/307467/what-are-good-habits-for-designing-command-line-arguments)
