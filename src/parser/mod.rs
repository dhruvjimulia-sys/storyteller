use chumsky::prelude::*;
use chumsky::primitive::Just;
use std::collections::HashSet;
use itertools::Itertools;
use crate::lexer::lexer_types::{LexerOutput, LexerToken};
use crate::errors::Error;
use crate::errors::compiler_errors;
pub mod ast;
mod keyword_defs;
use keyword_defs::defs;

fn statement_parser() -> impl Parser<LexerToken, ast::Statement, Error = Simple<LexerToken>> {
    fn keywords(keywords: &HashSet<String>) -> impl Parser<LexerToken, LexerToken, Error = Simple<LexerToken>> {
        fn full_keyword(full_keyword: &str) -> impl Parser<LexerToken, LexerToken, Error = Simple<LexerToken>> {
            let full_split = full_keyword.split(" ").filter(|key| key.len() != 0).collect::<Vec<_>>();
            let mut full_keyword_result: Box<dyn Parser<LexerToken, LexerToken, Error = Simple<LexerToken>>> = Box::new(keyword(full_split[0]));
            for i in 1..full_split.len() {
                full_keyword_result = Box::new(full_keyword_result.then_ignore(keyword(full_split[i])));
            }
            full_keyword_result
        }

        let keywords = keywords.iter().map(|s| s.as_str()).unique().collect::<Vec<_>>();
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

    fn text_tokens_except(token_set: HashSet<String>, min_num_tokens: usize) -> impl Parser<LexerToken, Vec<LexerToken>, Error = Simple<LexerToken>> {
        filter(move |token| match token {
            LexerToken::Text(text) => {
                let t_set = token_set.iter().map(|s| s.split(" ").collect::<Vec<_>>()[0].to_string()).collect::<HashSet<_>>();
                !t_set.contains(text)
            },
            _ => false
        }).repeated().at_least(min_num_tokens)
    }

    fn text_tokens(min_num_tokens: usize) -> impl Parser<LexerToken, Vec<LexerToken>, Error = Simple<LexerToken>>  {
        text_tokens_except(HashSet::new(), min_num_tokens)
    }

    let optional_surbodinate_clause = just(LexerToken::Comma).then(any().repeated()).or_not();
    let adverb_keyword = filter(|token: &LexerToken| match token {
        LexerToken::Text(s) => s.ends_with("ly"),
        _ => false
    });

    let assignment_statement =
        text_tokens_except(defs().to_be, 1)
        .then_ignore(keywords(&defs().to_be))
        .then(text_tokens(1))
        .then_ignore(optional_surbodinate_clause.clone())
        .then_ignore(end())
        .map(|(a, b)| ast::Statement::AssignmentStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let addition_statement =
        text_tokens_except(HashSet::from(["felt".to_string()]), 1)
        .then_ignore(keyword("felt"))
        .then_ignore(keyword("as"))
        .then_ignore(keywords(&defs().positive_adjective))
        .then_ignore(keyword("as"))
        .then(text_tokens(1))
        .then_ignore(optional_surbodinate_clause.clone())
        .then_ignore(end())
        .map(|(a, b)| ast::Statement::AddStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let subtraction_statement =
        text_tokens_except(HashSet::from(["felt".to_string()]), 1)
        .then_ignore(keyword("felt"))
        .then_ignore(keyword("as"))
        .then_ignore(keywords(&defs().negative_adjective))
        .then_ignore(keyword("as"))
        .then(text_tokens(1))
        .then_ignore(optional_surbodinate_clause.clone())
        .then_ignore(end())
        .map(|(a, b)| ast::Statement::SubStatement(
            ast::Variable(lexer_tokens_to_name(a)),
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(b))
        ));

    let quote = just(LexerToken::Quote);
    let comma = just(LexerToken::Comma);
    let inner_quote = none_of(vec![LexerToken::Quote]).repeated();

    let print_number_statement =
        quote.clone()
        .ignore_then(inner_quote.clone()
        .ignore_then(quote.clone().ignore_then(
            text_tokens_except(defs().said, 1)
            .then_ignore(keywords(&defs().said))
        )))
        .then_ignore(optional_surbodinate_clause.clone())
        .then_ignore(end())
        .map(|number| ast::Statement::PrintNumberStatement(
            ast::Variable(lexer_tokens_to_name(number)))
        );

    let print_string_statement =
        quote.clone()
        .ignore_then(inner_quote.clone()
        .ignore_then(quote.clone()
        .ignore_then(
            text_tokens_except(defs().said, 1)
            .then_ignore(keywords(&defs().said)
        ))
        .then_ignore(adverb_keyword.clone())))
        .then_ignore(optional_surbodinate_clause.clone())
        .then_ignore(end())
        .map(|number| ast::Statement::PrintStringStatement(
            ast::Variable(lexer_tokens_to_name(number))
        ));

    let looks_keyword = HashSet::from(["looked".to_string(), "looks".to_string()]);

    let input_statement =
        text_tokens_except(looks_keyword.clone(), 1)
        .then_ignore(keywords(&looks_keyword))
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
        .then_ignore(optional_surbodinate_clause.clone())
        .then_ignore(end())
        .map(|name| ast::Statement::InputStatement(
            ast::Variable(lexer_tokens_to_name(name))
        ));

    let goto_statement =
        text_tokens_except(defs().goto, 0)
        .ignore_then(keywords(&defs().goto))
        .ignore_then(text_tokens(1))
        .then_ignore(optional_surbodinate_clause)
        .then_ignore(end())
        .map(|name| ast::Statement::GotoStatement(
            ast::VariableOrNumberLiteral(lexer_tokens_to_name(name))
        ));

    let exit_statement =
        take_until(keyword("end"))
        .ignore_then(any().repeated())
        .then_ignore(end())
        .map(|_| ast::Statement::ExitStatement);

    let comment =
        take_until(end())
        .map(|_| ast::Statement::Comment);

    fn if_statement(statement_parser: Recursive<'_, LexerToken, ast::Statement, Simple<LexerToken>>) -> impl Parser<LexerToken, ast::Statement, Error = Simple<LexerToken>> + '_ {
        fn to_strings(set: HashSet<&str>) -> HashSet<String> {
            set.into_iter().map(|s| s.to_string()).collect::<HashSet<_>>()
        }
        
        let comma = just(LexerToken::Comma);
        let optional_surbodinate_clause = just(LexerToken::Comma).then(any::<LexerToken, Simple<LexerToken>>().repeated()).or_not();
        let greater_than_condition = keywords(&defs().to_be).or(keyword("felt")).then(keywords(&defs().positive_comparative_adjective)).then(keyword("than"));
        let less_than_condition = keywords(&defs().to_be).or(keyword("felt")).then(keywords(&defs().negative_comparative_adjective)).then(keyword("than"));
        let equal_to_condition = keywords(&defs().to_be).or(keywords(&to_strings(HashSet::from(["want to be like", "wanted to be like", "wants to be like"])))); 
        let not_equal_to_condition = keywords(&defs().to_be).ignore_then(keyword("not")).or(keywords(&to_strings(HashSet::from(["did not want to be like", "does not want to be like"])))); 
        let condition_start_tokens = defs().to_be.clone().into_iter().chain(vec!["felt".to_string()]).collect::<HashSet<_>>(); 
        
        let condition =
            text_tokens_except(condition_start_tokens.clone(), 1)
            .then_ignore(greater_than_condition)
            .then(text_tokens(1))
            .map(|(lhs, rhs)| ast::Condition::GreaterThan(
                ast::VariableOrNumberLiteral(lexer_tokens_to_name(lhs)),
                ast::VariableOrNumberLiteral(lexer_tokens_to_name(rhs))
            ))
            .or(
                text_tokens_except(condition_start_tokens.clone(), 1)
                .then_ignore(less_than_condition)
                .then(text_tokens(1))
                .map(|(lhs, rhs)| ast::Condition::LessThan(
                    ast::VariableOrNumberLiteral(lexer_tokens_to_name(lhs)),
                    ast::VariableOrNumberLiteral(lexer_tokens_to_name(rhs))
                ))
            )
            .or(
                text_tokens_except(condition_start_tokens.clone(), 1)
                .then_ignore(not_equal_to_condition)
                .then(text_tokens(1))
                .map(|(lhs, rhs)| ast::Condition::NotEqualTo(
                    ast::VariableOrNumberLiteral(lexer_tokens_to_name(lhs)),
                    ast::VariableOrNumberLiteral(lexer_tokens_to_name(rhs))
                ))
            )
            .or(
                text_tokens_except(condition_start_tokens, 1)
                .then_ignore(equal_to_condition)
                .then(text_tokens(1))
                .map(|(lhs, rhs)| ast::Condition::EqualTo(
                    ast::VariableOrNumberLiteral(lexer_tokens_to_name(lhs)),
                    ast::VariableOrNumberLiteral(lexer_tokens_to_name(rhs))
                ))
            );
            

        keyword("if")
        .ignore_then(text_tokens(1))
        .then_ignore(comma)
        .then_ignore(keyword("then"))
        .then_ignore(optional_surbodinate_clause)
        .then(take_until(end()))
        .map(move |(condition_tokens, (consequence, _))| {
            ast::Statement::IfStatement(
            condition.parse(condition_tokens).unwrap(),
            Box::new(statement_parser.parse(consequence).unwrap())
        )})
    }

    let statement = recursive(|statement| {
        if_statement(statement)
        .or(input_statement)
        .or(print_string_statement)
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
            .map(|statement| {
                statement_parser().parse(statement).unwrap()
            }).collect())
        });
    block_parser
}

pub fn parse_program(input: LexerOutput) -> Result<ast::Program, Vec<Error>> {
    let mut errors = vec![];
    let program = ast::Program(input.0.into_iter().map(|block| {
        let parsed_block = match statement_block_parser().parse(block.0.clone()) {
            Ok(s) => s,
            Err(_) =>  {
                errors.push(compiler_errors::unfinished_thought_error());
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