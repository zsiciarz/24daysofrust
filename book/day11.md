# Day 11 - postgres

> Relevancy: 1.6 stable (macros only on nightly)

Yes, I'm biased. [PostgreSQL](http://www.postgresql.org/) is my favorite SQL database. There is already a pure Rust driver for PostgreSQL - the [postgres](https://crates.io/crates/postgres) crate which will be the subject of today's article.

Connecting
----------

In this and the following examples we will assume a Postgres user `rust` with password `rust` and an existing database named... well, `rust`.

[include:6-6](../src/day11.rs)
[include:10-10](../src/day11.rs)
[include:32-39](../src/day11.rs)

The `Connection` type has a few methods related to making queries; perhaps the simplest one is `execute()` which immediately executes the query and returns the number of modified rows (or an error). This method can be used for example for insert/update queries but also DDL as shown below.

[include:40-46](../src/day11.rs)

The second argument looks slightly awkward, but that's the way of telling `execute()` that the query takes no parameters. Later on we will see a few examples that use query parameters.

Prepared queries and statements
-------------------------------

Let's add a few rows to our table. We will use the `prepare()` method.

[include:47-58](../src/day11.rs)

The query was prepared only once and what we got (after some crude error handling) is a `Statement` value. We can use its `execute()` method to actually run the query with the supplied parameters (note the borrowing).

To read the data from the database we will use the same `prepare()` method in conjunction with the `query()` method of a `Statement`. There's a significant difference between `execute()` and `query()`: the former returns just the number of affected rows, while the latter returns a collection of `Row` values.

[include:59-72](../src/day11.rs)

Keep in mind that the `get()` method will panic if it encounters incompatible types, for example if we changed `String` to `i32` above. There's also a safer `get_opt()` method returning a `Result` instead of panicking.

Advanced PostgreSQL types
-------------------------

All this is a bit boring so far. One of the reasons developers love PostgreSQL is its selection of interesting data types. Let's see how to use them in Rust (hint: it's kinda cool). We'll start from writing a generic helper function to read a single value from the first column of the first row.

[include:10-12](../src/day11.rs)
[include:20-28](../src/day11.rs)

We use the `try!` macro to minimize the noise from error handling. Now let's see it in action. The more interesting types like arrays, ranges etc. come from a few additional crates: [postgres_array](https://crates.io/crates/postgres_array) and [postgres_range](https://crates.io/crates/postgres_range).

[include:4-8](../src/day11.rs)
[include:13-18](../src/day11.rs)
[include:73-93](../src/day11.rs)

```sh
$ cargo run
Executing query: select 1=1
Ok(true)
Executing query: select 1=1
Err(WrongType(Bool))
Executing query: select '{4, 5, 6}'::int[]
Ok([4, 5, 6])
Executing query: select '{"foo": "bar", "answer": 42}'::json
Ok(Object({"answer": U64(42), "foo": String("bar")}))
Executing query: select '[10, 20)'::int4range
Ok([10,20))
Executing query: select '[2015-01-01, 2015-12-31]'::tsrange
Ok([Timespec { sec: 1420070400, nsec: 0 },Timespec { sec: 1451520000, nsec: 0 }])
```

Fantastic! The error handling is still there when we need it and we get values of reasonable Rust types.

Compile-time SQL checking
-------------------------

There is another crate worth mentioning here - [postgres_macros](https://crates.io/crates/postgres_macros). It provides the `sql!` macro that validates correctness of the SQL query given as argument **at compile time**.

```rust
let query = sql!("select '{4, 5, 6}'::int[]");
let this_wont_compile = sql!("eslect '{4, 5, 6}'::int[]");
```

See also
--------

 * [deuterium-orm](https://github.com/deuterium-orm/deuterium-orm) - a basic ORM for Rust supporting PostgreSQL
 * [r2d2](https://crates.io/crates/r2d2) - a generic connection pool with [Postgres support](https://crates.io/crates/r2d2_postgres)
 * [Building RESTful API server in Rust with nickel-postgres](http://blog.bguiz.com/2014/08/05/restful-api-in-rust-with-nickel-postgres/)
 * [Writing Postgres extensions in Rust](https://github.com/thehydroimpulse/postgres-extension.rs)
