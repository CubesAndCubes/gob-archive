use std::path::PathBuf;
use std::time::Instant;
use std::{env, fs};
use std::io::{Seek, SeekFrom, Write};

use gob_rs::core::Gob;

fn main() -> std::io::Result<()> {
    let mode = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No mode provided. Must be one of: x | extract | c | create");

        std::process::exit(1);
    });

    match mode.to_lowercase().as_str() {
        "x" => extract(false),
        "extract" => extract(true),
        "c" => create(false),
        "create" => create(true),
        _ => {
            eprintln!("Unknown mode: {mode}");

            std::process::exit(1);
        },
    }
}

fn create(verbose: bool) -> std::io::Result<()> {
    let source = PathBuf::from(env::args().nth(2).unwrap_or_else(|| {
        eprintln!("No source provided to create archive from.");

        std::process::exit(1);
    }));

    if !source.is_dir() {
        eprintln!("Source is not a directory: {}", source.display());

        std::process::exit(1);
    }

    let file_name = source.file_name().expect("Should be able to get file name.").to_str().expect("Should be able to convert to str.").to_string();

    let destination = match env::args().nth(3) {
        Some(destination) => PathBuf::from(destination),
        None => source.with_extension("GOB"),
    };

    let start_moment = Instant::now();

    println!("Reading: {file_name}");

    let gob = Gob::from_directory(&source)?;

    let file_count = gob.files.len() as u32;

    println!("Found {file_count} files");

    println!("Creating archive at: {}", destination.display());

    fs::create_dir_all(&destination.parent().expect("Should be able to get parent directory"))?;

    let mut file = fs::File::create(&destination)?;

    file.write_all(b"GOB ")?;

    let version: u32 = 0x14;

    file.write_all(&version.to_le_bytes())?;

    let body_offset: u32 = 12;

    file.write_all(&body_offset.to_le_bytes())?;

    file.write_all(&file_count.to_le_bytes())?;

    let mut file_data_offset: u32 = 16 + 136 * file_count;

    for virtual_file in &gob.files {
        file.write_all(&file_data_offset.to_le_bytes())?;

        let size = virtual_file.data.len() as u32;

        file_data_offset += size;

        file.write_all(&size.to_le_bytes())?;

        let filepath = virtual_file.filepath.as_os_str().as_encoded_bytes();

        file.write_all(&filepath)?;

        file.seek(SeekFrom::Current(128 - filepath.len() as i64))?;

        if verbose {
            println!("Archived: {}", virtual_file.filepath.display());
        }
    }

    for virtual_file in &gob.files {
        file.write_all(&virtual_file.data)?;
    }

    println!("Archive creation complete.");

    println!("Elapsed: {:.2?}", start_moment.elapsed());

    Ok(())
}

fn extract(verbose: bool) -> std::io::Result<()> {
    let source = PathBuf::from(env::args().nth(2).unwrap_or_else(|| {
        eprintln!("No source provided to extract from.");

        std::process::exit(1);
    }));

    if !source.is_file() {
        eprintln!("Source is not a file: {}", source.display());

        std::process::exit(1);
    }

    let file_stem = source.file_stem().expect("Should be able to get file stem.");

    let parent_directory = source.parent().expect("Should be able to get parent directory.");

    let file_name = source.file_name().expect("Should be able to get file name.").to_str().expect("Should be able to convert to str.").to_string();

    let destination = match env::args().nth(3) {
        Some(destination) => PathBuf::from(destination),
        None => parent_directory.join(file_stem),
    };

    let start_moment = Instant::now();

    println!("Reading: {file_name}");

    let gob = Gob::from_file(&source)?;

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

        if verbose {
            println!("Created: {}", virtual_file.filepath.display());
        }
    }

    println!("Extraction complete.");

    println!("Elapsed: {:.2?}", start_moment.elapsed());

    Ok(())
}
