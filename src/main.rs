use std::path::PathBuf;
use std::{env, fs::File};
use std::io::{Bytes, Read};

mod byte;
mod gob;

fn main() -> std::io::Result<()> {
    let mode = env::args().nth(1).expect("Mode should be provided.").to_lowercase();

    match mode.as_str() {
        "extract" | "x" => extract(),
        "create" | "c" => create(),
        _ => panic!("Unknown mode: {mode}"),
    }
}

fn create() -> std::io::Result<()> {
    panic!("Not yet implemented.");

    Ok(())
}

fn extract() -> std::io::Result<()> {
    let source = PathBuf::from(env::args().nth(2).expect("Source should be provided."));

    let file_stem = source.file_stem().expect("Should be able to get file stem.");

    let parent_directory = source.parent().expect("Should be able to get parent directory.");

    let file_name = source.file_name().expect("Should be able to get file name.").to_str().expect("Should be able to convert to str.").to_string();

    let destination = match env::args().nth(3) {
        Some(destination) => PathBuf::from(destination),
        None => parent_directory.join(file_stem),
    };

    let file = File::open(source)?;

    println!("Extracting: {file_name}");

    println!("Target: {}", destination.display());

    let mut bytes = file.bytes();

    let header = gob::Header::new(
        byte::slice!(bytes, 4),
        byte::slice!(bytes, 4),
        byte::slice!(bytes, 4),
    );

    for _ in 0..(u32::from_le_bytes(header.body_offset) - 12) {
        bytes.next();
    }

    let file_count = u32::from_le_bytes(byte::slice!(bytes, 4)) as usize;

    println!("Contains {file_count} files");

    Ok(())
}