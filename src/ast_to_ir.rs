use crate::parser::ast;
use std::collections::HashSet;
use num::{BigUint, Zero};
use chumsky::prelude::*;
use std::ops::Rem;
mod ir;

pub fn convert_ast_to_ir(ast: ast::Program) -> Vec<ir::Instruction> {
    let variables = get_all_variables(&ast);
    let mut ir: Vec<ir::Instruction> = Vec::new();
    ast.0.iter().enumerate().for_each(|(i, block)| {
        ir.push(ir::Instruction::Label(i.into()));
        block.0.iter().for_each(|statement| {
            ir.push(statement_to_ir(statement, &variables));
        })
    });
    ir
}

fn statement_to_ir(statement: &ast::Statement, variables: &HashSet<ir::Variable>) -> ir::Instruction {
    match *statement {
        ast::Statement::AssignmentStatement(ref lhs, ref rhs) => {
            ir::Instruction::AssignmentInstruction(ir::Variable(lhs.0.clone()), replace_if_poetic_literal(rhs.clone(), variables.clone()))
        }
        ast::Statement::AddStatement(ref lhs, ref rhs) => {
            ir::Instruction::AddInstruction(ir::Variable(lhs.0.clone()), replace_if_poetic_literal(rhs.clone(), variables.clone()))
        }
        ast::Statement::SubStatement(ref lhs, ref rhs) => {
            ir::Instruction::SubInstruction(ir::Variable(lhs.0.clone()), replace_if_poetic_literal(rhs.clone(), variables.clone()))
        }
        ast::Statement::PrintNumberStatement(ref variable) => {
            ir::Instruction::PrintNumberInstruction(ir::Variable(variable.0.clone()))
        }
        ast::Statement::PrintCharacterStatement(ref variable) => {
            ir::Instruction::PrintCharacterInstruction(ir::Variable(variable.0.clone()))
        }
        ast::Statement::InputStatement(ref variable) => {
            ir::Instruction::InputInstruction(ir::Variable(variable.0.clone()))
        }
        ast::Statement::ExitStatement => {
            ir::Instruction::ExitInstruction
        }
        ast::Statement::GotoStatement(ref label) => {
            ir::Instruction::GotoInstruction(replace_if_poetic_literal(label.clone(), variables.clone()))
        }
        ast::Statement::IfStatement(ref condition, ref statement) => {
            ir::Instruction::IfInstruction(condition_to_ir(condition, variables.clone()), Box::new(statement_to_ir(statement, variables)))
        }
    }
}

fn condition_to_ir(condition: &ast::Condition, variables: &HashSet<ir::Variable>) -> ir::Condition {
    match *condition {
        ast::Condition::EqualTo(ref lhs, ref rhs) => {
            ir::Condition::EqualTo(replace_if_poetic_literal(lhs.clone(), &variables), replace_if_poetic_literal(rhs.clone(), &variables))
        }
        ast::Condition::NotEqualTo(ref lhs, ref rhs) => {
            ir::Condition::NotEqualTo(replace_if_poetic_literal(lhs.clone(), &variables), replace_if_poetic_literal(rhs.clone(), &variables))
        }
        ast::Condition::GreaterThan(ref lhs, ref rhs) => {
            ir::Condition::GreaterThan(replace_if_poetic_literal(lhs.clone(), &variables), replace_if_poetic_literal(rhs.clone(), &variables))
        }
        ast::Condition::LessThan(ref lhs, ref rhs) => {
            ir::Condition::LessThan(replace_if_poetic_literal(lhs.clone(), &variables), replace_if_poetic_literal(rhs.clone(), &variables))
        }
        ast::Condition::GreaterThanOrEqualTo(ref lhs, ref rhs) => {
            ir::Condition::GreaterThanOrEqualTo(replace_if_poetic_literal(lhs.clone(), &variables), replace_if_poetic_literal(rhs.clone(), &variables))
        }
        ast::Condition::LessThanOrEqualTo(ref lhs, ref rhs) => {
            ir::Condition::LessThanOrEqualTo(replace_if_poetic_literal(lhs.clone(), &variables), replace_if_poetic_literal(rhs.clone(), &variables))
        }
    }
}

fn get_all_variables(ast: &ast::Program) -> HashSet<ir::Variable> {
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
                    let variables_in_statement = get_all_variables(&ast::Program(vec!(ast::Block(vec!((**statement).clone())))));
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

fn replace_if_poetic_literal(value: ast::VariableOrNumberLiteral, variables: &HashSet<ir::Variable>) -> ir::Expression {
    match value {
        ast::VariableOrNumberLiteral(value) => {
            if variables.contains(&ir::Variable(value.clone())) {
                ir::Expression::Variable(value)
            } else {
                ir::Expression::NumberLiteral(convert_poetic_literal_to_integer(value))
            }
        }
    }
}

pub fn convert_poetic_literal_to_integer(poetic_literal: String) -> BigUint {
    fn convert_poetic_string_to_digit(poetic_string: String) -> u8 {
        let mut result: BigUint = Zero::zero();
        for c in poetic_string.chars() {
            result += (c as u8) - ('a' as u8) + 1
        }
       let result_digits = (result.rem(BigUint::new(vec!(10)))).to_u32_digits();
       if result_digits.len() == 0 { 0 } else { result_digits[0] as u8 }
    }
    
    let mut result: BigUint = Zero::zero();
    let poetic_literal_spaces_split = text::ident::<_, Simple<char>>().padded().repeated().parse(poetic_literal).unwrap();

    for i in 0..poetic_literal_spaces_split.len() {
        let pow = 10_usize.pow((poetic_literal_spaces_split.len() - i - 1) as u32);
        result += pow * (convert_poetic_string_to_digit(poetic_literal_spaces_split[i].clone()) as usize)
    }
    result
}