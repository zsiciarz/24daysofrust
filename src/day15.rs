#![feature(plugin)]
#![plugin(clippy)]
#![plugin(json_macros)]

extern crate fuse;
extern crate time;
extern crate libc;
extern crate rustc_serialize;

use std::collections::BTreeMap;
use std::env;
use std::path::Path;
use libc::{ENOENT};
use time::Timespec;
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
use rustc_serialize::json;

struct JsonFilesystem {
    tree: json::Object,
    attrs: BTreeMap<u64, FileAttr>,
    inodes: BTreeMap<String, u64>,
}

impl JsonFilesystem {
    fn new(tree: &json::Object) -> JsonFilesystem {
        let mut attrs = BTreeMap::new();
        let mut inodes = BTreeMap::new();
        let ts = time::now().to_timespec();
        let attr = FileAttr {
            ino: 1,
            size: 0,
            blocks: 0,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        };
        attrs.insert(1, attr);
        inodes.insert("/".to_owned(), 1);
        for (i, (key, value)) in tree.iter().enumerate() {
            let attr = FileAttr {
                ino: i as u64 + 2,
                size: value.pretty().to_string().len() as u64,
                blocks: 0,
                atime: ts,
                mtime: ts,
                ctime: ts,
                crtime: ts,
                kind: FileType::RegularFile,
                perm: 0o644,
                nlink: 0,
                uid: 0,
                gid: 0,
                rdev: 0,
                flags: 0,
            };
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

    fn lookup(&mut self, _req: &Request, parent: u64, name: &Path, reply: ReplyEntry) {
        println!("lookup(parent={}, name={})", parent, name.display());
        let inode = match self.inodes.get(name.to_str().unwrap()) {
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

    fn read(&mut self, _req: &Request, ino: u64, fh: u64, offset: u64, size: u32, reply: ReplyData) {
        println!("read(ino={}, fh={}, offset={}, size={})", ino, fh, offset, size);
        for (key, &inode) in &self.inodes {
            if inode == ino {
                let value = self.tree.get(key).unwrap();
                reply.data(value.pretty().to_string().as_bytes());
                return;
            }
        }
        reply.error(ENOENT);
    }

    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: u64, mut reply: ReplyDirectory) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, &Path::new("."));
                reply.add(1, 1, FileType::Directory, &Path::new(".."));
                for (key, &inode) in &self.inodes {
                    if inode == 1 { continue; }
                    let offset = inode; // hack
                    println!("\tkey={}, inode={}, offset={}", key, inode, offset);
                    reply.add(inode, offset, FileType::RegularFile, &Path::new(key));
                }
            }
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }
}

fn main() {
    println!("24 days of Rust - fuse (days 15 & 16)");
    let data = json!({
        "foo": "bar",
        "answer": 42,
    });
    let tree = data.as_object().unwrap();
    let fs = JsonFilesystem::new(tree);
    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };
    fuse::mount(fs, &mountpoint, &[]);
}
