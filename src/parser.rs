use chumsky::prelude::{*, text::Character};
mod ast;

pub fn parser() -> impl Parser<char, ast::Program, Error = Simple<char>> {
    let inline_whitespace = filter(|c: &char| c.is_inline_whitespace()).repeated();
    let newline = just('\n');

    let to_be = text::keyword("was")
        .or(text::keyword("were"))
        .or(text::keyword("is"))
        .or(text::keyword("are"))
        .padded_by(inline_whitespace);

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

    let as_keyword = text::keyword("as").padded_by(inline_whitespace);
    let felt_keyword = text::keyword("felt").padded_by(inline_whitespace);
    let positive_adjective = text::keyword("good").padded_by(inline_whitespace);

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

    let statement =
        newline.not().rewind()
        .ignore_then(inline_whitespace.ignore_then(
            addition_statement.or(assignment_statement)
        ))
        .then_ignore(just("."));

    let block = statement.repeated()
        .map(|statements| ast::Block(statements));

    let program = block.separated_by(newline.repeated().exactly(2))
        .then_ignore(end())
        .map(|blocks| ast::Program(blocks));

    program
}