use walkdir::WalkDir;
use colored::Colorize;

pub fn dirsize(args: Vec<&str>) {
    if args.len() == 1 {
        let path = &args[0];
        match std::fs::metadata(path) {
            Ok(metadata) if metadata.is_dir() => {
                let mut total_size = 0;
                for entry in WalkDir::new(path) {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => {
                            println!("Failed to access directory entry: {}", err);
                            continue;
                        }
                    };
                    if entry.file_type().is_file() {
                        match entry.metadata() {
                            Ok(metadata) => total_size += metadata.len(),
                            Err(err) => println!("Failed to get file size: {}", err),
                        }
                    }
                }
                println!("Size : {}", bytes_to_human_readable(total_size));
            }
            Ok(_) => println!("{} is not a directory", path),
            Err(err) => println!("Failed to access path: {}", err),
        }
    } else {
        println!("{}", "dirsize command requires 1 argument!".red());
    }
}

pub fn bytes_to_human_readable(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes < KB {
        return format!("{} B", bytes);
    } else if bytes < MB {
        return format!("{:.2} KB", (bytes as f64) / (KB as f64));
    } else if bytes < GB {
        return format!("{:.2} MB", (bytes as f64) / (MB as f64));
    } else {
        return format!("{:.2} GB", (bytes as f64) / (GB as f64));
    }
}