use chumsky::{prelude::*, text::Character};
mod ast;

pub fn parser() -> impl Parser<char, ast::Program, Error = Simple<char>> {
    /*
    filter(|c: &char| c.is_ascii_digit())
    .map(|c| ast::Statement::Num { value: c.to_digit(10).unwrap() as i32 })
    .padded_by(filter(|c: &char| c.is_whitespace()).repeated())
    .then_ignore(end())
    */

    let block = text::int(10)
        .map(|s: String| ast::Block(vec!(ast::Statement::Num(s.parse().unwrap()))))
        .padded_by(filter(|c: &char| c.is_inline_whitespace()).repeated());

    let blocks = block.separated_by(just("\n\n"))
        .map(|blocks| ast::Program(blocks));

    blocks
}