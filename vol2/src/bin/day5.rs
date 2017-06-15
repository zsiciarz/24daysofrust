extern crate dotenv;
// extern crate envy;

#[macro_use]
extern crate serde_derive;

use std::env;

#[derive(Deserialize, Debug)]
struct Environment {
    lang: String,
}

#[derive(Deserialize, Debug)]
struct MailerConfig {
    email_backend: String,
    email_from: String,
}

fn main() {
    println!("24 Days of Rust, volume 2 - environment");
    match env::var("LANG") {
        Ok(lang) => println!("Language code: {}", lang),
        Err(e) => println!("Couldn't read LANG ({})", e),
    };

    // match envy::from_env::<Environment>() {
    //     Ok(environment) => println!("Language code: {}", environment.lang),
    //     Err(e) => println!("Couldn't read LANG ({})", e),
    // };

    dotenv::dotenv().expect("Failed to read .env file");
    println!(
        "Email backend: {}",
        env::var("EMAIL_BACKEND").expect("EMAIL_BACKEND not found")
    );

    // match envy::from_env::<MailerConfig>() {
    //     Ok(config) => println!("{:?}", config),
    //     Err(e) => println!("Couldn't read mailer config ({})", e),
    // };
}
