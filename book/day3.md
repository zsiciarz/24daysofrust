# Day 3 - csv

> Relevancy: 1.6 stable

Most of us programmers have encountered the [CSV format](http://en.wikipedia.org/wiki/Comma-separated_values) at some point of our career. Whether you cooperate with financial people, analyze some scientific data or simply allow the users of your web app to download a record of their activities, chances are you'll use some variation of CSV as the data format. Note that I said *some variation* - CSV itself isn't standardized and there are lots of quirks in different implementations.

CSV libraries exist for lots of languages, making it a common format for interoperability (alongside XML or JSON) and sometimes preferred for data of a tabular nature. In the Rust ecosystem there is the [csv crate](https://crates.io/crates/csv) which will be the focus of this blog post.

Writing to CSV
--------------

One would think that there's nothing simpler than writing a CSV file. Join the stringified values with commas and that's it, right? Unfortunately it's not that simple, what if the values contain commas, quotes, new line characters etc.? At this point you need a CSV library which knows how to handle all these edge cases. Fortunately the `csv` crate provides a `Writer` type that takes care of all that.

[include:15-24](../src/day3.rs)

Now let's check the output if the `Writer` handled comma in the last title correctly:

```sh
$ cat westerns.csv
A Fistful of Dollars,Rojo,1964
For a Few Dollars More,El Indio,1965
"The Good, the Bad and the Ugly",Tuco,1966
```

Yes! So we can write vectors of things as CSV rows, fine. But what if our application represents the data as some custom type, do we have to build a vector from that? Imagine this is an online movie catalog of some sorts. Having a `Movie` struct with `title`, `bad_guy` fields etc. is a better API design than relying on the order of items in a tuple or vector.

[include:2-2](../src/day3.rs)
[include:6-12](../src/day3.rs)

We need to import the `rustc_serialize` crate so that Rust can derive for us the `RustcEncodable` trait. By the way, this also enables serializing `Movie` objects to JSON.

[include:25-31](../src/day3.rs)

Try removing the `#[derive(RustcEncodable)]` attribute and see what happens. Turns out the CSV writer can handle anything that implements `RustcEncodable`.

CSV parsing
-----------

Writing CSV  is one part of the story. If you're a client of some API that exposes CSV data, you'll need to have a way to read that into some meaningful representation. But define meaningful? Let's start with plain tuples.

[include:32-36](../src/day3.rs)

We need to give the reader a hint regarding field types. If we changed it for example to `(String, i32, usize)`, `unwrap` would panic with a CSV decode error. However changing `usize` to `String` would work, although we would have to explicitly parse the field as integer.

```sh
$ cargo run
("For a Few Dollars More", "El Indio", 1965)
("The Good, the Bad and the Ugly", "Tuco", 1966)
("Hang \'Em High", "Wilson", 1968)
```

Wait, where's the first dollar movie (*A Fistful of Dollars*)? The `Reader` by default considers the first row in a CSV file as headers, which are not exposed in the iterator returned by `decode()`. You can use the `has_headers()` method to disable this behaviour.

[include:32-32](../src/day3.rs)

```sh
$ cargo run
("A Fistful of Dollars", "Rojo", 1964)
("For a Few Dollars More", "El Indio", 1965)
("The Good, the Bad and the Ugly", "Tuco", 1966)
("Hang \'Em High", "Wilson", 1968)
```

There is also a nice symmetry with the `Writer`. We can serialize structs to CSV, so we should be able to read into structs directly. If the struct implements `RustcDecodable` trait (usually by deriving), we can do it!

[include:37-42](../src/day3.rs)

You can find a few more examples in the [csv crate docs](http://burntsushi.net/rustdoc/csv/). It's also possible to change the delimiter (for example if you have TSV data - tab separated values), quote characters and row separators. I think it would be fantastic if the library allowed for different CSV *dialects*, as does the [Python standard library](https://docs.python.org/3.4/library/csv.html#csv-fmt-params). Other than that, the `csv` crate is definitely usable and quite performant. There are also ways to improve performance even more by giving up on convenient struct manipulation and using low-level field API directly.

Check out also [xsv](https://github.com/BurntSushi/xsv) - a commandline toolkit for working with CSV data written in Rust. Try reading the source to see how it uses the `csv` crate.
