mod lexer;
mod parser;
mod generator;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::process::Command;

fn main() {
    let file_name = env::args().nth(1).expect("Missing argument");
    let contents = read_file(&file_name);
    let tokens = lexer::parse_tokens(contents);
    let program = parser::parse_program(&tokens);
    match program {
        Ok(program) => {
            let assembly = generator::program_asm(program);
            let assembly_file_name = file_name.replace(".c", ".s");
            write_file(&assembly_file_name, assembly);
            let executable_file_name = file_name.replace(".c", "");
            Command::new("sh")
                .arg("-c")
                .arg(format!("gcc -m32 {} -o {}", assembly_file_name, executable_file_name))
                .output()
                .expect("failed to execute process");
            std::fs::remove_file(assembly_file_name).expect("Unable to delete assembly file");
        },
        Err(error) => println!("{:?}", error),
    }
}

fn read_file(file_name: &String) -> String {
    let mut file = File::open(file_name).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    return contents;
}

fn write_file(file_name: &String, contents: String) {
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(contents.as_bytes()).expect("Unable to write to file");
}
