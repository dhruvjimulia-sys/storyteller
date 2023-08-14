use chumsky::Parser;
use std::io::Write;
mod parser;
mod lexer;
mod preprocessor;
mod unit_tests;
mod ast_to_ir;
mod variable_extractor;
mod interpreter;
mod ir_to_c;
#[macro_use]
pub mod errors;
mod keyword_defs;
use errors::compiler_errors;
use std::collections::HashSet;

fn convert_file_contents_to_ir_and_variable_set(file_name: String) -> (Vec<ast_to_ir::ir::Instruction>, HashSet<ast_to_ir::ir::Variable>) {
    let file_contents = match std::fs::read_to_string(file_name) {
        Ok(file_contents) => file_contents,
        Err(_) => { compiler_errors::file_not_found_error().display(); return (vec!(), HashSet::new()); }
    };
    let lexer_output = lexer::lexer().parse(file_contents).expect("Lexer Error");
    let preprocessed_lexer_output = preprocessor::preprocess(lexer_output);
    let keywords = keyword_defs::get_keyword_defs();
    let ast = match parser::parse_program(preprocessed_lexer_output, keywords) {
        Ok(ast) => ast,
        Err(errors) => { errors.into_iter().for_each(|err| err.display()); return (vec!(), HashSet::new()); }
    };
    let variables = variable_extractor::get_variables(&ast);
    (ast_to_ir::convert_ast_to_ir(ast, &variables), variables)
}

pub fn interpret(file_name: String, input_stream: &mut dyn std::io::BufRead, output_stream: &mut dyn Write) {
    let (ir, _) = convert_file_contents_to_ir_and_variable_set(file_name);
    interpreter::interpret(ir, input_stream, output_stream);
}

pub fn compile(input_file_name: String, output_file_name: String) {
    let (ir, variables) = convert_file_contents_to_ir_and_variable_set(input_file_name);
    let mut output_file = match std::fs::File::create(output_file_name) {
        Ok(file) => file,
        Err(_) => { compiler_errors::file_not_found_error().display(); return; }
    };
    let c_code = ir_to_c::convert_ir_to_c(ir, variables);
    output_file.write_all(c_code.as_bytes()).expect("Error writing to output file");
}