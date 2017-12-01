mod lexer;
mod parser;

use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let command = env::args().nth(1).expect("Missing argument");
    match command.as_ref() {
        "lexer" => lexer_command(),
        _ => println!("Command not supported"),
    }
}

fn lexer_command() {
    let file_name = env::args().nth(2).expect("Missing argument");
    let contents = read_file(file_name);
    let tokens = lexer::parse_tokens(contents);
    // println!("{:?}", tokens);
    match parser::parse_program(&tokens) {
        Ok(program) => println!("{:?}", program),
        Err(error) => println!("{:?}", error),
    }
}

fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    return contents;
}

