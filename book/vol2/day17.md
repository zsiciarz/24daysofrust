# Day 17 - diesel

[`diesel`](http://diesel.rs/) is an in-development ORM (Object-Relational
Mapper) for Rust. It aims to be a safe and efficient layer between your
business logic and the database. In the words of its authors:

> Diesel gets rid of the boilerplate for database interaction and eliminates
> runtime errors, without sacrificing performance. It takes full advantage
> of Rust's type system to create a low overhead query builder that
> "feels like Rust".

The primary author of `diesel` - [Sean Griffin](https://twitter.com/sgrif) -
is also a Ruby on Rails committer and maintainer of Active Record, the ORM used
in Rails. So when I saw his name in relation to a Rust ORM, I knew I have
to check it out. Sean also gave a great talk at
[PolyConf 16](http://polyconf.com/) about ownership semantics (not only in
Rust) - [Owning Ownership](https://www.youtube.com/watch?v=PBV6j2BhEGE).

Diesel as a photo gallery backend
---------------------------------

For this example we're going to use the ORM as if we were building a photo
gallery app. Doesn't matter if it's a web or desktop app - the focus here is
only on the database interaction.

**Note**: I'm not going to describe the basics of `diesel` and how to get up
 and running. The
 [geting started guide](http://diesel.rs/guides/getting-started/) covers
the initial setup. I'm also using a nightly Rust compiler, but the essential
custom derive machinery should land in stable, hopefully soon (1.15 perhaps).

Let's see the model for our app:

```rust
#![feature(custom_attribute, proc_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

use schema::*;

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[has_many(photos)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
#[belongs_to(User)]
pub struct Photo {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
}
```

There are a few traits that we tell `diesel` to derive for us:

 - `Queryable` allows querying with `load()`, `first()` etc.
 - `Identifiable` marks structs which correspond to a single table and have an
   ID column
 - `Associations` allows specifying relations between tables and using them for
   `JOIN`s etc.
 - `AsChangeset` for things that can be updated

Here's the corresponding schema definition and some initial data:

```sql
create table users (
    id serial primary key,
    username varchar(255) not null,
    avatar varchar(255) null
);
insert into users (username) values ('zbyszek');

create table photos (
    id serial primary key,
    user_id integer not null references users (id),
    url varchar(255) not null
);
insert into photos (user_id, url) values (1, 'http://lorempixel.com/output/cats-q-c-640-480-10.jpg');
```

Note: I'm using PostgreSQL as a database backend here. At the moment `diesel`
supports SQLite as well, but some of the later examples in this article
are Postgres-specific.

Selects and relationships
-------------------------

We'll start with a simple select that fetches all photos by a single user.

```rust
let me = users::table.find(1).first::<User>(&conn).expect("Error loading user");
println!("{:?}", me);
let my_photos = Photo::belonging_to(&me)
    .load::<Photo>(&conn)
    .expect("Error loading photos");
println!("{:?}", my_photos);
```

```text
$ cargo run
User { id: 1, username: "zbyszek", avatar: None }
User zbyszek has 1 photo(s)
[Photo { id: 1, user_id: 1, url: "http://lorempixel.com/output/cats-q-c-640-480-10.jpg" }]
```

We're loading the user with ID=1 (`find()` searches by primary key) and
afterwards fetch all photos associated with this user. The `belonging_to()`
method takes a reference to the type declared in the `belongs_to`
attribute. This is an example of a 1:N relationship - a `User` can have
multiple `Photo`s. The Rust code above issues SQL queries similar to these:

```sql
select * from users where id=1 limit 1;
select * from photos where user_id=1;
```

If we're interested in the actual SQL statement, we can use the `print_sql!`
macro:

```rust
print_sql!(Photo::belonging_to(&me).count());
```

This would output `SELECT COUNT(*) FROM ``photos`` WHERE ``photos``.``user_id`` = ?`.
This macro works only on `QueryFragment`s and not on every statement that
hits the database, but there is work underway to have logging support in
connections or query builders.

If we wanted to do an `INNER JOIN` to attach a user to every photo, we can
use `inner_join()` like this:

```rust
let photos: Vec<(Photo, User)> =
    photos::table.inner_join(users::table).load(&conn).expect("Error loading photos");
for (photo, user) in photos {
    println!("Photo #{} by {}", photo.id, user.username);
}
```

This time the `load()` method yields tuples of structs. The order of types
in the tuple is the same as that of respective tables in the query. There's
also a `left_outer_join()` method when one end of the relation may be `NULL`.

We can also filter on fields of the joined table, as well as group the results:

```rust
let users_with_cat_photos: Vec<String> = users::table.select(users::username)
    .inner_join(photos::table)
    .filter(photos::url.like("%cat%"))
    .group_by(users::id)
    .load(&conn)
    .expect("Error loading users");
println!("{:?}", users_with_cat_photos);
```

This time we have only one column in the `SELECT` part of the query,
thanks to the `select()` method.

```text
$ cargo run
["zbyszek"]
```

Inserting and deleting
----------------------

```rust
#[derive(Debug, Insertable)]
#[table_name="photos"]
pub struct NewPhoto {
    pub user_id: i32,
    pub url: String,
}

impl User {
    fn new_photo(&self, url: &str) -> NewPhoto {
        NewPhoto {
            user_id: self.id,
            url: url.to_string(),
        }
    }
}
```

We've added a custom method to the `User` struct that prepares a new photo
to be inserted into the database. The `NewPhoto` struct implements the
`Insertable` trait so that `diesel` knows what to do in the following code:

```rust
let photo = me.new_photo("http://lorempixel.com/output/cats-q-c-640-480-8.jpg");
let mut inserted_photo = diesel::insert(&photo)
    .into(photos::table)
    .get_result::<Photo>(&conn)
    .expect("Failed to insert photo");
```

We've already seen a few methods that actually execute the query using a
database connection. `load()` returns an `Iterator` over the results,
while `first()` or `get_result()` return a single object. There's also
`execute()` which returns how many rows were affected by the query, as in
the example below:

```rust
let deleted_count = diesel::delete(photos::table.filter(photos::id.gt(1)))
    .execute(&conn)
    .expect("Failed to clean up photos");
println!("Deleted {} photo(s)", deleted_count);
```

We're using `filter` here to delete all photos *except* the first one.

Postgres arrays
---------------

Let's add tags to our photos! PostgreSQL has a data type that's perfect for
this purpose - arrays (of text). Since migrations in `diesel` are just
SQL code, let's add the column first.

```sql
alter table photos add column tags text[] not null default '{}';
```

We can run `diesel migration run` and a new column will be added to the
table. However, at this moment our program fails to compile:

```text
  --> src/example.rs:87:10
   |
87 |         .load::<Photo>(&conn)
   |          ^^^^ the trait `diesel::types::FromSqlRow<(diesel::types::Integer, diesel::types::Integer, diesel::types::Text, diesel::types::Array<diesel::types::Text>), _>` is not implemented for `(i32, i32, std::string::String)`
```

`diesel` inferred from the database that the `Photo` struct should have one
more field. So let's define it (I've omitted the attributes here):

```rust
pub struct Photo {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub tags: Vec<String>,
}
```

Thanks to a clever
[`FromSql` implementation](https://github.com/diesel-rs/diesel/blob/9ea449c480739253766bd097e7b06d038fe16590/diesel/src/pg/types/array.rs#L34),
Postgres arrays are transparently mapped to `Vec`s.

```rust
inserted_photo.tags = vec!["cute".to_string(), "kitten".to_string()];
let updated_photo: Photo = inserted_photo.save_changes(&conn).expect("Error updating photo");
println!("{:?}", updated_photo);
```

```text
$ cargo run
Photo { id: 27, user_id: 1, url: "http://lorempixel.com/output/cats-q-c-640-480-8.jpg", tags: ["cute", "kitten"] }
```

`diesel` even supports
[array lookups](https://www.postgresql.org/docs/9.1/static/functions-array.html)
such as `@>` to filter photos by tags. See the example below:

```rust
let cute_cat_count: i64 = photos::table.filter(photos::tags.contains(vec!["cute", "kitten"]))
    .count()
    .get_result(&conn)
    .expect("Error counting cute kittens");
println!("There's {} photos of cute cats", cute_cat_count);
```

```text
$ cargo run
There's 2 photos of cute cats
```

There's also an
[open issue](https://github.com/diesel-rs/diesel/issues/44) for JSON support
([this comment](https://github.com/diesel-rs/diesel/issues/44#issuecomment-218325285)
from Sean gives the basic idea how it could be done).

If you need to execute some custom SQL query that you  *really* couldn't
express with `diesel`, there's a last resort solution. The `sql()` function
can execute raw SQL queries. However it should be used only when absolutely
necessary as it provides no guarantees about safety and correctness of the
query.

For the record, here's the above cute cat counter implemented with `sql()`
before [Sean pointed out](https://twitter.com/sgrif/status/810365851903885312)
that `diesel` in fact supports Postgres array lookups.

```rust
use diesel::expression::sql_literal::sql;

let cute_cat_count: i64 = sql("select count(*) from photos \
                               where tags @> array['cute', 'kitten']")
    .get_result(&conn)
    .expect("Error executing raw SQL");
```

```text
$ cargo run
There's 2 photos of cute cats
```

Further reading
---------------

 - [Getting started with Diesel](http://diesel.rs/guides/getting-started/)
 - [Associations](http://docs.diesel.rs/diesel/associations/index.html)
 - [Comparing diesel and rust-postgres](https://hackernoon.com/comparing-diesel-and-rust-postgres-97fd8c656fdd#.kr5agrsji)
 - [`nickel-diesel`](https://crates.io/crates/nickel-diesel) - a diesel middleware for the Nickel web framework
 - [`rustorm`](https://crates.io/crates/rustorm) - another ORM for Rust
