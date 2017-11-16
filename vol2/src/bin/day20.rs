extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::io::{copy, Read, stdout};

use reqwest::header::{Authorization, Basic};

#[derive(Debug, Deserialize)]
struct Move {
    name: String,
}

#[derive(Debug, Deserialize)]
struct PokemonMove {
    #[serde(rename = "move")]
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

fn main() {
    println!("24 Days of Rust vol. 2 - reqwest");
    let mut response =
        reqwest::get("https://httpbin.org/status/418").expect("Failed to send request");
    println!("{}", response.status());
    for header in response.headers().iter() {
        println!("{}: {}", header.name(), header.value_string());
    }
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect(
        "Failed to read response",
    );
    println!("{}", buf);
    copy(&mut response, &mut stdout()).expect("Failed to read response");

    let client = reqwest::Client::new();
    let mut params = HashMap::new();
    params.insert("name", "Sir Lancelot");
    params.insert("quest", "to seek the Holy Grail");
    params.insert("favorite_colour", "blue");
    let mut response = client
        .post("https://httpbin.org/post")
        .form(&params)
        .send()
        .expect("Failed to send request");
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect(
        "Failed to read response",
    );
    println!("{}", buf);

    let mut response = client
        .request(reqwest::Method::Put, "https://httpbin.org/put")
        .json(&params)
        .send()
        .expect("Failed to send request");
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect(
        "Failed to read response",
    );
    println!("{}", buf);

    let response = client
        .get("https://httpbin.org/basic-auth/user/passwd")
        .send()
        .expect("Failed to send request");
    println!("{}", response.status());

    let credentials = Basic {
        username: "user".to_string(),
        password: Some("passwd".to_string()),
    };
    let response = client
        .get("https://httpbin.org/basic-auth/user/passwd")
        .header(Authorization(credentials))
        .send()
        .expect("Failed to send request");
    println!("{}", response.status());

    let mut response = client
        .get("http://pokeapi.co/api/v2/pokemon/111")
        .send()
        .expect("Failed to send request");
    if let Ok(pokemon) = response.json::<Pokemon>() {
        println!("{:#?}", pokemon);
    }
}
