use colored::*;
use std::fs;
use walkdir::WalkDir;

pub fn remove_files(args: Vec<&str>) {
    if args.len() == 1 {
        let path = &args[0];
        match fs::metadata(path) {
            Ok(metadata) if metadata.is_dir() => {
                for entry in WalkDir::new(path) {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => {
                            println!("Failed to access directory entry: {}", err);
                            continue;
                        }
                    };
                    if entry.file_type().is_file() {
                        if let Err(err) = fs::remove_file(entry.path()) {
                            println!("Failed to remove file: {}", err);
                        }
                    }
                }
                if let Err(err) = fs::remove_dir_all(path) {
                    println!("Failed to remove directory: {}", err);
                }
            }
            Ok(_) => {
                if let Err(err) = fs::remove_file(path) {
                    println!("Failed to remove file: {}", err);
                }
            }
            Err(err) => println!("Failed to access path: {}", err),
        }
    } else {
        println!("{}", "rm command requires 1 argument!".red());
    }
}