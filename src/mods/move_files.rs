use colored::*;

pub fn move_files(args: Vec<&str>){
    if args.len() == 2 {
        if std::fs::metadata(&args[0]).unwrap().is_dir() {
            let options = fs_extra::dir::CopyOptions::new();
            fs_extra::dir::copy(&args[0], &args[1], &options).expect("Failed to copy directory");
            fs_extra::dir::remove(&args[0]).expect("Failed to remove original directory");
        } 
        else {
            std::fs::rename(&args[0], &args[1]).expect("Failed to move file");
        }
    } 
    else {
        println!("{}", "mv command requires 2 arguments!".red());
    }
}