extern crate rand;
extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;

type UserId = u64;

fn add_friend(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<()> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    let _: () = conn.sadd(my_key, their_id)?;
    let _: () = conn.sadd(their_key, my_id)?;
    Ok(())
}

fn friends_in_common(
    conn: &Connection,
    my_id: UserId,
    their_id: UserId,
) -> RedisResult<HashSet<UserId>> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    conn.sinter((my_key, their_key))
}

fn add_score(conn: &Connection, username: &str, score: u32) -> RedisResult<()> {
    conn.zadd("leaderboard", username, score)
}

type Leaderboard = Vec<(String, u32)>;

fn show_leaderboard(conn: &Connection, n: isize) {
    let result: RedisResult<Leaderboard> = conn.zrevrange_withscores("leaderboard", 0, n - 1);
    match result {
        Ok(board) => {
            println!("----==== Top {} players ====----", n);
            for (i, (username, score)) in board.into_iter().enumerate() {
                println!("{:<5} {:^20} {:>4}", i + 1, username, score);
            }
        }
        Err(_) => println!("Failed to fetch leaderboard."),
    }
}

fn main() {
    println!("24 days of Rust - redis (day 18)");
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    let _: () = conn.set("answer", 42).unwrap();
    let answer: i32 = conn.get("answer").unwrap();
    println!("Answer: {}", answer);

    for i in 1..10u64 {
        add_friend(&conn, i, i + 2).expect("Friendship failed :(");
    }
    println!(
        "You have {} friends in common.",
        friends_in_common(&conn, 2, 3).map(|s| s.len()).unwrap_or(0)
    );

    let players = vec!["raynor", "kerrigan", "mengsk", "zasz", "tassadar"];
    for player in &players {
        let score = rand::random::<u32>() % 1000;
        add_score(&conn, *player, score).expect("Nuclear launch detected");
    }
    show_leaderboard(&conn, 3);
}
