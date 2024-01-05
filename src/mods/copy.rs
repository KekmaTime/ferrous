use colored::*;

pub fn copy(args: Vec<&str>) {
    if args.len() == 2 {
        if std::fs::metadata(&args[0]).unwrap().is_dir() {
            let options = fs_extra::dir::CopyOptions::new();
            fs_extra::dir::copy(&args[0], &args[1], &options).expect("Failed to copy directory");
        } else {
            std::fs::copy(&args[0], &args[1]).expect("Failed to copy file");
        }
    } else {
        println!("{}", "cp command requires 2 arguments!".red());
    }
}