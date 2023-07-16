use chumsky::prelude::{*, text::Character};
use crate::types::{LexerOutput, LexerStatement, LexerBlock, LexerToken};

pub fn lexer() -> impl Parser<char, LexerOutput, Error = Simple<char>> {
    let inline_whitespace = filter(|c: &char| c.is_inline_whitespace()).repeated();
    let newline = just('\n');

    let lexer_token =
        text::ident::<_, Simple<char>>().padded_by(inline_whitespace).map(|s| LexerToken::Text(s.to_string()))
        .or(just(",").padded_by(inline_whitespace).map(|_| LexerToken::Comma))
        .or(just("\"").padded_by(inline_whitespace).map(|_| LexerToken::Quote));

    let lexer_statement =
        newline.not().rewind()
        .ignore_then(none_of(".").repeated())
        .then_ignore(just("."))
        .padded_by(inline_whitespace)
        .map(move |vec| LexerStatement(lexer_token.repeated().parse(vec).unwrap()));

    let block = lexer_statement.repeated()
        .map(|statements| LexerBlock(statements));

    let lexer_program = block.separated_by(newline.repeated().at_least(1))
        .then_ignore(end())
        .map(|blocks| LexerOutput(blocks));

    lexer_program
}