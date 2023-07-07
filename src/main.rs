use chumsky::prelude::*;
pub mod parser;
mod lexer;
mod types;
#[cfg(test)]
mod unit_tests;

fn main() {
    let file_name = match std::env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            println!("IllegalArgumentError: Use cargo run -- <file_name>");
            return;
        }
    };
    let file_contents = match std::fs::read_to_string(file_name) {
        Ok(file_contents) => file_contents,
        Err(e) => {
            println!("IOError: {}", e);
            return;
        }
    };
    match lexer::lexer().parse(file_contents) {
        Ok(lexer_output) => println!("{:?}", parser::parse_program(lexer_output)),
        Err(errors) => println!("ParseError {:#?}", errors)
    }
}
