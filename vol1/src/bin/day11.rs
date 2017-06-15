// #![feature(plugin)]
// #![cfg_attr(target_family="unix", plugin(postgres_macros))]


extern crate rustc_serialize;
extern crate time;
extern crate postgres;
extern crate postgres_array;
// extern crate postgres_range;

use postgres::{Connection, TlsMode};
use postgres::types::FromSql;
use postgres::Result as PgResult;
use postgres_array::Array;
// use postgres_range::Range;

use rustc_serialize::json::Json;

use time::Timespec;

fn get_single_value<T>(conn: &Connection, query: &str) -> PgResult<T>
where
    T: FromSql,
{
    println!("Executing query: {}", query);
    let stmt = conn.prepare(query)?;
    let rows = stmt.query(&[])?;
    let row = rows.iter().next().unwrap();
    row.get_opt(0).unwrap()
}

#[cfg(target_family = "unix")]
fn sql_macro() {
    //let query = sql!("select '{4, 5, 6}'::int[]");
    //println!("{:?}", query);
}

#[cfg(not(target_family = "unix"))]
fn sql_macro() {
    println!("TODO");
}

fn main() {
    println!("24 days of Rust - postgres (day 11)");
    let dsn = "postgresql://rust:rust@localhost/rust";
    let conn = match Connection::connect(dsn, TlsMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Connection error: {:?}", e);
            return;
        }
    };
    conn.execute(
        "create table if not exists blog (
        id serial primary key,
        title \
                  varchar(255),
        body text)",
        &[],
    ).expect("Table creation failed");
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
        stmt.execute(&[&title, &text]).expect(
            "Inserting blogposts failed",
        );
    }
    let stmt = match conn.prepare("select id, title, body from blog where id < $1") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {:?}", e);
            return;
        }
    };
    let max_id: i32 = 3;
    let rows = stmt.query(&[&max_id]).expect("Selecting blogposts failed");
    for row in rows.iter() {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        println!("ID={}, title={}", id, title);
    }
    println!("{:?}", get_single_value::<bool>(&conn, "select 1=1"));
    println!("{:?}", get_single_value::<i32>(&conn, "select 1=1"));

    type IntArray = Array<Option<i32>>;
    let arr = get_single_value::<IntArray>(&conn, "select '{4, 5, 6}'::int[]");
    println!(
        "{:?}",
        arr.map(|arr| arr.iter().filter_map(|x| *x).collect::<Vec<_>>())
    );

    let json = get_single_value::<Json>(&conn, "select '{\"foo\": \"bar\", \"answer\": 42}'::json");
    println!("{:?}", json);

    // let range = get_single_value::<Range<i32>>(&conn, "select '[10, 20)'::int4range");
    // println!("{:?}", range);
    // let ts_range =
    // get_single_value::<Range<Timespec>>(&conn, "select '[2015-01-01, 2015-12-31]'::tsrange");
    // println!("{:?}", ts_range);

    sql_macro();
}
