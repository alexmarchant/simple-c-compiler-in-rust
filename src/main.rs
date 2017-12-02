mod lexer;
mod parser;
mod generator;

use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let file_name = env::args().nth(1).expect("Missing argument");
    let contents = read_file(file_name);
    let tokens = lexer::parse_tokens(contents);
    let program = parser::parse_program(&tokens);
    match program {
        Ok(program) => {
            let assembly = generator::program_asm(program);
            println!("{}", assembly);
        },
        Err(error) => println!("{:?}", error),
    }
}

fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    return contents;
}

