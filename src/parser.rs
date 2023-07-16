use chumsky::prelude::*;
use chumsky::primitive::Just;
use crate::types::{LexerOutput, LexerToken};
pub mod ast;

fn statement_parser() -> impl Parser<LexerToken, ast::Statement, Error = Simple<LexerToken>> {
    fn keyword(keyword: &str) -> Just<LexerToken, LexerToken, Simple<LexerToken>> {
        just(LexerToken::Text(keyword.to_string()))
    }

    fn lexer_tokens_to_name(vec: Vec<LexerToken>) -> String {
        vec.into_iter().map(|token| match token {
            LexerToken::Text(s) => s,
            _ => "".to_string()
        }).collect::<Vec<_>>().join(" ")
    }

    let to_be = keyword("was")
        .or(keyword("were"))
        .or(keyword("is"))
        .or(keyword("are"));

    let positive_adjective = keyword("good");
    let negative_adjective = keyword("bad");
    let said_keyword = keyword("said");
    let adverb_keyword = filter(|token: &LexerToken| match token {
        LexerToken::Text(s) => s.ends_with("ly"),
        _ => false
    });

    let assignment_statement =
        take_until(to_be)
        .then(take_until(end()))
        .map(|((a, _), (b, _))| ast::Statement::AssignmentStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let addition_statement =
        take_until(keyword("felt"))
        .then_ignore(keyword("as"))
        .then_ignore(positive_adjective)
        .then_ignore(keyword("as"))
        .then(take_until(end()))
        .map(|((a, _), (b, _))| ast::Statement::AddStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let subtraction_statement =
        take_until(keyword("felt"))
        .then_ignore(keyword("as"))
        .then_ignore(negative_adjective)
        .then_ignore(keyword("as"))
        .then(take_until(end()))
        .map(|((a, _), (b, _))| ast::Statement::SubStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let quote = just(LexerToken::Quote);
    let inner_quote = none_of(vec![LexerToken::Quote]).repeated();

    let print_number_statement =
        quote.clone()
        .ignore_then(inner_quote.clone()
        .ignore_then(quote.clone().ignore_then(take_until(said_keyword.clone()))))
        .map(|(number, _)| ast::Statement::PrintNumberStatement(
            ast::Variable(lexer_tokens_to_name(number)))
        );

    let print_character_statement =
        quote.clone()
        .ignore_then(inner_quote.clone()
        .ignore_then(quote.clone()
        .ignore_then(take_until(said_keyword.clone()))
        .then_ignore(adverb_keyword.clone())))
        .map(|(number, _)| ast::Statement::PrintCharacterStatement(
            ast::Variable(lexer_tokens_to_name(number))
        ));

    let statement =
        print_character_statement
        .or(print_number_statement)
        .or(assignment_statement)
        .or(addition_statement)
        .or(subtraction_statement);

    statement
}

fn statement_block_parser() -> impl Parser<LexerToken, ast::Block, Error = Simple<LexerToken>> {
    let block_parser =
        just(LexerToken::Period).not().repeated()
        .separated_by(just(LexerToken::Period))
        .allow_trailing()
        .map(|statements| {
            ast::Block(statements.into_iter()
            .filter(|statement| !statement.is_empty())
            .map(|statement| statement_parser().parse(statement).unwrap()).collect())
        });
    block_parser
}

pub fn parse_program(input: LexerOutput) -> ast::Program {
    ast::Program(input.0.iter().map(|block| {
        let parsed_block = match statement_block_parser().parse(block.0.clone()) {
            Ok(s) => s,
            Err(_) => panic!("Failed to parse statement")
        };
        parsed_block
    }).collect())
}