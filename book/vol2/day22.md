# Day 22 - lettre

[`lettre`](https://crates.io/crates/lettre) is a library to send emails over
SMTP from our Rust applications. Like many other crates in the growing,
but still young Rust ecosystem, it is still a work in progress. But while
there are a few features missing from `lettre`, we can send some emails
right now!

Just send the email already
---------------------------

Let's start with something simple: send a plain *Hello Rust* email. Here's
our first approach with `lettre`:

```rust
extern crate lettre;

use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;

fn main() {
    let email = EmailBuilder::new()
        .from("zbigniew@siciarz.net")
        .to("zbigniew@siciarz.net")
        .subject("Hello Rust!")
        .body("Hello Rust!")
        .build()
        .expect("Failed to build message");
    let mut transport = smtp::SmtpTransportBuilder::localhost()
        .expect("Failed to create transport")
        .build();
    println!("{:?}", transport.send(email.clone()));
}
```

The `EmailBuilder` exposes a fluent API to create an email message. Apart
from the methods shown above, we can set additional headers, CC and reply
addresses, or an HTML body. Finally the `build()` method ends the chain
and returns a `Result<Email, Error>`. So what happens if we run this example?

```text
$ cargo run
Err(Io(Error { repr: Os { code: 111, message: "Connection refused" } }))
```

Oops... But notice that we used `localhost()` in the transport configuration.
More often than not we won't have an SMTP server installed and running on
our local machine. Let's use GMail as our email backend instead:

```rust
use std::env;

let mut transport = smtp::SmtpTransportBuilder::new(("smtp.gmail.com", smtp::SUBMISSION_PORT))
    .expect("Failed to create transport")
    .credentials(&env::var("GMAIL_USERNAME").unwrap_or("user".to_string())[..],
                    &env::var("GMAIL_PASSWORD").unwrap_or("password".to_string())[..])
    .build();
println!("{:?}", transport.send(email.clone()));
```

We're reading GMail credentials from environment variables in order not to
hardcode them in our program. If all goes well, after running this code
we should receive an email!

The `SendableEmail` trait
-------------------------

We don't need to use `EmailBuilder` every time we want to send an email.
The `send()` method of the transport requires its argument to implement
`SendableEmail`, nothing else. For example, if we have a custom reporting
system that already has a notion of a report's recipient, we can implement
`SendableEmail` for the `Report` type and send that instead.

```rust
extern crate uuid;

use lettre::email::SendableEmail;

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

let report = Report {
    contents: "Some very important report".to_string(),
    recipient: "zbigniew@siciarz.net".to_string(),
};
transport.send(report).expect("Failed to send the report");
```

Frankly speaking, this trait doesn't feel like it has a complete API yet.
Most notably, there's no way to set the subject. I hope it's just an oversight
and a relevant method will be added to the trait in some future version of
`lettre`.

There are a few more missing things in this crate.
At the moment, the most important feature that's not implemented is the
ability to attach files to emails. However, this is being worked on. Also,
a [Mailgun backend](https://github.com/lettre/lettre/issues/108) is already
in progress.

Remember that `lettre` is still at a 0.x version number, but it's gaining
traction. And hopefully gaining more contributors as well :)

Further reading
---------------

 - [`mailstrom`](https://crates.io/crates/mailstrom) - a crate using `lettre`
   for mass email delivery
 - [`sendgrid`](https://crates.io/crates/sendgrid) a Rust client for SendGrid API
