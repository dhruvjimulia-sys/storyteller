use chumsky::prelude::{*, text::Character};
use crate::types::{LexerOutput, LexerBlock, LexerToken};

pub fn lexer() -> impl Parser<char, LexerOutput, Error = Simple<char>> {
    let inline_whitespace = filter(|c: &char| c.is_inline_whitespace()).repeated();
    let newline = just('\n');

    let lexer_token =
        text::ident::<_, Simple<char>>().map(|s| LexerToken::Text(s.to_string()))
        .or(just(",").map(|_| LexerToken::Comma))
        .or(just("\"").map(|_| LexerToken::Quote))
        .or(just(".").map(|_| LexerToken::Period));

    let block = 
        newline.not().rewind()
        .ignore_then(lexer_token.padded_by(inline_whitespace).repeated())
        .map(|tokens| LexerBlock(tokens));

    let lexer_program = block.separated_by(newline.repeated().at_least(1))
        .allow_trailing()
        .then_ignore(end())
        .map(|blocks| LexerOutput(blocks));

    lexer_program
}