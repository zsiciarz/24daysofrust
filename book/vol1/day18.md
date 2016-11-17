# Day 18 - redis

> Relevancy: 1.9 stable

Today I'm revisiting the topic of database access in Rust. I mentioned PostgreSQL client library [a week ago](https://siciarz.net/24-days-of-rust-postgres/). This time we'll move from SQL to NoSQL land. Our focus for today will be [Redis](http://redis.io/) - a data structure server. The [redis crate](https://crates.io/crates/redis) is a client library to access Redis from Rust.

Connecting
----------

The `redis` crate provides a `Client` type that is used to connect to the Redis server. The `get_connection()` method returns a `Connection` object which execute Redis commands.

[include:2-2](../../src/day18.rs)
[include:4-4](../../src/day18.rs)
[include:44-48](../../src/day18.rs)

Most of the [Redis commands](http://redis.io/commands) translate directly to `Connection` methods. But if you encounter an error similar to `Type ``redis::connection::Connection`` does not implement any method in scope named ``set`` `, you probably forgot to import the `Commands` trait.

(I accidentally used the same example number as Armin did in the readme; not surprising since it is the [Answer to the Ultimate Question of Life, the Universe, and Everything](http://en.wikipedia.org/wiki/Phrases_from_The_Hitchhiker%27s_Guide_to_the_Galaxy#Answer_to_the_Ultimate_Question_of_Life.2C_the_Universe.2C_and_Everything_.2842.29).)

Friends in common
-----------------

When viewing someone's profile page on most of the social networking sites, you can see the number (or even a full list) of friends that you both have in common. This is very easy to achieve in Redis using sets.

In case someone accepts your friendship request, a function similar to the one below will be called.

[include:5-5](../../src/day18.rs)
[include:7-15](../../src/day18.rs)

I'm assuming here that the friendship relation is mutual. That's why there are two `sadd` calls - one to add yourself to their set of friends and the other one is symmetrical. Now checking friends in common is just a matter of set intersection - expressed in Redis as the `SINTER` command.

[include:17-21](../../src/day18.rs)

We can now simulate adding a few friends:

[include:50-54](../../src/day18.rs)

Here's the output:

```sh
$ cargo run
You have 1 friend(s) in common.
```

Leaderboards
------------

[Sorted sets](http://redis.io/commands#sorted_set) are possibly my favorite Redis data structure. They're a perfect fit to create leaderboards for example in online games. Add scores with `ZADD`, fetch the leaderboard with `ZREVRANGE` - that's the gist of it.

[include:23-25](../../src/day18.rs)

The `add_score` function is just a wrapper to provide a more high-level API. It will be called every time player's score changes.

[include:27-40](../../src/day18.rs)

The `Leaderboard` alias is there just to simplify the result type. We use `zrevrange_withscores` to get the leaderboard data (sorted by score descending) and display it using Rust's [string formatting](http://doc.rust-lang.org/std/fmt/) syntax.

Putting all this together:

[include:56-61](../../src/day18.rs)

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
