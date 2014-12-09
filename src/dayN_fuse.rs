extern crate fuse;

use std::os;
use fuse::Filesystem;

struct JsonFilesystem;

impl Filesystem for JsonFilesystem {
}

fn main() {
    println!("24 days of Rust - fuse (day N)");
    let mountpoint = Path::new(os::args()[1].as_slice());
    fuse::mount(JsonFilesystem, &mountpoint, &[]);
}
