use std::path::PathBuf;
use std::time::Instant;
use std::{env, fs};
use std::io::Write;

use crate::gob::Gob;

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

    let start_moment = Instant::now();

    println!("Reading: {file_name}");

    let gob = Gob::from(source);

    println!("Found {} files", gob.files.len());

    println!("Extracting to: {}", destination.display());

    fs::create_dir_all(&destination)?;

    for virtual_file in gob.files {
        match virtual_file.filepath.parent() {
            Some(parent) => fs::create_dir_all(destination.join(parent))?,
            None => (),
        };

        let mut file = fs::File::create(&destination.join(&virtual_file.filepath))?;

        file.write_all(&virtual_file.data)?;

        println!("Created: {}", virtual_file.filepath.display());
    }

    println!("Extraction complete.");

    println!("Elapsed: {:.2?}", start_moment.elapsed());

    Ok(())
}