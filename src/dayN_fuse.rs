extern crate fuse;
extern crate libc;

use std::os;
use libc::{ENOENT, ENOSYS};
use fuse::{FileAttr, Filesystem, Request, ReplyAttr, ReplyEntry, ReplyDirectory};

struct JsonFilesystem;

impl Filesystem for JsonFilesystem {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(): ino {}", ino);
        reply.error(ENOSYS);
    }
}

fn main() {
    println!("24 days of Rust - fuse (day N)");
    let mountpoint = Path::new(os::args()[1].as_slice());
    fuse::mount(JsonFilesystem, &mountpoint, &[]);
}
