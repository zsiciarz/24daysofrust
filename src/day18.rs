extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;

type UserId = u64;

fn add_friend(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<()> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    let _: () = try!(conn.sadd(my_key, their_id));
    let _: () = try!(conn.sadd(their_key, my_id));
    Ok(())
}

fn friends_in_common(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<HashSet<UserId>> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    let result: HashSet<UserId> = try!(conn.sinter((my_key, their_key)));
    Ok(result)
}

fn main() {
    println!("24 days of Rust - redis (day 18)");
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let _: () = conn.set("answer", 42i).unwrap();
    let answer: int = conn.get("answer").unwrap();
    println!("Answer: {}", answer);

    for i in range(1, 10u64) {
        add_friend(&conn, i, i + 2).ok().expect("Friendship failed :(");
    }
    println!("{}", friends_in_common(&conn, 2, 3));
}
