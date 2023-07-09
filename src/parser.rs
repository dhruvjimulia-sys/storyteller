use chumsky::prelude::{*, text::Character};
use crate::types::LexerOutput;
pub mod ast;

fn statement_parser() -> impl Parser<char, ast::Statement, Error = Simple<char>> {
    let inline_whitespace = filter(|c: &char| c.is_inline_whitespace()).repeated();

    let quote = just("\"").padded_by(inline_whitespace);

    let to_be = text::keyword("was")
        .or(text::keyword("were"))
        .or(text::keyword("is"))
        .or(text::keyword("are"))
        .padded_by(inline_whitespace);

    let as_keyword = text::keyword("as").padded_by(inline_whitespace);
    let felt_keyword = text::keyword("felt").padded_by(inline_whitespace);
    let positive_adjective = text::keyword("good").padded_by(inline_whitespace);
    let negative_adjective = text::keyword("bad").padded_by(inline_whitespace);
    let said_keyword = text::keyword("said").padded_by(inline_whitespace);
    let comma = just(",").padded_by(inline_whitespace);
    let inner_quote = none_of("\"").repeated();
    let adverb_keyword = just("earnestly").padded_by(inline_whitespace);

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
        take_until(felt_keyword.clone())
        .then_ignore(as_keyword.clone())
        .then_ignore(positive_adjective)
        .then_ignore(as_keyword.clone())
        .then(idents)
        .map(|((a, _), b)| ast::Statement::AddStatement(
            ast::Variable(a.into_iter().collect()),
            ast::VariableOrNumberLiteral(b)
        ));
    
    let subtraction_statement =
        take_until(felt_keyword)
        .then_ignore(as_keyword.clone())
        .then_ignore(negative_adjective)
        .then_ignore(as_keyword)
        .then(idents)
        .map(|((a, _), b)| ast::Statement::SubStatement(
            ast::Variable(a.into_iter().collect()),
            ast::VariableOrNumberLiteral(b)
        ));

    let print_number_statement =
        quote.ignore_then(inner_quote.clone().ignore_then(quote.ignore_then(take_until(said_keyword.clone()))))
        .or(take_until(said_keyword.clone()).then_ignore(comma.or_not()).then_ignore(quote.then_ignore(inner_quote.clone().then_ignore(quote))))
        .map(|(number, _)| ast::Statement::PrintNumberStatement(
            ast::Variable(number.into_iter().collect())
        ));
        
    let print_character_statement =
        quote.ignore_then(inner_quote.clone().ignore_then(quote
            .ignore_then(comma.clone().or_not().ignore_then(take_until(said_keyword.clone()))).then_ignore(adverb_keyword))
        )
        .or(
            take_until(said_keyword).then_ignore(adverb_keyword).then_ignore(comma.or_not()).then_ignore(quote.then_ignore(inner_quote.then_ignore(quote)))
        )
        .map(|(number, _)| ast::Statement::PrintCharacterStatement(
            ast::Variable(number.into_iter().collect())
        ));

    let statement =
        print_character_statement
        .or(print_number_statement)
        .or(assignment_statement)
        .or(addition_statement)
        .or(subtraction_statement);
        

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