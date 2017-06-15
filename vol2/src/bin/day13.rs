#[cfg(target_family = "unix")]
extern crate lzma;
extern crate zip;

use std::fs::{File, metadata};
use std::io::{Read, Seek, Write};
use zip::read::{ZipArchive, ZipFile};
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};

#[cfg(target_family = "unix")]
use lzma::{LzmaWriter, LzmaError};

static FILE_CONTENTS: &'static [u8] = include_bytes!("../../Cargo.lock");

fn create_zip_archive<T: Seek + Write>(buf: &mut T) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);
    writer.start_file("example.txt", FileOptions::default())?;
    writer.write_all(FILE_CONTENTS)?;
    writer.finish()?;
    Ok(())
}

fn browse_zip_archive<T, F, U>(buf: &mut T, browse_func: F) -> ZipResult<Vec<U>>
where
    T: Read + Seek,
    F: Fn(&ZipFile) -> ZipResult<U>,
{
    let mut archive = ZipArchive::new(buf)?;
    (0..archive.len())
        .map(|i| archive.by_index(i).and_then(|file| browse_func(&file)))
        .collect()
}

fn create_bz2_archive<T: Seek + Write>(buf: &mut T) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);
    writer.start_file(
        "example.txt",
        FileOptions::default().compression_method(
            zip::CompressionMethod::Bzip2,
        ),
    )?;
    writer.write_all(FILE_CONTENTS)?;
    writer.finish()?;
    Ok(())
}

#[cfg(target_family = "unix")]
fn create_xz_archive<T: Write>(buf: &mut T) -> Result<(), LzmaError> {
    let mut writer = LzmaWriter::new_compressor(buf, 6)?;
    writer.write_all(FILE_CONTENTS)?;
    writer.finish()?;
    Ok(())
}

#[cfg(target_family = "windows")]
fn create_xz_archive<T: Write>(_: &mut T) -> Result<(), ()> {
    Ok(())
}

fn main() {
    println!("24 Days of Rust vol. 2 - zip");
    let mut file = File::create("example.zip").expect("Couldn't create file");
    create_zip_archive(&mut file).expect("Couldn't create archive");

    let mut file = File::open("example.zip").expect("Couldn't open file");
    let files = browse_zip_archive(&mut file, |f| {
        Ok(format!(
            "{}: {} -> {}",
            f.name(),
            f.size(),
            f.compressed_size()
        ))
    });
    println!("{:?}", files);

    let mut file = File::create("example.bz2").expect("Couldn't create file");
    create_bz2_archive(&mut file).expect("Couldn't create archive");

    let mut file = File::create("example.xz").expect("Couldn't create file");
    create_xz_archive(&mut file).expect("Couldn't create archive");

    if let Ok(meta) = metadata("example.zip") {
        println!("ZIP file size: {} bytes", meta.len());
    }
    if let Ok(meta) = metadata("example.bz2") {
        println!("BZ2 file size: {} bytes", meta.len());
    }
    if let Ok(meta) = metadata("example.xz") {
        println!("XZ file size: {} bytes", meta.len());
    }
}
