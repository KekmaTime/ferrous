use std::io::{self, Write};
use std::str::SplitWhitespace;
use colored::*;

pub fn get_input() -> String {
    print!("{}", "> ".green());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn parse_input(input: &str) -> (&str, SplitWhitespace) {
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;
    (command, args)
}