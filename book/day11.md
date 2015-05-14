# Day 11 - postgres

Yes, I'm biased. [PostgreSQL](http://www.postgresql.org/) is my favorite SQL database and the [upcoming 9.4 release](http://www.postgresql.org/message-id/26295.1417708564@sss.pgh.pa.us?utm_source=postgresweekly&amp;utm_medium=email) makes me even more excited. There is already a pure Rust driver for PostgreSQL - the [postgres](https://crates.io/crates/postgres) crate which will be the subject of today's article.

Connecting
----------

In this and the following examples we will assume a Postgres user `rust` with password `rust` and an existing database named... well, `rust`.

    :::rust
    extern crate postgres;

    use postgres::{Connection, SslMode};

    fn main() {
        let dsn = "postgresql://rust:rust@localhost/rust";
        let conn = match Connection::connect(dsn, &amp;SslMode::None) {
            Ok(conn) => conn,
            Err(e) => {
                println!("Connection error: {}", e);
                return;
            }
        };
    }

The `Connection` type has a few methods related to making queries; perhaps the simplest one is `execute()` which immediately executes the query and returns the number of modified rows (or an error). This method can be used for example for insert/update queries but also DDL as shown below.

    :::rust
    conn.execute("create table if not exists blog (
        id serial primary key,
        title varchar(255),
        body text)", &amp;[]).ok().expect("Table creation failed");

The second argument looks slightly awkward, but that's the way of telling `execute()` that the query takes no parameters. Later on we will see a few examples that use query parameters.

Prepared queries and statements
-------------------------------

Let's add a few rows to our table. We will use the `prepare()` method.

    :::rust
    let stmt = match conn.prepare("insert into blog (title, body) values ($1, $2)") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {}", e);
            return;
        }
    };
    for i in range(1, 5u) {
        let title = format!("Blogpost number {}", i);
        let text = format!("Content of the blogpost #{}", i);
        stmt.execute(&amp;[&amp;title, &amp;text]).ok().expect("Inserting blogposts failed");
    }

The query was prepared only once and what we got (after some crude error handling) is a `Statement` value. We can use its `execute()` method to actually run the query with the supplied parameters (note the borrowing).

To read the data from the database we will use the same `prepare()` method in conjunction with the `query()` method of a `Statement`. There's a significant difference between `execute()` and `query()`: the former returns just the number of affected rows, while the latter returns a collection of `Row` values.

    :::rust
    let stmt = match conn.prepare("select id, title, body from blog where id < $1") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {}", e);
            return;
        }
    };
    let max_id: i32 = 3;
    let mut rows = stmt.query(&amp;[&amp;max_id]).ok().expect("Selecting blogposts failed");
    for row in rows {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        println!("ID={}, title={}", id, title);
    }

Keep in mind that the `get()` method will panic if it encounters incompatible types, for example if we changed `String` to `i32` above. There's also a safer `get_opt()` method returning a `Result` instead of panicking.

Advanced PostgreSQL types
-------------------------

All this is a bit boring so far. One of the reasons developers love PostgreSQL is its selection of interesting data types. Let's see how to use them in Rust (hint: it's kinda cool). We'll start from writing a generic helper function to read a single value from the first column of the first row.

    :::rust
    use postgres::{Connection, Error, FromSql, SslMode};
    use postgres::Result as PgResult;

    fn get_single_value<T>(conn: &amp;Connection, query: &amp;str) -> PgResult<T>
        where T: FromSql {
        println!("Executing query: {}", query);
        let stmt = try!(conn.prepare(query));
        let mut rows = try!(stmt.query(&amp;[]));
        let row = try!(rows.next().ok_or(Error::BadData));
        row.get_opt(0)
    }

We use the `try!` macro to minimize the noise from error handling. Now let's see it in action.

    :::rust
    println!("{}", get_single_value::<bool>(&amp;conn, "select 1=1"));
    println!("{}", get_single_value::<i32>(&amp;conn, "select 1=1"));
    type IntArray = ArrayBase<Option<i32>>;
    let arr = get_single_value::<IntArray>(&amp;conn, "select '{4, 5, 6}'::int[]");
    println!("{}", arr.map(|arr| arr.values()
            .filter_map(|x| *x) // unwraps Some values and skips None
            .collect::<Vec<_>>()));
    let json = get_single_value::<Json>(&amp;conn, "select '{\"foo\": \"bar\", \"answer\": 42}'::json");
    println!("{}", json);

<!-- -->

    :::sh
    $ cargo run
    Executing query: select 1=1
    Ok(true)
    Executing query: select 1=1
    Err(Unexpected type Bool)
    Executing query: select '{4, 5, 6}'::int[]
    Ok([4, 5, 6])
    Executing query: select '{"foo": "bar", "answer": 42}'::json
    Ok({"answer":42,"foo":"bar"})

Fantastic! The error handling is still there when we need it and we get values of reasonable Rust types.

Compile-time SQL checking
-------------------------

There is another crate worth mentioning here - [postgres_macros](https://crates.io/crates/postgres_macros). It provides the `sql!` macro that validates correctness of the SQL query given as argument. See the [readme](https://github.com/sfackler/rust-postgres-macros) for an example. Unfortunately I couldn't get it to install, but it looks like an interesting experiment.

See also
--------

 * [nickel-potgres](https://github.com/nickel-org/nickel-postgres) - a Postgres middleware for the [nickel.rs](http://nickel.rs/) web framework
 * [Building RESTful API server in Rust with nickel-postgres](http://blog.bguiz.com/2014/08/05/restful-api-in-rust-with-nickel-postgres/)
 * [Writing Postgres extensions in Rust](https://github.com/thehydroimpulse/postgres-extension.rs)

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly and postgres 0.1.3.
</small>
