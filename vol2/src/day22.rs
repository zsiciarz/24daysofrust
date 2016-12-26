extern crate dotenv;
extern crate lettre;
extern crate uuid;

use std::env;
use lettre::email::{EmailBuilder, SendableEmail};
use lettre::transport::EmailTransport;
use lettre::transport::smtp;

struct Report {
    contents: String,
    recipient: String,
}

impl SendableEmail for Report {
    fn from_address(&self) -> String {
        "complicated.report.system@gmail.com".to_string()
    }

    fn to_addresses(&self) -> Vec<String> {
        vec![self.recipient.clone()]
    }

    fn message(&self) -> String {
        format!("\nHere's the report you asked for:\n\n{}", self.contents)
    }

    fn message_id(&self) -> String {
        uuid::Uuid::new_v4().simple().to_string()
    }
}

fn main() {
    println!("24 Days of Rust vol. 2 - lettre");
    dotenv::dotenv().ok();
    let email = EmailBuilder::new()
        .from("zbigniew.siciarz@goldenline.pl")
        .to("zbigniew.siciarz@goldenline.pl")
        .subject("Hello Rust!")
        .body("Hello Rust!")
        .build()
        .expect("Failed to build email message");
    println!("{:?}", email);

    let mut transport = smtp::SmtpTransportBuilder::localhost()
        .expect("Failed to create transport")
        .build();
    println!("{:?}", transport.send(email.clone()));

    let mut transport = smtp::SmtpTransportBuilder::new(("smtp.gmail.com", smtp::SUBMISSION_PORT))
        .expect("Failed to create transport")
        .credentials(&env::var("GMAIL_USERNAME").unwrap_or("user".to_string())[..],
                     &env::var("GMAIL_PASSWORD").unwrap_or("password".to_string())[..])
        .build();
    println!("{:?}", transport.send(email));

    let report = Report {
        contents: "Some very important report".to_string(),
        // recipient: "zbigniew@siciarz.net".to_string(),
        recipient: "zbigniew.siciarz@goldenline.pl".to_string(),
    };
    println!("{:?}", transport.send(report));
}
