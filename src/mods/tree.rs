use std::fs;
use std::io;
use crate::dirsize;

pub struct Counts {
  dirs: i32,
  files: i32,
  size: u64
}

pub fn walk(dir: &str, prefix: &str, counts: &mut Counts) -> io::Result<()> {
  let mut paths: Vec<_> = fs::read_dir(dir)?.map(|entry| entry.unwrap().path()).collect();
  let mut index = paths.len();

  paths.sort_by(|a, b| {
    let aname = a.file_name().unwrap().to_str().unwrap();
    let bname = b.file_name().unwrap().to_str().unwrap();
    aname.cmp(bname)
  });

  for path in paths {
    let name = path.file_name().unwrap().to_str().unwrap();
    index -= 1;

    if name.starts_with(".") {
      continue;
    }

    if path.is_dir() {
      counts.dirs += 1;
      walk(&format!("{}/{}", dir, name), &format!("{}│   ", prefix), counts)?;
    } 
    else {
      counts.files += 1;
      counts.size += fs::metadata(&path)?.len();
    }

    if index == 0 {
      println!("{}└── {} ({})", prefix, name, dirsize::bytes_to_human_readable(counts.size));
    } 
    else {
      println!("{}├── {} ({})", prefix, name, dirsize::bytes_to_human_readable(counts.size));
    }
  }

  Ok(())
}

pub fn tree(dir: &str) -> io::Result<()> {
  println!("{}", dir);

  let mut counts = Counts { dirs: 0, files: 0, size: 0 };
  walk(dir, "", &mut counts)?;

  println!("\n{} directories, {} files, size: {}", counts.dirs, counts.files, dirsize::bytes_to_human_readable(counts.size));

  Ok(())
}