use crate::parser;
use parser::ast;
use crate::lexer;
use chumsky::prelude::*;

#[test] 
fn parser_correctly_parses_assignment_statement_with_was() {
    let program = "Charlie was a wizard.";
    let lexer_output = lexer::lexer().parse(program).unwrap();
    let ast = parser::parse_program(lexer_output);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("Charlie".to_string()),
                ast::VariableOrNumberLiteral("a wizard".to_string())
            )
        ]
    )]));
}

#[test] 
fn parser_correctly_parses_assignment_statement_with_were() {
    let program = "The dog and the cat were great company.";
    let lexer_output = lexer::lexer().parse(program).unwrap();
    let ast = parser::parse_program(lexer_output);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("The dog and the cat".to_string()),
                ast::VariableOrNumberLiteral("great company".to_string())
            )
        ]
    )]));
}

#[test] 
fn parser_correctly_parses_assignment_statement_with_is() {
    let program = "Ron is here.";
    let lexer_output = lexer::lexer().parse(program).unwrap();
    let ast = parser::parse_program(lexer_output);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("Ron".to_string()),
                ast::VariableOrNumberLiteral("here".to_string())
            )
        ]
    )]));
}

#[test] 
fn parser_correctly_parses_assignment_statement_with_are() {
    let program = "Percy and Annabeth are here.";
    let lexer_output = lexer::lexer().parse(program).unwrap();
    let ast = parser::parse_program(lexer_output);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("Percy and Annabeth".to_string()),
                ast::VariableOrNumberLiteral("here".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_add_statement() {
    let program = "Percy felt as good as a friend.";
    let lexer_output = lexer::lexer().parse(program).unwrap();
    let ast = parser::parse_program(lexer_output);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AddStatement(
                ast::Variable("Percy".to_string()),
                ast::VariableOrNumberLiteral("a friend".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_sub_statement() {
    let program = "Macbeth felt as bad as rain.";
    let lexer_output = lexer::lexer().parse(program).unwrap();
    let ast = parser::parse_program(lexer_output);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::SubStatement(
                ast::Variable("Macbeth".to_string()),
                ast::VariableOrNumberLiteral("rain".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_print_number_statement() {
    let program = "Charlie said, \"I am a wizard\".";
    let lexer_output = lexer::lexer().parse(program).unwrap();
    let ast = parser::parse_program(lexer_output);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::PrintNumberStatement(
                ast::Variable("Charlie".to_string())
            )
        ]
    )]));
}