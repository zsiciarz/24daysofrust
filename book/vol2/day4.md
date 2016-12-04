# Day 4 - slog

Structured logging
------------------

I hope you're using **any** sort of logging in your applications. Even if that
means printing stuff to stdout and relying on shell output redirection,
it's still better than no logging at all. However, most programming languages
have great libraries for logging. Be it Java, Python, Elixir - there are
logging utilities everywhere. But more often than not, what is logged is some
sort of prose that a programmer thought was applicable at a time.

I once wrote this line of Python code:

```python
logger.error('Oh no, something terrible happened: %s', details)
```

Even though I've configured my logging levels, formatters and other stuff
that's very useful, in the end I still had to `grep terrible` to find
what actually happened, and later parse the details with a nasty regexp.
Not very convenient. **Structured logging** is a concept that puts *events*
over *messages*; we're now logging events with associated key-value data,
not plain string messages.

`slog` basics
-------------

[`slog`](https://crates.io/crates/slog) is the single most popular crate for
structured logging in Rust. It supports various outputs (called *drains*
in `slog` terminology), but for now we'll stick to the terminal output provided
by the [`slog_term`](https://crates.io/crates/slog-term) crate.

```rust
#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

fn main() {
    let drain = slog_term::streamer().build().fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => "0.5"));
    info!(root_logger, "Application started";
        "started_at" => format!("{}", time::now().rfc3339()));
}
```

The first line in `main()` creates and configures a drain which will output
log messages to the terminal. By default, `slog_term` autodetects color
terminals and uses a compact style for output. Next we're creating the root
logger. Every application must have a root logger, so it makes sense to
provide some initial logging context from the start, such as app version.
We're using a convenience 'o!` macro to define that context as key-value pairs.

Once we have a logger, we can log stuff! There's a couple macros such as
`debug!`, `info!`, `err!` etc. to log events of different severity. We can add
more context in each call to these macros. Here lies the power of
structured logging: it's **composable**. Every time we add some more context,
we retain the context defined when logger was created. See the output:

```text
$ cargo run
Nov 30 20:46:47.073 INFO Application started, version: 0.5, started_at: 2016-11-30T20:46:47+01:00
```

Even though the call to `info!` doesn't mention app version at all, it is there.
Cool, but there's more. We can create child loggers that will inherit their
context from parent logger. Let's assume our app has some users that can
sign in.

```rust
struct User {
    username: String,
    logger: slog::Logger,
}

impl User {
    fn new(username: &str, logger: &slog::Logger) -> Self {
        User {
            username: username.to_string(),
            logger: logger.new(o!("username" => username.to_string())),
        }
    }

    fn sign_in(&self) {
        info!(self.logger, "User signed in");
    }
}

// in main:
let user = User::new("zbyszek", &root_logger);
user.sign_in();
```

```text
$ cargo run
Nov 30 20:59:42.811 INFO User signed in, username: zbyszek, version: 0.5
```

Our `User` objects contain their own loggers that inherit from the root logger
and add context relevant to user interactions. The event inside `sign_in()`
method still logs app version coming from the root logger, while enhancing the
context with the username of the user who signs in. Again, **composition** is
a strong selling point of `slog`.

> Note: we can avoid passing `Logger` around in the arguments, thanks to
> the [`slog-scope`](https://crates.io/crates/slog-scope) crate. I decided
> not to do that here to make the examples more explicit.

Other outputs
-------------

With [`slog-stream`](https://crates.io/crates/slog-stream) we can have
drains that write events to anything implementing
[`io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html).
Let's build a drain that writes both to stdout and a log file.

```rust
extern crate slog_json;
extern crate slog_stream;

use std::fs::File;

let console_drain = slog_term::streamer().build();
let file = File::create("app.log").expect("Couldn't open log file");
let file_drain = slog_stream::stream(file, slog_json::default());
let logger = slog::Logger::root(slog::duplicate(console_drain, file_drain).fuse(), o!());
warn!(logger, "not_enough_resources"; "resource" => "cat pictures");
```

The `slog_json::default()` function creates a JSON formatter with some
convenient keys like timestamp and log level already added to the objects.
We feed it in the second argument to `stream()` which accepts anything that
is writable - like files, stdout or network sockets.
Later we use `duplicate()` to create a higher level drain that will
forward events to both outputs.

```text
$ cargo run
Dec 01 22:25:08.520 WARN not_enough_resources, resource: cat pictures
$ cat app.log
{"ts":"2016-12-01T22:25:08.521265800+01:00","level":"WARN","msg":"not_enough_resources","resource":"cat pictures"}
```

Of course we could stream events into something more appropriate than a simple
file, like [Logstash](https://www.elastic.co/products/logstash) or
[Graylog](https://www.graylog.org/).

There's a few more bits and pieces related to `slog`. We can filter log levels
either with drains, or entirely at compile time. `slog` itself removes trace
and debug logging in release builds. There's also a way to log events
asynchronously to improve performance. Furthermore, if you've already
used `env_logger` before, `slog` comes with
[a shim](https://github.com/slog-rs/envlogger) to ease transition.

Further reading
---------------

 - [Getting started with slog](https://github.com/slog-rs/slog/wiki/Getting-started)
 - [structlog - Structured Logging for Python](http://www.structlog.org/en/stable/)
 - [serilog](https://serilog.net/) - structured logging for .NET
 - [Benefits of Structured Logging vs basic logging](http://softwareengineering.stackexchange.com/questions/312197/benefits-of-structured-logging-vs-basic-logging)
