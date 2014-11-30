extern crate csv;

use csv::Writer;

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
}
