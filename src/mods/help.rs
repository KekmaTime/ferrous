use colored::*;

pub fn print_help() {
    println!("{}", "Ferrous, version 1.0.0".green());
    println!("{}", "These shell commands are defined internally. Type `help' to see this list.".blue());
    println!("{}", "Type `help name' to find out more about the function `name'.".blue());
    println!("{}", "Use `info' to find out more about commands not in this list.".blue());
    println!();
    println!("{}", "A star (*) next to a name means that the command is disabled.".red());
    println!();
    println!("{}", " Almost all Unix-based commands should work for example:".yellow());
    println!("{}", " cd <directory> - Change the current directory to <directory>".yellow());
    println!("{}", "ls - List the contents of the current directory".yellow());
    println!("{}", "pwd - Print the current working directory".yellow());
    println!("{}", "help - Display helpful information about builtin commands".yellow());
    println!("{}", "echo - Display a line of text".yellow());
    println!("{}", "clear - Clear the terminal screen".yellow());
    println!("{}", " exit - Exit the shell".yellow());
    println!("{}", "cat - Concatenate files and print on the standard output".yellow());
    println!("{}", "mkdir - Create a directory".yellow());
    println!("{}", "rm - Remove files or directories".yellow());
    println!("{}", "touch - Change file timestamps".yellow());
    println!("{}", "cp - Copy files and directories".yellow());
    println!("{}", "mv - Move (rename) files".yellow());
    println!("{}", "rmdir - Remove empty directories".yellow());
}