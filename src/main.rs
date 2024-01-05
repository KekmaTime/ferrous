mod mods;
use mods::*;
use colored::*;
type TrimFn = fn(&str) -> &str;

fn main() {
    loop {
        let input = input::get_input();
        let mut commands = input.trim().split("|").map(str::trim as TrimFn).peekable();
        let mut prev_command = None;

        while let Some(command) = commands.next() {
            let (command, args) = input::parse_input(command);

            match command {
                "cf" => {
                    commands::change_directory(args);
                    prev_command = None;
                }
                "exit" => return,
                "help" => {
                    help::print_help();
                    prev_command = None;
                }
                "copy" => {
                    let args: Vec<&str> = args.collect();
                    if args.len() == 2 {
                        if permissions::check_permissions(args[0], false).unwrap_or(false) && permissions::check_permissions(args[1], true).unwrap_or(false) {
                            copy::copy(args);
                        } else {
                            println!("Permission denied");
                        }
                    } else {
                        println!("copy command requires 2 arguments!");
                    }
                    prev_command = None;
                }
                "move" => {
                    let args: Vec<&str> = args.collect();
                    if args.len() == 2 {
                        if permissions::check_permissions(args[0], true).unwrap_or(false) && permissions::check_permissions(args[1], true).unwrap_or(false) {
                            move_files::move_files(args);
                        } else {
                            println!("Permission denied");
                        }
                    } else {
                        println!("move command requires 2 arguments!");
                    }
                    prev_command = None;
                }
                "remove" => {
                    let args: Vec<&str> = args.collect();
                    if permissions::check_permissions(args[0], true).unwrap_or(false) {
                        remove_files::remove_files(args);
                    } else {
                        println!("Permission denied");
                    }
                    prev_command = None;
                }
                "mem" => {
                    let mem_info = sys_info::mem_info().expect("Failed to get memory info");
                    println!("Total memory: {} KB", mem_info.total);
                    println!("Free memory: {} KB", mem_info.free);
                    println!("Available memory: {} KB", mem_info.avail);
                    println!("Memory usage: {:.2}% KB", 100.0 * (mem_info.total - mem_info.free) as f64 / mem_info.total as f64);
                    prev_command = None;
                }
                "compare" => {
                    let args: Vec<&str> = args.collect();
                    if args.len() == 2 {
                        if permissions::check_permissions(args[0], false).unwrap_or(false) && permissions::check_permissions(args[1], false).unwrap_or(false) {
                            if std::fs::metadata(&args[0]).unwrap().is_dir() || std::fs::metadata(&args[1]).unwrap().is_dir() {
                                println!("{}", "compare command does not support directories!".red());
                            } else {
                                match compare::compare_files(&args[0], &args[1]) {
                                    Ok(true) => println!("Files are identical"),
                                    Ok(false) => println!("Files are not identical"),
                                    Err(e) => println!("Failed to compare files: {}", e),
                                }
                            }
                        } else {
                            println!("Permission denied");
                        }
                    } else {
                        println!("{}", "compare command requires 2 arguments!".red());
                    }
                    prev_command = None;
                }
                "dirsize" => {
                    let args: Vec<&str> = args.collect();
                    if args.len() == 1 {
                        if permissions::check_permissions(args[0], false).unwrap_or(false) {
                            dirsize::dirsize(args);
                        } else {
                            println!("Permission denied");
                        }
                    } else {
                        println!("dirsize command requires 1 argument!");
                    }
                    prev_command = None;
                }
                "tree" => {
                    let args: Vec<&str> = args.collect();
                    if args.len() == 1 {
                        if permissions::check_permissions(args[0], false).unwrap_or(false) {
                            let _ = tree::tree(&args[0]);
                        } else {
                            println!("Permission denied");
                        }
                    } else {
                        println!("{}", "tree command requires 1 argument!".red());
                    }
                    prev_command = None;
                }
                "diskinfo" => {
                    diskinfo::print_disk_info();
                    prev_command = None;
                }
                command => {
                    prev_command =
                        commands::execute_command(prev_command, command, args, &mut commands);
                    if prev_command.is_none() {
                        println!("{}", "Command execution failed!".red());
                    }
                }
            }
        }

        if let Some(mut final_command) = prev_command {
            final_command.wait().unwrap();
        }
    }
}