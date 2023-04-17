# Day 15 - FUSE filesystems, part 1

> Relevancy: 1.8 nightly

A traditional filesystem is typically implemented as a kernel module. However, some Unix-like operating systems (Linux, FreeBSD, Mac OS X and a few others) allow for userspace filesystems through a mechanism called [FUSE](http://en.wikipedia.org/wiki/Filesystem_in_Userspace). The [canonical FUSE library](http://fuse.sourceforge.net/) is written in C and there are some bindings from other languages ([Python](https://github.com/terencehonles/fusepy), [Ruby](https://github.com/lwoggardner/rfusefs) etc.)

The [fuse crate](https://github.com/zargony/rust-fuse) is very interesting because it's a rewrite from C to Rust, leveraging many of Rust features unavailable in C. The only binding to libfuse is related to mounting and unmounting the filesystem, the rest is pure Rust.

Small disclaimer: I'm not very well versed in filesystems, in fact this is my first attempt at FUSE. I'm learning a lot while writing this article, hopefully for the best of all of us :-)

JSON filesystem
---------------

In this and the following article we'll try to create a read-only FUSE filesystem that represents a JSON object (a key-value map). The keys will map to filenames, while file contents will represent values stored under respective keys.

Let's start with a filesystem that does absolutely nothing, but can be mounted and unmounted. The following example is almost directly taken from [rust-fuse](https://github.com/zargony/rust-fuse/blob/bcfb2f9eb6679c12881a8d002e4af89e7b3482dc/examples/null.rs) sources. This will be our starting point:

```rust
extern crate fuse;

use std::env;
use fuse::Filesystem;

struct JsonFilesystem;

impl Filesystem for JsonFilesystem {
}

fn main() {
    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };
    fuse::mount(JsonFilesystem, &mountpoint, &[]);
}
```

Create the mount point (fancy name for an empty directory) and run the program:

```sh
$ mkdir /tmp/rust-fuse
$ cargo run /tmp/rust-fuse
```

**Note**: If you get `fusermount: failed to open /etc/fuse.conf: Permission denied` errors, you need to add yourself to the `fuse` group:

```sh
$ sudo addgroup <USERNAME> fuse
```

Log out and log in again to apply changes.

Now try the following in another terminal window, while our Rust program is still running:

```sh
$ ls /tmp/rust-fuse
ls: cannot access /tmp/rust-fuse: Function not implemented
```

Great! This means our filesystem is mounted, but listing directory contents is not implemented. In fact no action is implemented yet, so let's get to work!

By the way - to unmount your filesystem, stop the Rust program and then execute `fusermount -u /tmp/rust-fuse`. If you don't unmount, you'll run into errors next time you try `cargo run`.

Attributes
----------

First of all we need to add a number of imports at the top. We'll be using the [libc](https://crates.io/crates/libc) and [time](https://crates.io/crates/time) crates, so make sure to add them as dependencies to `Cargo.toml`.

```rust
extern crate libc;
extern crate time;

use std::path::Path;
use libc::{ENOENT, ENOSYS};
use time::Timespec;
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
```

Let's start with the `getattr` function. FUSE will call it when reading file or directory attributes, such as size, creation time, uid etc.

```rust
impl Filesystem for JsonFilesystem {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        reply.error(ENOSYS);
    }
}
```

We still return an `ENOSYS` status code (this is what causes the `Function not implemented` error), but we will see in the program output that FUSE called `getattr` on inode number 1, which is the root directory of our filesystem. OK, now let's flesh out this method:

```rust
fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
    println!("getattr(ino={})", ino);
    let ts = Timespec::new(0, 0);
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
    let ttl = Timespec::new(1, 0);
    if ino == 1 {
        reply.attr(&ttl, &attr);
    } else {
        reply.error(ENOENT);
    }
}
```

The `FileAttr` struct is somewhat similar to `struct stat` in C. If you now run `stat` on the root directory, you'll see the permissions and familiar dates from the 70's (timestamps are 0):

```sh
$ stat /tmp/rust-fuse
File: `/tmp/rust-fuse'
Size: 0               Blocks: 0          IO Block: 4096   directory
Device: 27h/39d     Inode: 1           Links: 0
Access: (0755/drwxr-xr-x)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 1970-01-01 01:00:00.000000000 +0100
Modify: 1970-01-01 01:00:00.000000000 +0100
Change: 1970-01-01 01:00:00.000000000 +0100
```

Directory listing
-----------------

Now it's time to make `ls` work in our filesystem (for some definition of work...).

```rust
fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: u64, mut reply: ReplyDirectory) {
    println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
    reply.error(ENOSYS);
}
```

We start with a dummy `readdir` method as before, let's see if it gets called when `ls`-ing:

```sh
$ cargo run /tmp/rust-fuse
getattr(ino=1)
readdir(ino=1, fh=0, offset=0)
```

Yay! `ls` still displays an error, but that's because we wanted it to (remember, `ENOSYS`). Now we can actually implement the directory listing functionality.

```rust
fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: u64, mut reply: ReplyDirectory) {
    println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
    if ino == 1 {
        if offset == 0 {
            reply.add(1, 0, FileType::Directory, &Path::new("."));
            reply.add(1, 1, FileType::Directory, &Path::new(".."));
        }
        reply.ok();
    } else {
        reply.error(ENOENT);
    }
}
```

As with `getattr` we check only the first inode, otherwise return a `File not found` error (`ENOENT`). The `offset == 0` check is necessary, otherwise `readdir` will loop infinitely. So what happens if we run `ls` now?

```sh
$ ls -la /tmp/rust-fuse
total 4
drwxr-xr-x  0 root root    0 Jan  1  1970 .
drwxrwxrwt 10 root root 4096 Dec 10 00:38 ..
```

Hooray! We've implemented an empty directory :-)

To be continued...
------------------

That was a lot for today, time to take a short break. See you tomorrow in [part 2](day16.md)!

See also
--------

* [A nice overview](http://www.cs.hmc.edu/~geoff/classes/hmc.cs135.201109/homework/fuse/fuse_doc.html#function-purposes) of FUSE functions
* [FUSE "hello world" program in C](http://fuse.sourceforge.net/helloworld.html)
