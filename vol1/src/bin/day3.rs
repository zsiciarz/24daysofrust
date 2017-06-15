extern crate csv;
extern crate rustc_serialize;

use csv::{Reader, Writer};

#[derive(RustcDecodable, RustcEncodable)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: usize,
}

fn main() {
    println!("24 days of Rust - csv (day 3)");
    let dollar_films = vec![
        ("A Fistful of Dollars", "Rojo", 1964),
        ("For a Few Dollars More", "El Indio", 1965),
        ("The Good, the Bad and the Ugly", "Tuco", 1966),
    ];
    let path = "westerns.csv";
    let mut writer = Writer::from_file(path).unwrap();
    for row in dollar_films {
        writer.encode(row).expect("CSV writer error");
    }
    let movie = Movie {
        title: "Hang 'Em High".to_string(),
        bad_guy: "Wilson".to_string(),
        pub_year: 1968,
    };
    writer.encode(movie).expect("CSV writer error");
    writer.flush().expect("Flush error");
    let mut reader = Reader::from_file(path).unwrap().has_headers(false);
    for row in reader.decode() {
        let row: (String, String, usize) = row.unwrap();
        println!("{:?}", row);
    }
    let mut reader = Reader::from_file(path).unwrap().has_headers(false);
    for row in reader.decode() {
        let movie: Movie = row.unwrap();
        println!(
            "{} was a bad guy in '{}' in {}",
            movie.bad_guy,
            movie.title,
            movie.pub_year
        );
    }
}
