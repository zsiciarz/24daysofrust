# Day 18 - redis

> Relevancy: 1.1 stable

Today I'm revisiting the topic of database access in Rust. I mentioned PostgreSQL client library [a week ago](https://siciarz.net/24-days-of-rust-postgres/). This time we'll move from SQL to NoSQL land. Our focus for today will be [Redis](http://redis.io/) - a data structure server. The [redis crate](https://crates.io/crates/redis) is a client library to access Redis from Rust.

Connecting
----------

The `redis` crate provides a `Client` type that is used to connect to the Redis server. The `get_connection()` method returns a `Connection` object which execute Redis commands.

```rust
extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};

fn main() {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let _: () = conn.set("answer", 42).unwrap();
    let answer: i32 = conn.get("answer").unwrap();
    println!("Answer: {}", answer);
}
```

Most of the [Redis commands](http://redis.io/commands) translate directly to `Connection` methods. But if you encounter an error similar to `Type ``redis::connection::Connection`` does not implement any method in scope named ``set`` `, you probably forgot to import the `Commands` trait.

(I accidentally used the same example number as Armin did in the readme; not surprising since it is the [Answer to the Ultimate Question of Life, the Universe, and Everything](http://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Answer_to_the_Ultimate_Question_of_Life.2C_the_Universe.2C_and_Everything_.2842.29).)

Friends in common
-----------------

When viewing someone's profile page on most of the social networking sites, you can see the number (or even a full list) of friends that you both have in common. This is very easy to achieve in Redis using sets.

In case someone accepts your friendship request, a function similar to the one below will be called.

```
extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;

type UserId = u64;

fn add_friend(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<()> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    try!(conn.sadd(my_key, their_id));
    try!(conn.sadd(their_key, my_id));
    Ok(())
}
```

I'm assuming here that the friendship relation is mutual. That's why there are two `sadd` calls - one to add yourself to their set of friends and the other one is symmetrical. Now checking friends in common is just a matter of set intersection - expressed in Redis as the `SINTER` command.

```rust
fn friends_in_common(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<HashSet<UserId>> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    Ok(try!(conn.sinter((my_key, their_key))))
}
```

We can now simulate adding a few friends:

```rust
for i in 1..10u64 {
    add_friend(&conn, i, i + 2).ok().expect("Friendship failed :(");
}
println!("You have {} friends in common.",
         friends_in_common(&conn, 2, 3).map(|s| s.len()).unwrap_or(0));
```

Here's the output:

```sh
$ cargo run
You have 1 friend(s) in common.
```

Leaderboards
------------

[Sorted sets](http://redis.io/commands#sorted_set) are possibly my favorite Redis data structure. They're a perfect fit to create leaderboards for example in online games. Add scores with `ZADD`, fetch the leaderboard with `ZREVRANGE` - that's the gist of it.

```rust
fn add_score(conn: &Connection, username: &str, score: u32) -> RedisResult<()> {
    conn.zadd("leaderboard", username, score)
}
```

The `add_score` function is just a wrapper to provide a more high-level API. It will be called every time player's score changes.

```rust
type Leaderboard = Vec<(String, u32)>;

fn show_leaderboard(conn: &Connection, n: isize) {
    let result: RedisResult<Leaderboard> = conn.zrevrange_withscores("leaderboard", 0, n - 1);
    match result {
        Ok(board) => {
            println!("----==== Top {} players ====----", n);
            for (i, (username, score)) in board.into_iter().enumerate() {
                println!("{:<5} {:^20} {:>4}", i + 1, username, score);
            }
        },
        Err(_) => println!("Failed to fetch leaderboard."),
    }
}
```

The `Leaderboard` alias is there just to simplify the result type. We use `zrevrange_withscores` to get the leaderboard data (sorted by score descending) and display it using Rust's [string formatting](http://doc.rust-lang.org/std/fmt/) syntax.

Putting all this together:

```rust
let players = vec!["raynor", "kerrigan", "mengsk", "zasz", "tassadar"];
for player in players.iter() {
    let score = rand::random::<u32>() % 1000;
    add_score(&conn, *player, score).ok().expect("Nuclear launch detected");
}
show_leaderboard(&conn, 3);
```

And if we run this, we'll get something similar to the output below:

```sh
$ cargo run
----==== Top 3 players ====----
1            mengsk         986
2           tassadar        879
3           kerrigan        489
```

See also
--------

 * [User friendships](http://jimneath.org/2011/03/24/using-redis-with-ruby-on-rails.html#example_uses_in_rails) with Redis and Ruby on Rails
 * [The Top 3 Game Changing Redis Use Cases](https://redislabs.com/blog/the-top-3-game-changing-redis-use-cases)
 * [Creating high score tables (leaderboards) using Redis](http://www.agoragames.com/blog/2011/01/01/creating-high-score-tables-leaderboards-using-redis/)
