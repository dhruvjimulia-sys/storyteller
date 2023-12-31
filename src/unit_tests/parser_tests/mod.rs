mod parser_test_utils;
use crate::parser::ast;
use parser_test_utils::parse_program_string;

#[test] 
fn parser_correctly_parses_assignment_statement_with_was() {
    let program = "Charlie was a wizard.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("charlie".to_string()),
                ast::VariableOrNumberLiteral("a wizard".to_string())
            )
        ]
    )]));
}

#[test] 
fn parser_correctly_parses_assignment_statement_with_were() {
    let program = "The dog and the cat were great company.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("the dog and the cat".to_string()),
                ast::VariableOrNumberLiteral("great company".to_string())
            )
        ]
    )]));
}

#[test] 
fn parser_correctly_parses_assignment_statement_with_is() {
    let program = "Ron is here.";
    let ast = parse_program_string(program);


    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("ron".to_string()),
                ast::VariableOrNumberLiteral("here".to_string())
            )
        ]
    )]));
}

#[test] 
fn parser_correctly_parses_assignment_statement_with_are() {
    let program = "Percy and Annabeth are here.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("percy and annabeth".to_string()),
                ast::VariableOrNumberLiteral("here".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_add_statement() {
    let program = "Percy felt as good as a friend.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AddStatement(
                ast::Variable("percy".to_string()),
                ast::VariableOrNumberLiteral("a friend".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_sub_statement() {
    let program = "Macbeth felt as bad as rain.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::SubStatement(
                ast::Variable("macbeth".to_string()),
                ast::VariableOrNumberLiteral("rain".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_print_number_statement() {
    let program = "\"I am a wizard\" Charlie said.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::PrintNumberStatement(
                ast::Variable("charlie".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_print_character_statement() {
    let program = "\"I am a wizard\" Charlie said slyly.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::PrintStringStatement(
                ast::Variable("charlie".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_statements_that_end_in_exclamation_marks() {
    let program = "Bob was running! Katniss was tired. John was swimming!";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("bob".to_string()),
                ast::VariableOrNumberLiteral("running".to_string())
            ),
            ast::Statement::AssignmentStatement(
                ast::Variable("katniss".to_string()),
                ast::VariableOrNumberLiteral("tired".to_string())
            ),
            ast::Statement::AssignmentStatement(
                ast::Variable("john".to_string()),
                ast::VariableOrNumberLiteral("swimming".to_string())
            )
        ]
    )]));
}

#[test]
fn parser_correctly_parses_statements_that_end_in_question_marks() {
    let program = "Bob was running? Katniss was tired. John was swimming?";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::AssignmentStatement(
                ast::Variable("bob".to_string()),
                ast::VariableOrNumberLiteral("running".to_string())
            ),
            ast::Statement::AssignmentStatement(
                ast::Variable("katniss".to_string()),
                ast::VariableOrNumberLiteral("tired".to_string())
            ),
            ast::Statement::AssignmentStatement(
                ast::Variable("john".to_string()),
                ast::VariableOrNumberLiteral("swimming".to_string())
            )
        ]
    )]));
}


#[test]
fn parser_correctly_parses_input_statements() {
    let program = "Taylor looked up to the skies beyond, waiting for an answer.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::InputStatement(
                ast::Variable("taylor".to_string())
            )
        ]
    )]));
}


#[test]
fn parser_correctly_parses_exit_statements() {
    let program = "I hoped with all my heart that all this misery comes to an end.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::ExitStatement
        ]
    )]));
}


#[test]
fn parser_correctly_parses_goto_statement() {
    let program = "Cindrella wished she could go to heaven.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::GotoStatement(ast::VariableOrNumberLiteral("heaven".to_string()))
        ]
    )]));
}

#[test]
fn parser_correctly_parses_if_statment() {
    let program = "If Cinderella is better than the prince, then go to heaven.";
    let ast = parse_program_string(program);

    assert_eq!(ast, ast::Program(vec![ast::Block(
        vec![
            ast::Statement::IfStatement(
                ast::Condition::GreaterThan(ast::VariableOrNumberLiteral("cinderella".to_string()),
                ast::VariableOrNumberLiteral("the prince".to_string())),
                Box::new(ast::Statement::GotoStatement(ast::VariableOrNumberLiteral("heaven".to_string())))
            )
        ]
    )]));
}