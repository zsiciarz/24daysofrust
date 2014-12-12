extern crate image;

use std::io::File;

fn main() {
    println!("24 days of Rust - image (day 12)");
    let img = image::open(&Path::new("data/in.jpg")).ok().expect("Opening image failed");
    let filtered = img.fliph();
    let out = File::create(&Path::new("out.jpg")).unwrap();
    let _ = filtered.save(out, image::JPEG).ok().expect("Saving image failed");

    let kernel = [-1.0f32, -1.0, -1.0,
                  -1.0, 8.0, -1.0,
                  -1.0, -1.0, -1.0];
    let filtered = img.filter3x3(&kernel);
    let out = File::create(&Path::new("out.jpg")).unwrap();
    let _ = filtered.save(out, image::JPEG).ok().expect("Saving image failed");
}
