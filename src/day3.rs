extern crate csv;
extern crate "rustc-serialize" as rustc_serialize;

use csv::{Reader, Writer};

#[deriving(RustcDecodable, RustcEncodable)]
struct Movie {
    title: String,
    bad_guy: String,
    pub_year: uint,
}

fn main() {
    println!("24 days of Rust - csv (day 3)");
    let dollar_films = vec![
        ("A Fistful of Dollars", "Rojo", 1964u),
        ("For a Few Dollars More", "El Indio", 1965u),
        ("The Good, the Bad and the Ugly", "Tuco", 1966u),
    ];
    let path = Path::new("westerns.csv");
    let mut writer = Writer::from_file(&path);
    for row in dollar_films.into_iter() {
        writer.encode(row).ok().expect("CSV writer error");
    }
    let movie = Movie {
        title: "Hang 'Em High".to_string(),
        bad_guy: "Wilson".to_string(),
        pub_year: 1968u,
    };
    writer.encode(movie).ok().expect("CSV writer error");
    writer.flush().ok().expect("Flush error");
    let mut reader = Reader::from_file(&path).has_headers(false);
    for row in reader.decode() {
        let row: (String, String, uint) = row.unwrap();
        println!("{}", row);
    }
    let mut reader = Reader::from_file(&path).has_headers(false);
    for row in reader.decode() {
        let movie: Movie = row.unwrap();
        println!("{} was a bad guy in '{}' in {}",
            movie.bad_guy, movie.title, movie.pub_year);
    }
}
