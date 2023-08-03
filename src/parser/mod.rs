use chumsky::prelude::*;
use chumsky::primitive::Just;
use itertools::Itertools;
use crate::lexer::lexer_types::{LexerOutput, LexerToken};
use crate::errors::Error;
use crate::errors::compiler_errors;
pub mod ast;

fn statement_parser() -> impl Parser<LexerToken, ast::Statement, Error = Simple<LexerToken>> {
    let to_be =
        &["was", "were", "is", "are", "wanted to be like", "wants to be like", "wanted to be like"];
    let positive_adjective =
        &["good", "great", "awesome", "amazing", "fantastic", "wonderful", "incredible", "nice", "cool", "happy", "joyful", "joyous", "glad", "delighted", "pleased", "satisfied", "content", "cheerful", "merry", "jolly", "jovial", "gleeful", "carefree", "sunny", "elated", "exhilarated", "ecstatic", "euphoric", "overjoyed", "exultant", "rapturous", "blissful", "radiant", "thrilled", "ravished"];
    let negative_adjective = 
        &["bad", "terrible", "awful", "horrible", "dreadful", "unpleasant", "unlucky", "displeased", "miserable", "sad", "sorrowful", "dejected", "regretful", "depressed", "downcast", "despondent", "disconsolate", "desolate", "glum", "gloomy", "melancholic", "mournful", "forlorn", "crestfallen", "broken-hearted", "heartbroken", "grief-stricken", "disheartened", "dismayed", "dispirited", "discouraged", "hopeless"];
    let said_keywords =
        &["said", "stated", "exclaimed", "whispered", "shouted", "mumbled", "replied", "responded", "declared", "announced", "asserted", "acknowledged", "conveyed", "uttered", "ventured", "suggested", "disclosed", "protested", "objected", "interjected", "speculated", "greeted", "quoted", "noted", "mentioned", "alledged", "insisted", "confessed", "recited", "pleaded", "concluded", "inquired", "muttered"];
    let goto_keywords =
        &["go to", "goes to", "went to", "gone to", "going to"];

    fn keywords(keywords: &[&str]) -> impl Parser<LexerToken, LexerToken, Error = Simple<LexerToken>> {
        fn full_keyword(full_keyword: &str) -> impl Parser<LexerToken, LexerToken, Error = Simple<LexerToken>> {
            let full_split = full_keyword.split(" ").filter(|key| key.len() != 0).collect::<Vec<_>>();
            let mut full_keyword_result: Box<dyn Parser<LexerToken, LexerToken, Error = Simple<LexerToken>>> = Box::new(keyword(full_split[0]));
            for i in 1..full_split.len() {
                full_keyword_result = Box::new(full_keyword_result.then_ignore(keyword(full_split[i])));
            }
            full_keyword_result
        }

        let keywords = keywords.iter().unique().collect::<Vec<_>>();
        let mut result: Box<dyn Parser<LexerToken, LexerToken, Error = Simple<LexerToken>>> = Box::new(full_keyword(keywords[0]));
        for i in 1..keywords.len() {
            result = Box::new(result.or(full_keyword(keywords[i])));
        }
        result
    }

    fn keyword(keyword: &str) -> Just<LexerToken, LexerToken, Simple<LexerToken>> {
        just(LexerToken::Text(keyword.to_string()))
    }

    fn lexer_tokens_to_name(vec: Vec<LexerToken>) -> String {
        vec.into_iter()
        .map(|token| match token {
            LexerToken::Text(s) => s,
            _ => "".to_string()
        }).collect::<Vec<_>>().join(" ")
    }

    let adverb_keyword = filter(|token: &LexerToken| match token {
        LexerToken::Text(s) => s.ends_with("ly"),
        _ => false
    });

    let assignment_statement =
        take_until(keywords(to_be))
        .then(filter(|token| match token {
            LexerToken::Text(_) => true,
            _ => false
        }).repeated())
        .then_ignore(end())
        .map(|((a, _), b)| ast::Statement::AssignmentStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let addition_statement =
        take_until(keyword("felt"))
        .then_ignore(keyword("as"))
        .then_ignore(keywords(positive_adjective))
        .then_ignore(keyword("as"))
        .then(take_until(end()))
        .map(|((a, _), (b, _))| ast::Statement::AddStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let subtraction_statement =
        take_until(keyword("felt"))
        .then_ignore(keyword("as"))
        .then_ignore(keywords(negative_adjective))
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
        .ignore_then(quote.clone().ignore_then(take_until(keywords(said_keywords)))))
        .map(|(number, _)| ast::Statement::PrintNumberStatement(
            ast::Variable(lexer_tokens_to_name(number)))
        );

    let print_character_statement =
        quote.clone()
        .ignore_then(inner_quote.clone()
        .ignore_then(quote.clone()
        .ignore_then(take_until(keywords(said_keywords)))
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

    let goto_statement =
        take_until(keywords(goto_keywords))
        .ignore_then(take_until(end()))
        .map(|(name, _)| ast::Statement::GotoStatement(
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(name))
        ));

    let exit_statement =
        take_until(keyword("end"))
        .map(|_| ast::Statement::ExitStatement);

    let comment =
        take_until(end())
        .map(|_| ast::Statement::Comment);

    fn if_statement(statement_parser: Recursive<'_, LexerToken, ast::Statement, Simple<LexerToken>>) -> impl Parser<LexerToken, ast::Statement, Error = Simple<LexerToken>> + '_ {
        let positive_comparative_adjective =
            keyword("better").or(keyword("greater")).or(keyword("more"));
        let negative_comparative_adjective =
            keyword("worse").or(keyword("less"));
        let comma = just(LexerToken::Comma);
        let condition =
            take_until(keyword("is").then(positive_comparative_adjective).then(keyword("than")))
            .then(take_until(end()))
            .map(|((lhs, _), (rhs, _))| ast::Condition::GreaterThan(
                ast::VariableOrNumberLiteral(lexer_tokens_to_name(lhs)),
                ast::VariableOrNumberLiteral(lexer_tokens_to_name(rhs))
            ))
            .or(
                take_until(keyword("is").then(negative_comparative_adjective).then(keyword("than")))
                .then(take_until(end()))
                .map(|((lhs, _), (rhs, _))| ast::Condition::LessThan(
                    ast::VariableOrNumberLiteral(lexer_tokens_to_name(lhs)),
                    ast::VariableOrNumberLiteral(lexer_tokens_to_name(rhs))
                ))
            );

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
        .or(comment)
    });

    statement
}

fn statement_block_parser() -> impl Parser<LexerToken, ast::Block, Error = Simple<LexerToken>> {
    let sentence_end_punctuation = just(LexerToken::Period)
        .or(just(LexerToken::QuestionMark))
        .or(just(LexerToken::ExclamationMark));

    let block_parser =
        (sentence_end_punctuation.clone().not().repeated()
            .then_ignore(sentence_end_punctuation)
        ).repeated().at_least(1)
        .or(
            end().map(|_| vec!())
        )
        .map(|statements| {
            ast::Block(statements.into_iter()
            .filter(|statement| !statement.is_empty())
            .map(|statement| statement_parser().parse(statement).unwrap()).collect())
        });
    block_parser
}

pub fn parse_program(input: LexerOutput) -> Result<ast::Program, Vec<Error<'static>>> {
    let mut errors = vec![];
    let program = ast::Program(input.0.into_iter().map(|block| {
        let parsed_block = match statement_block_parser().parse(block.0.clone()) {
            Ok(s) => s,
            Err(_) =>  {
                errors.push(compiler_errors::UNFINISHED_THOUGHT_ERROR);
                ast::Block(vec!())
            }
        };
        parsed_block
    }).collect());
    if errors.is_empty() {
        Ok(program)
    } else {
        Err(errors)
    }
}