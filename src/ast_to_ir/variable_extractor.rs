use crate::parser::ast;
use std::collections::HashSet;
use crate::ast_to_ir::ir;

pub fn get_variables(ast: &ast::Program) -> HashSet<ir::Variable> {
    let mut variables = HashSet::new();
    ast.0.iter().for_each(|block| {
        block.0.iter().for_each(|statement| {
            match statement {
                ast::Statement::AssignmentStatement(lhs, _) => {
                    variables.insert(ir::Variable(lhs.0.clone()));
                }
                ast::Statement::AddStatement(lhs, _) => {
                    variables.insert(ir::Variable(lhs.0.clone()));
                }
                ast::Statement::SubStatement(lhs, _) => {
                    variables.insert(ir::Variable(lhs.0.clone()));
                }
                ast::Statement::PrintNumberStatement(variable) => {
                    variables.insert(ir::Variable(variable.0.clone()));
                }
                ast::Statement::PrintCharacterStatement(variable) => {
                    variables.insert(ir::Variable(variable.0.clone()));
                }
                ast::Statement::InputStatement(variable) => {
                    variables.insert(ir::Variable(variable.0.clone()));
                }
                ast::Statement::IfStatement(_, statement) => {
                    let variables_in_statement = get_variables(&ast::Program(vec!(ast::Block(vec!((**statement).clone())))));
                    for variable in variables_in_statement {
                        variables.insert(variable);
                    };
                }
                _ => {}
            }
        });
    });
    variables
}