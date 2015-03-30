#![feature(plugin)]
#![plugin(postgres_macros)]

extern crate rustc_serialize;
extern crate postgres;
extern crate postgres_array;

use postgres::{Connection, Error, FromSql, SslMode};
use postgres::Result as PgResult;
use postgres_array::ArrayBase;

use rustc_serialize::json::Json;

fn get_single_value<T>(conn: &Connection, query: &str) -> PgResult<T>
    where T: FromSql {
    println!("Executing query: {}", query);
    let stmt = try!(conn.prepare(query));
    let rows = try!(stmt.query(&[]));
    let row = try!(rows.iter().next().ok_or(Error::BadResponse));
    row.get_opt(0)
}

fn main() {
    println!("24 days of Rust - postgres (day 11)");
    let dsn = "postgresql://rust:rust@localhost/rust";
    let conn = match Connection::connect(dsn, &SslMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Connection error: {:?}", e);
            return;
        }
    };
    conn.execute("create table if not exists blog (
        id serial primary key,
        title varchar(255),
        body text)", &[]).ok().expect("Table creation failed");
    let stmt = match conn.prepare("insert into blog (title, body) values ($1, $2)") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {:?}", e);
            return;
        }
    };
    for i in 1..5 {
        let title = format!("Blogpost number {}", i);
        let text = format!("Content of the blogpost #{}", i);
        stmt.execute(&[&title, &text]).ok().expect("Inserting blogposts failed");
    }
    let stmt = match conn.prepare("select id, title, body from blog where id < $1") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {:?}", e);
            return;
        }
    };
    let max_id: i32 = 3;
    let rows = stmt.query(&[&max_id]).ok().expect("Selecting blogposts failed");
    for row in rows {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        println!("ID={}, title={}", id, title);
    }
    println!("{:?}", get_single_value::<bool>(&conn, "select 1=1"));
    println!("{:?}", get_single_value::<i32>(&conn, "select 1=1"));

    type IntArray = ArrayBase<Option<i32>>;
    let arr = get_single_value::<IntArray>(&conn, "select '{4, 5, 6}'::int[]");
    println!("{:?}", arr.map(|arr| arr.values()
            .filter_map(|x| *x) // unwraps Some values and skips None
            .collect::<Vec<_>>()));

    let json = get_single_value::<Json>(&conn, "select '{\"foo\": \"bar\", \"answer\": 42}'::json");
    println!("{:?}", json);

    let query = sql!("select '{4, 5, 6}'::int[]");
    println!("{:?}", query);
}
