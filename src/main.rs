use chumsky::Parser;
pub mod parser;
mod lexer;
mod preprocessor;
#[cfg(test)]
mod unit_tests;
mod ast_to_ir;
mod interpreter;

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
    let ast = match lexer::lexer().parse(file_contents) {
        Ok(lexer_output) => parser::parse_program(preprocessor::preprocess(lexer_output)),
        Err(errors) => {
            println!("ParseError {:#?}", errors);
            return;
        }
    };
    let ir = ast_to_ir::convert_ast_to_ir(ast);
    interpreter::interpret(ir);
}


/*
fn main() {
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


    let test_parser = any::<char, Simple<char>>().repeated().separated_by(just(","));

    println!("{:?}", test_parser.parse("d,e,f"));
}
*/

// fn main() {
//     println!("{}", ast_to_ir::convert_poetic_literal_to_integer("a".to_string()));
//     // let result = BigUint::new(vec!(11));
//     // println!("{}", result.clone().to_string());
//     // let test = (result.rem(BigUint::new(vec!(10)))).to_u32_digits();
//     // for digit in test {
//     //     println!("{}", digit);
//     // }
// }
