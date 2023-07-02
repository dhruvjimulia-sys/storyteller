use chumsky::prelude::{*, text::Character};
use crate::types::LexerOutput;
mod ast;

fn statement_parser() -> impl Parser<char, ast::Statement, Error = Simple<char>> {
    let inline_whitespace = filter(|c: &char| c.is_inline_whitespace()).repeated();

    let to_be = text::keyword("was")
        .or(text::keyword("were"))
        .or(text::keyword("is"))
        .or(text::keyword("are"))
        .padded_by(inline_whitespace);

    let as_keyword = text::keyword("as").padded_by(inline_whitespace);
    let felt_keyword = text::keyword("felt").padded_by(inline_whitespace);
    let positive_adjective = text::keyword("good").padded_by(inline_whitespace);    

    let ident = 
        text::ident()
        .padded_by(inline_whitespace);

    let idents =
        ident
        .repeated()
        .map(|vec| vec.join(" "))
        .padded_by(inline_whitespace);

    let assignment_statement =
        take_until(text::keyword(".").or(to_be))
        .then(idents)
        .map(|((a, _), b)| ast::Statement::AssignmentStatement(
            ast::Variable(a.into_iter().collect()),
            ast::VariableOrNumberLiteral(b)
        ));

    let addition_statement =
        take_until(felt_keyword)
        .then_ignore(as_keyword.clone())
        .then_ignore(positive_adjective)
        .then_ignore(as_keyword)
        .then(idents)
        .map(|((a, _), b)| ast::Statement::AddStatement(
            ast::Variable(a.into_iter().collect()),
            ast::VariableOrNumberLiteral(b)
        ));

    let statement = assignment_statement
        .or(addition_statement);

    statement
}

pub fn parse_program(input: LexerOutput) -> ast::Program {
    ast::Program(input.0.iter().map(|block| {
        let statements = block.0.iter().map(|statement| {
            let parsed_statement = match statement_parser().parse(statement.0.as_str()) {
                Ok(s) => s,
                Err(_) => panic!("Failed to parse statement")
            };
            parsed_statement
        });
        ast::Block(statements.collect())
    }).collect())
}