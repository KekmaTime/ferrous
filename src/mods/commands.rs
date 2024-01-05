use std::env;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::str::SplitWhitespace;
use std::iter::Peekable;
use std::str::Split;
use std::iter::Map;
use which::which;
use colored::*;
    
type TrimFn = fn(&str) -> &str;

pub fn execute_command(prev_command: Option<Child>, command: &str, args: SplitWhitespace, commands: &mut Peekable<Map<Split<'_, &str>, TrimFn>>) -> Option<Child> {
    if which(command).is_err() {
        eprintln!("{}", format!("Unknown command: {}", command).red());
        return None;
    }

    let stdin = prev_command.map_or(Stdio::inherit(), |output: Child| {
        Stdio::from(output.stdout.expect("Failed to unwrap stdout"))
    });
    let stdout = if commands.peek().is_some() {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };

    let output = Command::new(command)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn();

    match output {
        Ok(output) => Some(output),
        Err(e) => {
            eprintln!("{}", format!("{}", e).red());
            None
        }
    }
}

pub fn change_directory(args: SplitWhitespace) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", format!("{}", e).red());
    }
}