extern crate postgres;

use postgres::{Connection, SslMode};

fn main() {
    println!("24 days of Rust - postgres (day 11)");
    let dsn = "postgresql://rust:rust@localhost/rust";
    let conn = match Connection::connect(dsn, &SslMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Connection error: {}", e);
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
            println!("Preparing query failed: {}", e);
            return;
        }
    };
    for i in range(1, 5u) {
        let title = format!("Blogpost number {}", i);
        let text = format!("Content of the blogpost #{}", i);
        stmt.execute(&[&title, &text]).ok().expect("Inserting blogposts failed");
    }
    let stmt = match conn.prepare("select id, title, body from blog where id < $1") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Preparing query failed: {}", e);
            return;
        }
    };
    let max_id: i32 = 3;
    let mut rows = stmt.query(&[&max_id]).ok().expect("Selecting blogposts failed");
    for row in rows {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        println!("ID={}, title={}", id, title);
    }
}
