use crate::parser;
use parser::ast;
use crate::lexer;
use crate::preprocessor;
use crate::keyword_defs;
use chumsky::prelude::*;

pub fn parse_program_string(program_string: &str) -> ast::Program {
    let lexer_output = preprocessor::preprocess(lexer::lexer().parse(program_string).unwrap());
    let keywords = keyword_defs::defs();
    let ast = match parser::parse_program(lexer_output, keywords) {
        Ok(ast) => ast,
        Err(_) => { panic!("Parse program failed") }
    };
    ast
}