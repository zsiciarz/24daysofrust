# Day 13 - zip and lzma compression

The [`zip`](https://crates.io/crates/zip) crate is the most commonly used
Rust library for manipulating ZIP archives. It supports reading and writing
.zip files with different compression methods (store, deflate, bzip2).

There are at least three crates for LZMA (de)compression on crates.io.
[`lzma`](https://crates.io/crates/lzma) is pure Rust, but currently allows
only reading from archives.
[`rust-lzma`](https://crates.io/crates/rust-lzma) supports both reading
and writing compressed data, but it's a wrapper for `liblzma`. Similarly
the [`xz2`](https://crates.io/crates/xz2) crate is a binding for `liblzma`.

Creating archives
-----------------

We're going to compress a text file, namely the `Cargo.lock` file which
exists in every `cargo` project and keeps track of precise versions
of dependencies. This is for illustration only, the lock file doesn't grow
so much it would require compression.

```rust
static FILE_CONTENTS: &'static [u8] = include_bytes!("../../Cargo.lock");
```

The `include_bytes!` macro comes from the standard library and allows
for embedding literal arrays of bytes in the source code.

```rust
extern crate zip;

use std::io::{Seek, Write};
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};

fn create_zip_archive<T: Seek + Write>(buf: &mut T) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);
    writer.start_file("example.txt", FileOptions::default())?;
    writer.write(FILE_CONTENTS)?;
    writer.finish()?;
    Ok(())
}
```

The `zip` crate exposes a `ZipWriter` struct which wraps anything that's
`Seek + Write` (a file, stdout, an in-memory buffer etc).

```rust
fn main() {
    let mut file = File::create("example.zip").expect("Couldn't create file");
    create_zip_archive(&mut file).expect("Couldn't create archive");
}
```

After running this, we should now have an `example.zip` file in the current
directory. You can verify with `unzip` or a GUI archive reader like 7-Zip
that it contains correct data.

Reading ZIP archives
--------------------

In the same vein as `ZipWriter` wraps a writable object, the `ZipArchive`
is a wrapper around `Read + Seek`. We can use it to read archive contents
like in the example below:

```rust
fn browse_zip_archive<T, F, U>(buf: &mut T, browse_func: F) -> ZipResult<Vec<U>>
    where T: Read + Seek,
          F: Fn(&ZipFile) -> ZipResult<U>
{
    let mut archive = ZipArchive::new(buf)?;
    (0..archive.len())
        .map(|i| archive.by_index(i).and_then(|file| browse_func(&file)))
        .collect()
}
```

The `browse_zip_archive` function goes through all files in the archive and
applies a callback function to each one. This flexibility allows the caller
to decide what to do with each file in turn. The values returned by the
callback are collected into a `Vec` and returned if all goes well. We're
using a clever trick here: `Result`
[implements `FromIterator`](https://doc.rust-lang.org/std/result/enum.Result.html#method.from_iter).
This means we can turn an iterator of `Result`s into a `Result` wrapping a
container (`Vec` here) with a single call to `collect()`. And if any element
is an `Err`, the `Err` is returned from the entire function.

```rust
let mut file = File::open("example.zip").expect("Couldn't open file");
let files = browse_zip_archive(&mut file, |f| {
    Ok(format!("{}: {} -> {}", f.name(), f.size(), f.compressed_size()))
});
println!("{:?}", files);
```

```text
$ cargo run
Ok(["example.txt: 66386 -> 10570"])
```

Other archive formats
---------------------

```rust
fn create_bz2_archive<T: Seek + Write>(buf: &mut T) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);
    writer.start_file("example.txt",
                    FileOptions::default().compression_method(zip::CompressionMethod::Bzip2))?;
    writer.write(FILE_CONTENTS)?;
    writer.finish()?;
    Ok(())
}
```

We can use `zip` to create a BZIP2 archive. The only change is in the
compression method used by `ZipWriter`.

And now let's use the `rust-lzma` crate to compress our file to an `.xz`
archive.

```rust
use lzma::{LzmaWriter, LzmaError};

fn create_xz_archive<T: Write>(buf: &mut T) -> Result<(), LzmaError> {
    let mut writer = LzmaWriter::new_compressor(buf, 6)?;
    writer.write(FILE_CONTENTS)?;
    writer.finish()?;
    Ok(())
}
```

LZMA compression doesn't require the buffer to be seekable, it just emits a
stream of compressed bytes as it goes over the input. The other difference
is that `LzmaWriter` supports different compression levels (6 is typically
the default).

Comparison
----------

We may be interested in space efficiency of various compression methods.
Let's use the [`metadata`](https://doc.rust-lang.org/std/fs/fn.metadata.html)
function to retrieve size of each file:

```rust
if let Ok(meta) = metadata("example.zip") {
    println!("ZIP file size: {} bytes", meta.len());
}
if let Ok(meta) = metadata("example.bz2") {
    println!("BZ2 file size: {} bytes", meta.len());
}
if let Ok(meta) = metadata("example.xz") {
    println!("XZ file size: {} bytes", meta.len());
}
```

```text
$ cargo run
ZIP file size: 10696 bytes
BZ2 file size: 8524 bytes
XZ file size: 9154 bytes
```

Further reading
---------------

 - [tar-rs](https://github.com/alexcrichton/tar-rs)
 - [brotli](https://crates.io/crates/brotli)
 - [Comparison of archive formats](https://en.wikipedia.org/wiki/Comparison_of_archive_formats)
 - [Gzip vs Bzip2 vs XZ Performance Comparison](https://www.rootusers.com/gzip-vs-bzip2-vs-xz-performance-comparison/)
