use chumsky::prelude::*;
mod parser;

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
    match parser::parser().parse(file_contents) {
        Ok(ast) => println!("{:?}", ast),
        Err(e) => println!("ParseError"),
    }
}
