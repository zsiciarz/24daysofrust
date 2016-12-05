# Day 5 - environment variables

> Environment variables are a set of dynamic named values that can affect the
> way running processes will behave on a computer.

That's Wikipedia. Let's read it again. *Dynamic* - because they can change.
*Named* - because like any other variables, they have names.
*Affect processes* - this is the most important part. Environment variables
tell the program in what context it is running - what's the current language,
where is user's home directory etc. They can also store configuration for
the process. For example, a popular cloud hosting platform
([Heroku](https://devcenter.heroku.com/articles/config-vars)) exposes
configuration values to the app as environment variables.

Standard library
----------------

The simplest way to access environment variables from Rust is to use the
built-in [`std::env`](https://doc.rust-lang.org/std/env/) module. It exposes
a few useful functions, such as `vars()` which returns an iterator over
all environment variables. In the example below we're using `var()` to read
current language setting.

```rust
use std::env;

fn main() {
    match env::var("LANG") {
        Ok(lang) => println!("Language code: {}", lang),
        Err(e) => println!("Couldn't read LANG ({})", e),
    };
}
```

```text
$ cargo run
Language code: pl_PL.UTF-8
```

envy
----

[`envy`](https://crates.io/crates/envy) is a small crate that uses `serde`
to automatically deserialize process environment into a Rust struct.

```rust
#![feature(proc_macro)]

extern crate envy;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Environment {
    lang: String,
}

match envy::from_env::<Environment>() {
    Ok(environment) => println!("Language code: {}", environment.lang),
    Err(e) => println!("Couldn't read LANG ({})", e),
};
```

`envy` normalizes variable names to lower case, so `LANG` becomes `lang`
attribute of the struct. But because it's just `serde`, we can use
[serde attributes](https://serde.rs/attributes.html#field-attributes) to
control renames. Another thing worth noting is `serialize_with` attribute.
For example if we had a comma-separated list in an environment variable,
we could deserialize it to a `Vec` with something similar to the code in this
[issue on GitHub](https://github.com/serde-rs/serde/issues/581).

dotenv
------

Sometimes we don't want to export a lot of environment variables manually.
We could write a shell script that wraps our application... or we can use
`dotenv`. Dotenv is a convention to put environment variables in a `.env`
file. There are libraries to read `.env` in
[Ruby](https://github.com/bkeepers/dotenv),
[Python](https://github.com/theskumar/python-dotenv),
[PHP](https://github.com/vlucas/phpdotenv) and a few other languages, but
here we're interested in the [`dotenv`](https://crates.io/crates/dotenv) crate.

The `dotenv::dotenv()` function does one simple thing: it sets environment
variables based on `.env` file contents. If we have a `.env` file like this:

```text
EMAIL_FROM=root@localhost
EMAIL_BACKEND=smtp
```

We can now read these from our Rust program like any other environment
variables.

```rust
extern crate dotenv;

dotenv::dotenv().expect("Failed to read .env file");
println!("Email backend: {}",
         env::var("EMAIL_BACKEND").expect("EMAIL_BACKEND not found"));
```

Even better - we can combine `dotenv` with `envy` and read our configuration
stored in the `.env` file into a Rust struct.

```rust
#[derive(Deserialize, Debug)]
struct MailerConfig {
    email_backend: String,
    email_from: String,
}

dotenv::dotenv().expect("Failed to read .env file");
match envy::from_env::<MailerConfig>() {
    Ok(config) => println!("{:?}", config),
    Err(e) => println!("Couldn't read mailer config ({})", e),
};
```

```text
$ cargo run
MailerConfig { email_backend: "smtp", email_from: "root@localhost" }
```

Further reading
---------------

 - [What are PATH and other environment variables, and how can I set or use them?](http://superuser.com/questions/284342/what-are-path-and-other-environment-variables-and-how-can-i-set-or-use-them)
 - [The Twelve-factor app: store config in the environment](https://12factor.net/config)
 - [Implementing Deserialize with serde](https://serde.rs/impl-deserialize.html)
