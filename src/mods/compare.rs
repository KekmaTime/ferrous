use std::fs;
use std::io;

pub fn compare_files(path1: &str, path2: &str) -> io::Result<bool> {
    let contents1 = fs::read_to_string(path1)?;
    let contents2 = fs::read_to_string(path2)?;
    Ok(contents1 == contents2)
}