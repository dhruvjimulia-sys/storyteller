use chumsky::prelude::{*, text::Character};
use crate::types::{LexerOutput, LexerStatement, LexerBlock};

pub fn lexer() -> impl Parser<char, LexerOutput, Error = Simple<char>> {
    let inline_whitespace = filter(|c: &char| c.is_inline_whitespace()).repeated();
    let newline = just('\n');

    let lexer_statement =
        newline.not().rewind()
        .ignore_then(none_of(".").repeated())
        .then_ignore(just("."))
        .padded_by(inline_whitespace)
        .map(|vec| LexerStatement(vec.into_iter().collect()));

    let block = lexer_statement.repeated()
        .map(|statements| LexerBlock(statements));

    let lexer_program = block.separated_by(newline.repeated().at_least(1))
        .then_ignore(end())
        .map(|blocks| LexerOutput(blocks));

    lexer_program
}