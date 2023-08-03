use chumsky::prelude::*;
use chumsky::primitive::Just;
use crate::lexer::lexer_types::{LexerOutput, LexerToken};
use crate::errors::Error;
use crate::errors::compiler_errors;
pub mod ast;

fn statement_parser() -> impl Parser<LexerToken, ast::Statement, Error = Simple<LexerToken>> {
    let to_be = keyword("was").or(keyword("were")).or(keyword("is")).or(keyword("are"));
    let positive_adjective =
        keywords(&["good", "great", "awesome", "amazing", "fantastic", "wonderful", "excellent", "nice", "cool", "fun", "happy", "joyful", "joyous", "glad", "delighted", "pleased", "satisfied", "content", "cheerful", "merry", "jolly", "jovial", "jocular", "gleeful", "carefree", "untroubled", "sunny", "blithe", "elated", "exhilarated", "ecstatic", "euphoric", "overjoyed", "exultant", "rapturous", "blissful", "radiant", "thrilled", "ravished"]);
    let negative_adjective = 
        keyword("bad")
        .or(keyword("terrible"));
    let said_keyword = keyword("said")
        .or(keyword("will").then_ignore(keyword("say")));
    let goto_keywords =
        keyword("go").then(keyword("to"))
        .or(keyword("goes").then(keyword("to")))
        .or(keyword("went").then(keyword("to")))
        .or(keyword("gone").then(keyword("to")))
        .or(keyword("going").then(keyword("to")));

    fn keywords(keywords: &[&str]) -> impl Parser<LexerToken, LexerToken, Error = Simple<LexerToken>> {
        fn or_helper(first: impl Parser<LexerToken, LexerToken, Error = Simple<LexerToken>>, second: impl Parser<LexerToken, LexerToken, Error = Simple<LexerToken>>) -> impl Parser<LexerToken, LexerToken, Error = Simple<LexerToken>> {
            first.or(second)
        }
        // keywords.into_iter().reduce(|a, b| or_helper(keyword(a), keyword(b))).collect::<Vec<_>>();
        // let result: &dyn Parser<LexerToken, LexerToken, Error = Simple<LexerToken>> = keywords.into_iter().map(|k| &keyword(k));
        // works!
        // let another = or_helper(result[0].clone(), result[1].clone());
        let mut result: Box<dyn Parser<LexerToken, LexerToken, Error = Simple<LexerToken>>> = Box::new(keyword(keywords[0]));
        let keys = keywords.into_iter().map(|k| keyword(k)).collect::<Vec<_>>();
        for i in 1..keywords.len() {
            result = Box::new(or_helper(result, keys[i].clone()));
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
        take_until(to_be)
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
        .then_ignore(positive_adjective)
        .then_ignore(keyword("as"))
        .then(take_until(end()))
        .map(|((a, _), (b, _))| ast::Statement::AddStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let subtraction_statement =
        take_until(keyword("felt"))
        .then_ignore(keyword("as"))
        .then_ignore(negative_adjective)
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
        .ignore_then(quote.clone().ignore_then(take_until(said_keyword.clone()))))
        .map(|(number, _)| ast::Statement::PrintNumberStatement(
            ast::Variable(lexer_tokens_to_name(number)))
        );

    let print_character_statement =
        quote.clone()
        .ignore_then(inner_quote.clone()
        .ignore_then(quote.clone()
        .ignore_then(take_until(said_keyword.clone()))
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
        take_until(goto_keywords)
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