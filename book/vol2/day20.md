# Day 20 - reqwest

In the first volume of 24 Days of Rust, I've
[written about hyper](https://zsiciarz.github.io/24daysofrust/book/vol1/day5.html)
as the Rust HTTP toolkit. A lot of things have changed in the last two years,
but [hyper](http://hyper.rs/) is still the best solution for HTTP in Rust.
However, `hyper` is undergoing some major changes to use
[`tokio`](https://github.com/tokio-rs/tokio) for async I/O. While this
will be fantastic for use cases where performance is top priority,
it will also make `hyper` APIs a bit more complex. It's good to know about
event loops, futures and services, but sometimes we just want to send a GET
request and call it a day.

Sean McArthur created [Reqwest](https://crates.io/crates/reqwest/) specifically
for such scenarios. It is a high level HTTP client built on top of `hyper`.
The situation here is somewhat similar to Python, where
[requests](http://docs.python-requests.org/en/master/) provides a
simple API on top of heavy `urllib3` machinery. `Reqwest` is relatively new
compared to `requests`, but it has the same goal - to make simple HTTP requests
easy and delegate complex tasks to `hyper`.

`Reqwest` also uses OS-provided TLS support if possible. This avoids a lot of
pain setting up OpenSSL on Windows.

Our first request
-----------------

The simplest example is just a *fire-and-forget* call to `reqwest::get()`.

```rust
extern crate reqwest;

use std::io::Read;

let mut response = reqwest::get("https://httpbin.org/status/418")
    .expect("Failed to send request");
println!("{}", response.status());
for header in response.headers().iter() {
    println!("{}: {}", header.name(), header.value_string());
}
let mut buf = String::new();
response.read_to_string(&mut buf).expect("Failed to read response");
println!("{}", buf);
```

We can inspect status code and headers of the response as shown above.
`Response` implements the standard `std::io::Read` trait, so we can read it
into a string or a byte buffer.

```text
418 I'm a teapot
Server: nginx
Date: Tue, 20 Dec 2016 17:30:26 GMT
Content-Length: 135
Connection: keep-alive
Access-Control-Allow-Origin: *
x-more-info: http://tools.ietf.org/html/rfc2324
Access-Control-Allow-Credentials: true

    -=[ teapot ]=-

       _...._
     .'  _ _ `.
    | ."` ^ `". _,
    \_;`"---"`|//
      |       ;/
      \_     _/
        `"""`
```

Aside: I've just recently discovered the
[`std::io::copy`](https://doc.rust-lang.org/std/io/fn.copy.html) function.
It takes two values - one `Read` and the other `Write` - and streams bytes
from the reader to the writer. So if we wanted to just print the response
to standard output, we can do it like this:

```rust
use std::io::copy;

copy(&mut response, &mut stdout()).expect("Failed to read response");
```

Using a Client to POST/PUT data
-------------------------------

With `hyper` we have to be rather verbose when it comes to POST-ing data.
We need to manually serialize/urlencode key/value pairs before actually
stuffing them in the request body. `reqwest` handles that automatically
for us. But while `get()` was a nice shortcut, most of the time we'll be
working with a `Client`.

```rust
use std::collections::HashMap;

let client = reqwest::Client::new();
let mut params = HashMap::new();
params.insert("name", "Sir Lancelot");
params.insert("quest", "to seek the Holy Grail");
params.insert("favorite_colour", "blue");
let mut response = client.post("https://httpbin.org/post")
    .form(&params)
    .send()
    .expect("Failed to send request");
let mut buf = String::new();
response.read_to_string(&mut buf).expect("Failed to read response");
println!("{}", buf);
```

All request methods of a `Client` do not return a response just yet.
The return value is a
[`RequestBuilder`](https://docs.rs/reqwest/0.2.0/reqwest/struct.RequestBuilder.html),
which allows to add payload, headers etc. before actually sending the request
with `send()`.

```text
{
  "args": {},
  "data": "",
  "files": {},
  "form": {
    "favorite_colour": "blue",
    "name": "Sir Lancelot",
    "quest": "to seek the Holy Grail"
  },
  // ...
}
```

We can submit the same payload (the `HashMap`) as a JSON-encoded request
body. And while we're at it, let's change the HTTP method to PUT.

```rust
let mut response = client.request(reqwest::Method::Put, "https://httpbin.org/put")
    .json(&params)
    .send()
    .expect("Failed to send request");
let mut buf = String::new();
response.read_to_string(&mut buf).expect("Failed to read response");
println!("{}", buf);
```

There's no shortcut method for PUT, but `request()` takes the `Method` enum
variant as its first argument. The `RequestBuilder::json()` method serializes
its argument to JSON and sets that as request's body.

```text
{
  "args": {},
  "data": "{\"name\":\"Sir Lancelot\",\"quest\":\"to seek the Holy Grail\",\"favorite_colour\":\"blue\"}",
  "files": {},
  "form": {},
  // ...
  "json": {
    "favorite_colour": "blue",
    "name": "Sir Lancelot",
    "quest": "to seek the Holy Grail"
  },
  // ...
}
```

Basic Authentication
--------------------

`reqwest` re-exports the entire `hyper::headers` module as part of its
public API. This means we can use the strongly-typed headers from `hyper`
directly in our requests. For example if we wanted to access a page protected
by HTTP Basic Auth, we can do it as follows:

```rust
use reqwest::header::{Authorization, Basic};

let response = client.get("https://httpbin.org/basic-auth/user/passwd")
    .send()
    .expect("Failed to send request");
println!("{}", response.status());

let credentials = Basic {
    username: "user".to_string(),
    password: Some("passwd".to_string()),
};
let response = client.get("https://httpbin.org/basic-auth/user/passwd")
    .header(Authorization(credentials))
    .send()
    .expect("Failed to send request");
println!("{}", response.status());
```

And what are the responses in both cases?

```text
401 Unauthorized
200 OK
```

Decoding a JSON response
------------------------

Not only can we send JSON as the request body, we can deserialize the response
as well. Let's use the free and open [Pokéapi](http://pokeapi.co/) to learn
more about creatures from a mildly popular game franchise.

```rust
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Deserialize)]
struct Move {
    name: String,
}

#[derive(Debug, Deserialize)]
struct PokemonMove {
    #[serde(rename="move")]
    move_: Move,
}

#[derive(Debug, Deserialize)]
struct Pokemon {
    id: i32,
    name: String,
    height: i32,
    weight: i32,
    moves: Vec<PokemonMove>,
}

let mut response = client.get("http://pokeapi.co/api/v2/pokemon/111")
    .send()
    .expect("Failed to send request");
if let Ok(pokemon) = response.json::<Pokemon>() {
    println!("{:#?}", pokemon);
}
```

Responses from `reqwest` have a `json()` convenience method, which tries to
deserialize the content from JSON into a Rust struct. Note that we had to
rename the `move` field as it is a keyword in Rust. The responses from Pokéapi
contain a lot more data, but that's ok with `serde`. Excessive JSON fields
will be ignored until we add them to the struct.

```text
$ cargo run
Pokemon {
    id: 111,
    name: "rhyhorn",
    height: 10,
    weight: 1150,
    moves: [
        PokemonMove {
            move_: Move {
                name: "swords-dance"
            }
        },
        PokemonMove {
            move_: Move {
                name: "stomp"
            }
        },
        // and a whole lot more...
    ]
}
```


Further reading
---------------

 - [Introducing Reqwest](http://seanmonstar.com/post/153221119046/introducing-reqwest)
 - [reddit thread](https://www.reddit.com/r/rust/comments/51ehop/tokio_branch_of_hyper/) about a Tokio branch of hyper
