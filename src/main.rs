use chumsky::prelude::*;
pub mod parser;
mod lexer;
mod types;
#[cfg(test)]
mod unit_tests;
/*
fn main() {
    let file_name = match std::env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            println!("IllegalArgumentError: Use cargo run -- <file_name>");
            return;
        }
    };
    let file_contents = match std::fs::read_to_string(file_name) {
        Ok(file_contents) => file_contents,
        Err(e) => {
            println!("IOError: {}", e);
            return;
        }
    };
    match lexer::lexer().parse(file_contents) {
        Ok(lexer_output) => println!("{:?}", parser::parse_program(lexer_output)),
        Err(errors) => println!("ParseError {:#?}", errors)
    }
}
*/

fn main() {

    // let test_parser = take_until(just::<char, &str, Simple<char>>("ly").then_ignore(inline_whitespace));
    // let ascii_chars = filter::<_, _, Simple<char>>(|c: &char| {
    //     c.is_ascii_alphabetic()
    // });
    // let test_parser = ascii_chars.then_ignore(just::<char, &str, Simple<char>>("ly").then_ignore(inline_whitespace));


    // let test_parser: Just<LexerToken, LexerToken, Simple<LexerToken>> = just(LexerToken::Text("quick".to_string()));

    /* 
    fn keyword(keyword: &str) -> Just<LexerToken, LexerToken, Simple<LexerToken>> {
        just(LexerToken::Text(keyword.to_string()))
    }

    let quote = just(LexerToken::Quote);
    let comma = just(LexerToken::Comma);
    let inner_quote = none_of(vec![LexerToken::Quote]).repeated();

    let test_parser = take_until(keyword("said"))
        .then_ignore(keyword("earnestly"))
        .then_ignore(comma.or_not())
        .then_ignore(quote.clone()
        .then_ignore(inner_quote.then_ignore(quote)));

    let test_statement = LexerBlock(vec!(
        LexerToken::Text("Charlie".to_string()),
        LexerToken::Text("said".to_string()),
        LexerToken::Text("earnestly".to_string()),
        LexerToken::Comma,
        LexerToken::Quote, LexerToken::Text("I".to_string()),
        LexerToken::Text("am".to_string()),
        LexerToken::Text("a".to_string()),
        LexerToken::Text("wizard".to_string()),
        LexerToken::Quote));
    */

    let test_parser = any::<char, Simple<char>>().repeated().separated_by(just(","));

    println!("{:?}", test_parser.parse("d,e,f"));
}
