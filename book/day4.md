# Day 4 - docopt

One of the few chores when building a commandline program is argument parsing, so that `myprogram --config=myfile.conf --verbose -o output.txt` makes sense. Some arguments come in short and long variants, some are optional and some are positional only. There are a lot of libraries for argument parsing, some are even included in the respective languages' distributions. In Rust's case there's the [getopts crate](http://doc.rust-lang.org/getopts/index.html).

The first thing a moderately savvy user will do is... no, not read the documentation, but run `myprogram -h` (or `--help`) to discover available options. `getopts` and other libraries can derive such help summary for you, saving your time and reducing duplication. But what if it was the other way round? You'd write the usage message, listing possible options, and the tool would build an argument parser from that. Enter [docopt](http://docopt.org/).

What is docopt?
---------------

The **docopt** initiative [originated in the Python community](https://www.youtube.com/watch?v=pXhcPJK5cMc) in 2012 as an attempt to standardize common conventions for commandline arguments. The main idea is that the help message describes the interface of your program. Once you follow a few rules when writing that message, docopt can understand it and build an argument parser.

docopt for Rust
---------------

We'll start by declaring a dependency in `Cargo.toml`:

    :::toml
    [dependencies]
    docopt = "~0.6.11"
    docopt_macros = "~0.6.11"

Cargo will now pull the [docopt crate](https://crates.io/crates/docopt) along with the macros which are distributed separately. This is a common pattern in Rust projects because macros introduce a runtime dependency on `libsyntax` (see [one explanation here](https://github.com/Ogeon/rustful/issues/1)).

In our example application we'll try to mimic the `wc` tool for counting lines, words or characters in a file. Let's start with importing required libraries and writing the usage message:

    :::rust
    extern crate serialize;
    extern crate docopt;

    use docopt::Docopt;

    static USAGE: &amp;'static str = "
    Usage: wc [options] [<file>]

    Options:
        -c, --bytes  print the byte counts
        -m, --chars  print the character counts
        -l, --lines  print the newline counts
        -w, --words  print the word counts
        -L, --max-line-length  print the length of the longest line
        -h, --help  display this help and exit
        -v, --version  output version information and exit
    ";

Nothing fancy here, just a long string. See the [docopt specification](http://docopt.org/) for the details of the format. We want to be able to decode the options into a struct, so it makes sanse for the struct to implement the `Decodable` trait.

    :::rust
    #[deriving(Decodable)]
    struct Args {
        arg_file: Option<String>,
        flag_bytes: bool,
        flag_chars: bool,
        flag_lines: bool,
        flag_words: bool,
        flag_max_line_length: bool,
    }

The arguments must map to field names. Flags (toggle switches such as `-c` above) map to `flag_`-prefixed boolean fields, option arguments or meta-variables map to `arg_`-prefixed fields. If the argument is optional it can be represented by an `Option` value. But how do we actually turn the commandline args into a struct?

    :::rust
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

Now this is a dense line of code. Let's walk through it one step at a time.

    :::rust
    let docopt = match Docopt::new(USAGE) {
        Ok(d) => d,
        Err(e) => e.exit(),
    };
    println!("{}", docopt);
    let args: Args = match docopt.decode() {
        Ok(args) => args,
        Err(e) => e.exit(),
    };

`Docopt::new()` returns a `Result<Docopt, Error>` value. The errors from docopt have a handy `exit()` method that prints the error message and quits the program. Printing a `Docopt` value gives us a lot of debugging information. The `decode()` method is responsible for creating our arguments object and we extract it from the `Ok` variant. We can now use `args` as any other struct in our program.

    :::rust
    println!("Counting stuff in {}", args.arg_file.unwrap_or("standard input".to_string()));
    if args.flag_bytes {
        println!("Counting bytes!");
    }
    if args.flag_chars {
        println!("Counting characters!");
    }
    // etc.


docopt_macros
-------------

But hey, didn't I tell you earlier about reducing duplication? In the example above there are two sources of information, namely the usage string and the `Args` struct. It's quite clear that these two represent the same concept, but you'll need to mantain two separate code pieces instead of one and that is prone to errors.

`docopt!` to the rescue! This is a funny (for some value of fun, of course) macro that will generate the struct for us.

    :::rust
    #![feature(phase)]
    extern crate serialize;
    extern crate docopt;
    #[phase(plugin)]
    extern crate docopt_macros;

    docopt!(Args, "
    Usage: wc [options] [<file>]

    Options:
        -c, --bytes  print the byte counts
        -m, --chars  print the character counts
        -l, --lines  print the newline counts
        -w, --words  print the word counts
        -L, --max-line-length  print the length of the longest line
        -h, --help  display this help and exit
        -v, --version  output version information and exit
    ", arg_file: Option<String>)

The macro takes the name of the type to generate, usage string and (optionally) types for the generated fields. It also validates that the usage message conforms to the docopt spec. The validation happens at compile time, when the `Args` struct is generated so there's no runtime overhead. But most importantly we have now a **single** piece of information to maintain instead of two.

There's one more advantage of the macro - our code inside `main()` can be simplified a bit. As the struct is generated from the usage message, we can get rid of one intermediate `Result` unwrapping; the struct has a static `docopt()` method which returns a `Docopt` value.

    :::rust
    let docopt = Args::docopt();
    println!("{}", docopt);
    let args: Args = docopt.decode().unwrap_or_else(|e| e.exit());
    // use args as before

Docopt for Rust recently gained an ability to generate tab completion files for the shell (only bash at the moment). See the [readme](https://github.com/docopt/docopt.rs#tab-completion-support) for more on that.

See also
--------

 * [using docopt in C](http://kblomqvist.github.io/2013/03/21/creating-beatiful-command-line-interfaces-for-embedded-systems-part1/) for embedded systems
 * docopt implementations in various languages under the [docopt organization on GitHub](https://github.com/docopt)
 * [wc in Rust](https://github.com/uutils/coreutils/blob/master/src/wc/wc.rs) from the coreutils rewrite project

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly and docopt 0.6.11.
</small>
