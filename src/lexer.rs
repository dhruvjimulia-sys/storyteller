use chumsky::prelude::{*, text::Character};
mod types;

pub fn lexer() -> impl Parser<char, types::LexerOutput, Error = Simple<char>> {
    let inline_whitespace = filter(|c: &char| c.is_inline_whitespace()).repeated();
    let newline = just('\n');

    let lexer_statement =
        newline.not().rewind()
        .ignore_then(none_of(".").repeated())
        .then_ignore(just("."))
        .padded_by(inline_whitespace)
        .map(|vec| types::LexerStatement(vec.into_iter().collect()));

    let block = lexer_statement.repeated()
        .map(|statements| types::LexerBlock(statements));

    let lexer_program = block.separated_by(newline.repeated().at_least(1))
        .then_ignore(end())
        .map(|blocks| types::LexerOutput(blocks));

    lexer_program
}