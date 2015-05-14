# Day 16 - FUSE filesystems, part 2

We finished [part 1](https://siciarz.net/24-days-of-rust-fuse-filesystems-part-1/) with a working FUSE filesystem representing an empty directory. Today we will continue the story and finally add some files to the mix.

Directory listing revisited
---------------------------

We need to add a minimal amount of JSON handling before actually presenting the data in our filesystem. The `JsonFilesystem` struct will hold a `json::Object` value (this is a typedef for a `TreeMap`) representing the root object in the input JSON.

    :::rust
    #![feature(phase)]
    #[phase(plugin)]
    extern crate json_macros;
    extern crate serialize;

    use serialize::json;

    struct JsonFilesystem {
        tree: json::Object,
    }

    fn main() {
        let data = json!({
            "foo": "bar",
            "answer": 42,
        });
        let tree = data.as_object().unwrap();
        let fs = JsonFilesystem { tree: tree.clone() };
        let mountpoint = match os::args().as_slice() {
            [_, ref path] => Path::new(path),
            _ => {
                println!("Usage: {} <MOUNTPOINT>", os::args()[0]);
                return;
            }
        };
        fuse::mount(fs, &amp;mountpoint, &amp;[]);
    }

We used the [json! macro](https://siciarz.net/24-days-of-rust-working-json/) to embed JSON data directly in the source. The `readdir` method must be updated to add directory entries for each key in the JSON.

    :::rust
    for (i, key) in self.tree.keys().enumerate() {
        let inode: u64 = 2 + i as u64;
        let offset: u64 = 2 + i as u64;
        reply.add(inode, offset, FileType::RegularFile, &amp;PosixPath::new(key));
    }

Let's look at the result:

    :::sh
    $ ls -la /tmp/rust-fuse
    total 4
    drwxr-xr-x  0 root root    0 Jan  1  1970 .
    drwxrwxrwt 10 root root 4096 Dec 10 00:38 ..
    -?????????  ? ?    ?        ?            ? answer
    -?????????  ? ?    ?        ?            ? foo

Starting to look good! Our JSON keys are there, but what's with these question marks? Turns out FUSE calls another method called `lookup()` and since we haven't defined it yet, a default implementation will be used. But before we write our customized `lookup`, let's refactor the code a bit.

Both `getattr` and `lookup` create `FileAttr` values based on inode number or filename (JSON key in our example). Since we intend our filesystem to be read-only, let's create all necessary atributes upon initialization. This will simplify the other methods a lot.

    :::rust
    struct JsonFilesystem {
        tree: json::Object,
        attrs: TreeMap<u64, FileAttr>,
        inodes: TreeMap<String, u64>,
    }

    impl JsonFilesystem {
        fn new(tree: &amp;json::Object) -> JsonFilesystem {
            let mut attrs = TreeMap::new();
            let mut inodes = TreeMap::new();
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
                perm: USER_DIR,
                nlink: 0,
                uid: 0,
                gid: 0,
                rdev: 0,
                flags: 0,
            };
            attrs.insert(1, attr);
            inodes.insert("/".to_string(), 1);
            for (i, (key, value)) in tree.iter().enumerate() {
                let attr = FileAttr {
                    ino: i as u64 + 2,
                    size: value.to_pretty_str().len() as u64,
                    blocks: 0,
                    atime: ts,
                    mtime: ts,
                    ctime: ts,
                    crtime: ts,
                    kind: FileType::RegularFile,
                    perm: USER_FILE,
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

We used `time::now()` to set current timestamps for creation/modification times instead of the epoch time (January 1, 1970). This time we're explicit about initializing `FileAttr` following the suggestion from [the reddit thread](http://www.reddit.com/r/rust/comments/2pdxbe/24_days_of_rust_fuse_filesystems_part_1/cmvtgi1). We also calculate the size of our "files" and store that in the attribute struct.

With that in our toolkit, let's implement `lookup()`.

    :::rust
    fn lookup(&amp;mut self, _req: &amp;Request, parent: u64, name: &amp;PosixPath, reply: ReplyEntry) {
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
                reply.entry(&amp;ttl, attr, 0);
            },
            None => reply.error(ENOENT),
        };
    }

This method is quite straightforward since we moved attribute-related logic to `JsonFilesystem::new`. If we run `ls` now:

    :::sh
    $ ls -la /tmp/rust-fuse
    total 4
    drwxr-xr-x  0 root root     0 Dec 16 10:17 .
    drwxrwxrwt 15 root root  4096 Dec 16 10:17 ..
    -rw-r--r--  0 root root     2 Dec 16 10:17 answer
    -rw-r--r--  0 root root     5 Dec 16 10:17 foo

No more question marks! The `getattr` method is even simpler now:

    :::rust
    fn getattr(&amp;mut self, _req: &amp;Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        match self.attrs.get(&amp;ino) {
            Some(attr) => {
                let ttl = Timespec::new(1, 0);
                reply.attr(&amp;ttl, attr);
            },
            None => reply.error(ENOENT),
        };
    }

Reading files
-------------

This is the final piece of the puzzle in our filesystem. Fortunately it is pretty simple after all we've done so far. We need to implement the `read` method.

    :::rust
    fn read(&amp;mut self, _req: &amp;Request, ino: u64, fh: u64, offset: u64, size: uint, reply: ReplyData) {
        println!("read(ino={}, fh={}, offset={}, size={})", ino, fh, offset, size);
        for (key, &amp;inode) in self.inodes.iter() {
            if inode == ino {
                let value = self.tree.get(key).unwrap();
                reply.data(value.to_pretty_str().as_bytes());
                return;
            }
        }
        reply.error(ENOENT);
    }

Let's try reading a "file" now:

    :::sh
    $ less /tmp/rust-fuse/answer
    42
    $ wc /tmp/rust-fuse/foo
    0 1 5 /tmp/rust-fuse/foo
    $ grep -rn bar /tmp/rust-fuse
    /tmp/rust-fuse/foo:1:"bar"

**Awesome!**

As usual, the code for yesterday and today is in the [24daysofrust](https://github.com/zsiciarz/24daysofrust) repository on GitHub. Of course this is by no means a complete JSON-as-a-filesystem implementation. Here are some ideas for future expansion:

 * nesting - if a key contains a JSON object, why not represent it as a subdirectory
 * creating files, writing to them, deleting etc. - a fully writeable filesystem
 * getting rid of linear search in `read` (that's evidently not very efficient)

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly.
</small>
