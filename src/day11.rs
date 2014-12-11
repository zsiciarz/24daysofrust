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
}
