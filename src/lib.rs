use chumsky::Parser;
use std::io::Write;
pub mod parser;
mod lexer;
mod preprocessor;
#[cfg(test)]
mod unit_tests;
mod ast_to_ir;
mod interpreter;

pub fn interpret(file_name: String, input_stream: &mut dyn std::io::BufRead, output_stream: &mut dyn Write) {
    let file_contents = match std::fs::read_to_string(file_name) {
        Ok(file_contents) => file_contents,
        Err(e) => {
            println!("IOError: {}", e);
            return;
        }
    };
    let ast = match lexer::lexer().parse(file_contents) {
        Ok(lexer_output) => parser::parse_program(preprocessor::preprocess(lexer_output)),
        Err(errors) => {
            println!("ParseError {:#?}", errors);
            return;
        }
    };
    let ir = ast_to_ir::convert_ast_to_ir(ast);
    interpreter::interpret(ir, input_stream, output_stream);
}
