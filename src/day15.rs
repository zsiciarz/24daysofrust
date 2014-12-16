#![feature(phase)]

extern crate fuse;
#[phase(plugin)]
extern crate json_macros;
extern crate time;
extern crate libc;
extern crate serialize;

use std::collections::TreeMap;
use std::io::{FileType, USER_FILE, USER_DIR};
use std::mem;
use std::os;
use libc::{ENOENT};
use time::Timespec;
use fuse::{FileAttr, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
use serialize::json;

struct JsonFilesystem {
    tree: json::Object,
    attrs: TreeMap<u64, FileAttr>,
    inodes: TreeMap<String, u64>,
}

impl JsonFilesystem {
    fn new(tree: &json::Object) -> JsonFilesystem {
        let mut attrs = TreeMap::new();
        let mut inodes = TreeMap::new();
        let mut attr: FileAttr = unsafe { mem::zeroed() };
        attr.ino = 1;
        attr.kind = FileType::Directory;
        attr.perm = USER_DIR;
        attrs.insert(1, attr);
        inodes.insert("/".to_string(), 1);
        for (i, (key, value)) in tree.iter().enumerate() {
            let mut attr: FileAttr = unsafe { mem::zeroed() };
            attr.ino = i as u64 + 2;
            attr.kind = FileType::RegularFile;
            attr.perm = USER_FILE;
            attr.size = value.as_string().unwrap().len() as u64;
            attrs.insert(attr.ino, attr);
            inodes.insert(key.clone(), attr.ino);
        }
        JsonFilesystem { tree: tree.clone(), attrs: attrs, inodes: inodes }
    }
}

impl Filesystem for JsonFilesystem {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        match self.attrs.get(&ino) {
            Some(attr) => {
                let ttl = Timespec::new(1, 0);
                reply.attr(&ttl, attr);
            },
            None => reply.error(ENOENT),
        };
    }

    fn lookup(&mut self, _req: &Request, parent: u64, name: &PosixPath, reply: ReplyEntry) {
        println!("lookup(parent={}, name={})", parent, name.display());
        let inode = match self.inodes.get(name.as_str().unwrap()) {
            Some(inode) => inode,
            None => {
                reply.error(ENOENT);
                return;
            },
        };
        match self.attrs.get(inode) {
            Some(attr) => {
                let ttl = Timespec::new(1, 0);
                reply.entry(&ttl, attr, 0);
            },
            None => reply.error(ENOENT),
        };

    }

    fn read(&mut self, _req: &Request, ino: u64, fh: u64, offset: u64, size: uint, reply: ReplyData) {
        println!("read(ino={}, fh={}, offset={}, size={})", ino, fh, offset, size);
        let bytes = "Hello world!\n".as_bytes();
        reply.data(bytes);
    }

    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: u64, mut reply: ReplyDirectory) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, &PosixPath::new("."));
                reply.add(1, 1, FileType::Directory, &PosixPath::new(".."));
                for (key, &inode) in self.inodes.iter() {
                    if inode == 1 { continue; }
                    let offset = inode; // hack
                    println!("\tkey={}, inode={}, offset={}", key, inode, offset);
                    reply.add(inode, offset, FileType::RegularFile, &PosixPath::new(key));
                }
            }
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }
}

fn main() {
    println!("24 days of Rust - fuse (day 15&16)");
    let data = json!({
        "foo": "bar",
        "answer": "42",
    });
    let tree = data.as_object().unwrap();
    let fs = JsonFilesystem::new(tree);
    let mountpoint = match os::args().as_slice() {
        [_, ref path] => Path::new(path),
        _ => {
            println!("Usage: {} <MOUNTPOINT>", os::args()[0]);
            return;
        }
    };
    fuse::mount(fs, &mountpoint, &[]);
}
