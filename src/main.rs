// #![allow(unused)]
use chrono::prelude::{DateTime, Local};
use clap::Parser;
use core::panic;
use std::fs::{self};
use std::path::Path;

#[derive(Parser)]
struct CliArgs {
    // The path of the files to organize
    working_directory: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: CliArgs = CliArgs::parse();
    let working_directory = &arguments.working_directory;
    println!("Beginning reorganization of files based on Created Date metadata.");

    // Get the ReadDir iterator for the working directory
    if let Ok(files) = fs::read_dir(working_directory) {
        for file_result in files {
            if let Ok(file) = file_result {
                // Ignore stupid file system files and directories already inside the working directory
                if file.file_name() == ".DS_Store" || Path::new(&file.path()).is_dir() {
                    continue;
                };

                if let Ok(metadata) = file.metadata() {
                    let created_at_iso_date = DateTime::<Local>::from(metadata.created().unwrap())
                        .date_naive()
                        .to_string();

                    let move_to_location = Path::new(&working_directory)
                        .join(&created_at_iso_date)
                        .join(file.file_name());

                    if !Path::new(move_to_location.parent().unwrap()).is_dir() {
                        println!("Creating directory for date {}", &created_at_iso_date);
                        let _ = fs::create_dir(Path::new(move_to_location.parent().unwrap()));
                        // TODO Handle the Result if it errored - It's possible if the file is in use for it to fail the rename/move.
                    }

                    // Using rename "moves" the file from where it is (i.e. working_directory) to within the directory of its created date
                    let _ = fs::rename(file.path(), move_to_location);
                } else {
                    println!(
                        "Error reading metadata for file: {:?}; at path: {:?}. Skipping this file and continuing with the others.",
                        file.file_name(),
                        file.path()
                    );
                }
            }
        }
        println!("Files organized.");
        return Ok(());
    } else {
        panic!("Error reading files for directory: {}", &working_directory)
    }
}
