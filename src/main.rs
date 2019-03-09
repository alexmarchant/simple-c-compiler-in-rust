mod lexer;
mod parser;
mod generator;

extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use std::process::Command;
use std::io;
use clap::{Arg, App};

fn main() {
    let matches = App::new("acc")
                      .arg(Arg::with_name("debug")
                           .short("d")
                           .help("Debug mode"))
                      .arg(Arg::with_name("INPUT")
                           .help("Sets the input file to use")
                           .required(true)
                           .index(1))
                      .get_matches();
    let file_name = matches.value_of("INPUT").unwrap().to_string();
    let debug = matches.is_present("debug");
    let contents = read_file(&file_name);
    let tokens = lexer::parse_tokens(contents);
    let program = parser::parse_program(tokens);
    if debug {
        println!("-----AST-----");
        println!("{:#?}", program);
    }
    match program {
        Ok(program) => {
            let assembly_file_name = file_name.replace(".c", ".s");
            let assembly = generator::program_asm(program);
            if debug {
                println!("");
                println!("-----ASM-----");
                println!("{}", assembly);
            }
            write_file(&assembly_file_name, assembly);

            let executable_file_name = file_name.replace(".c", "");
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("gcc {} -o {}", assembly_file_name, executable_file_name))
                .output()
                .expect("failed to execute process");

            if debug {
                println!("");
                println!("-----GCC status-----");
                println!("status: {}", output.status);
            }
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            assert!(output.status.success());

            std::fs::remove_file(assembly_file_name).expect("Unable to delete assembly file");
        },
        Err(err) => println!("{}", err),
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
