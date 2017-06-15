#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate time;

use std::fs::File;
use slog::Drain;

#[allow(dead_code)]
struct User {
    username: String,
    logger: slog::Logger,
}

type Waldo = String;
type DatabaseError = String;

impl User {
    fn new(username: &str, logger: &slog::Logger) -> Self {
        User {
            username: username.to_string(),
            logger: logger.new(o!("username" => username.to_string())),
        }
    }

    fn sign_in(&self) {
        info!(self.logger, "User signed in");
    }

    fn find_waldo(&self) -> Option<Waldo> {
        match read_database(&self.username) {
            Ok(waldo) => {
                info!(self.logger, "Found Waldo");
                Some(waldo)
            }
            Err(error) => {
                error!(self.logger, "Failed to find Waldo"; "error" => error);
                None
            }
        }
    }
}

fn read_database(username: &str) -> Result<String, DatabaseError> {
    Err(format!("{}: Not this time", username))
}

fn main() {
    println!("24 Days of Rust vol. 2 - slog");
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stderr());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => "0.5"));
    info!(root_logger, "Application started";
        "started_at" => format!("{}", time::now().rfc3339()));
    let user = User::new("zbyszek", &root_logger);
    user.sign_in();
    let _ = user.find_waldo();

    let console_drain = slog_term::FullFormat::new(
        slog_term::PlainSyncDecorator::new(std::io::stdout()),
    ).build()
        .fuse();
    let file = File::create("app.log").expect("Couldn't open log file");
    let file_drain = slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(file))
        .build()
        .fuse();
    let logger = slog::Logger::root(slog::Duplicate::new(console_drain, file_drain).fuse(), o!());
    warn!(logger, "not_enough_resources"; "resource" => "cat pictures");
}
