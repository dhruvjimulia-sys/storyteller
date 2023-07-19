use chumsky::prelude::*;
use chumsky::primitive::Just;
use crate::types::{LexerOutput, LexerToken};
use std::collections::HashSet;
pub mod ast;

fn statement_parser() -> impl Parser<LexerToken, ast::Statement, Error = Simple<LexerToken>> {
    fn keyword(keyword: &str) -> Just<LexerToken, LexerToken, Simple<LexerToken>> {
        just(LexerToken::Text(keyword.to_string()))
    }

    fn lexer_tokens_to_name(vec: Vec<LexerToken>) -> String {
        let articles_and_possessives = HashSet::from(["a", "an", "the", "my", "your", "his", "her", "its", "our", "their"]);

        vec.into_iter()
        .filter(|token| match token {
            LexerToken::Text(s) => !articles_and_possessives.contains(s.as_str()),
            _ => true
        })
        .map(|token| match token {
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
    let comma = just(LexerToken::Comma);
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

    let input_statement =
        take_until(keyword("looked").or(keyword("looks")))
        .then_ignore(keyword("up"))
        .then_ignore(keyword("to"))
        .then_ignore(keyword("the"))
        .then_ignore(keyword("skies"))
        .then_ignore(keyword("beyond"))
        .then_ignore(comma)
        .then_ignore(keyword("waiting"))
        .then_ignore(keyword("for"))
        .then_ignore(keyword("an"))
        .then_ignore(keyword("answer"))
        .map(|(name, _)| ast::Statement::InputStatement(
            ast::Variable(lexer_tokens_to_name(name))
        ));

    let goto_keywords =
        keyword("go").then(keyword("to"))
        .or(keyword("goes").then(keyword("to")))
        .or(keyword("went").then(keyword("to")))
        .or(keyword("gone").then(keyword("to")))
        .or(keyword("going").then(keyword("to")));

    let goto_statement =
        take_until(goto_keywords)
        .ignore_then(take_until(end()))
        .map(|(name, _)| ast::Statement::GotoStatement(
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(name))
        ));

    let exit_statement =
        take_until(keyword("end"))
        .map(|_| ast::Statement::ExitStatement);

    fn if_statement(statement_parser: Recursive<'_, LexerToken, ast::Statement, Simple<LexerToken>>) -> impl Parser<LexerToken, ast::Statement, Error = Simple<LexerToken>> + '_ {
        let comma = just(LexerToken::Comma);
        let condition =
            take_until(keyword("is").then(keyword("better")).then(keyword("than")))
            .then(take_until(end()))
            .map(|((lhs, _), (rhs, _))| ast::Condition::LessThan(
                ast::VariableOrNumberLiteral(lexer_tokens_to_name(lhs)),
                ast::VariableOrNumberLiteral(lexer_tokens_to_name(rhs))
            ));

        keyword("if")
        .ignore_then(take_until(comma))
        .then_ignore(keyword("then"))
        .then(take_until(end()))
        .map(move |((condition_tokens, _), (consequence, _))| ast::Statement::IfStatement(
            condition.parse(condition_tokens).unwrap(),
            Box::new(statement_parser.parse(consequence).unwrap())
        ))
    }

    let statement = recursive(|statement| {
        if_statement(statement)
        .or(input_statement)
        .or(print_character_statement)
        .or(print_number_statement)
        .or(assignment_statement)
        .or(addition_statement)
        .or(subtraction_statement)
        .or(goto_statement)
        .or(exit_statement)
    });

    statement
}

fn statement_block_parser() -> impl Parser<LexerToken, ast::Block, Error = Simple<LexerToken>> {
    let sentence_end_punctuation = just(LexerToken::Period)
        .or(just(LexerToken::QuestionMark))
        .or(just(LexerToken::ExclamationMark));

    let block_parser =
        sentence_end_punctuation.clone().not().repeated()
        .separated_by(sentence_end_punctuation)
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