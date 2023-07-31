use chumsky::Parser;
use std::io::Write;
pub mod parser;
mod lexer;
mod preprocessor;
mod unit_tests;
mod ast_to_ir;
mod interpreter;
#[macro_use]
mod compiler_errors;
use crate::compiler_errors::Error;

pub fn interpret(file_name: String, input_stream: &mut dyn std::io::BufRead, output_stream: &mut dyn Write) {
    let file_contents = match std::fs::read_to_string(file_name) {
        Ok(file_contents) => file_contents,
        Err(_) => { compiler_errors::IO_ERROR.display(); return; }
    };
    let lexer_output = lexer::lexer().parse(file_contents).expect("Lexer Error");
    let preprocessed_lexer_output = preprocessor::preprocess(lexer_output);
    let ast = match parser::parse_program(preprocessed_lexer_output) {
        Ok(ast) => ast,
        Err(errors) => { errors.into_iter().for_each(|err| err.display()); return; }
    };
    let ir = ast_to_ir::convert_ast_to_ir(ast);
    interpreter::interpret(ir, input_stream, output_stream);
}
