use crate::parser::ast;
use std::collections::HashSet;
use num::{BigUint, Zero};
use chumsky::prelude::*;
pub mod ir;
mod variable_extractor;
mod pronoun_replacer;

pub fn convert_ast_to_ir(ast: ast::Program) -> Vec<ir::Instruction> {
    let variables = variable_extractor::get_variables(&ast);
    let processed_ast = pronoun_replacer::replace_pronouns(&ast, &variables);
    let mut ir: Vec<ir::Instruction> = Vec::new();
    processed_ast.0.iter().enumerate().for_each(|(i, block)| {
        ir.push(ir::Instruction::Label(i.into()));
        block.0.iter().for_each(|statement| {
            match statement_to_ir(statement, &variables) {
                Some(instruction) => ir.push(instruction),
                None => {}
            }
        })
    });
    ir
}

fn statement_to_ir(statement: &ast::Statement, variables: &HashSet<ir::Variable>) -> Option<ir::Instruction> {
    match *statement {
        ast::Statement::AssignmentStatement(ref lhs, ref rhs) => {
            Some(ir::Instruction::AssignmentInstruction(ir::Variable(lhs.0.clone()), replace_if_poetic_literal(rhs.clone(), variables)))
        }
        ast::Statement::AddStatement(ref lhs, ref rhs) => {
            Some(ir::Instruction::AddInstruction(ir::Variable(lhs.0.clone()), replace_if_poetic_literal(rhs.clone(), variables)))
        }
        ast::Statement::SubStatement(ref lhs, ref rhs) => {
            Some(ir::Instruction::SubInstruction(ir::Variable(lhs.0.clone()), replace_if_poetic_literal(rhs.clone(), variables)))
        }
        ast::Statement::PrintNumberStatement(ref variable) => {
            Some(ir::Instruction::PrintNumberInstruction(ir::Variable(variable.0.clone())))
        }
        ast::Statement::PrintStringStatement(ref variable) => {
            Some(ir::Instruction::PrintCharacterInstruction(ir::Variable(variable.0.clone())))
        }
        ast::Statement::InputStatement(ref variable) => {
            Some(ir::Instruction::InputInstruction(ir::Variable(variable.0.clone())))
        }
        ast::Statement::ExitStatement => {
            Some(ir::Instruction::ExitInstruction)
        }
        ast::Statement::GotoStatement(ref label) => {
            Some(ir::Instruction::GotoInstruction(replace_if_poetic_literal(label.clone(), variables)))
        }
        ast::Statement::IfStatement(ref condition, ref statement) => {
            match statement_to_ir(statement, variables) {
                Some(inner_statement) => {
                    Some(ir::Instruction::IfInstruction(condition_to_ir(condition, variables), Box::new(inner_statement)))
                }
                None => {
                    None
                }
            }
        }
        ast::Statement::Comment => {
            None
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
    }
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
        (poetic_string.len() % 10).try_into().unwrap()
    }
    
    let mut result: BigUint = Zero::zero();
    let digits_radix = 36;
    let poetic_literal_spaces_split = text::digits::<_, Simple<char>>(digits_radix).padded().repeated().parse(poetic_literal).unwrap();

    for i in 0..poetic_literal_spaces_split.len() {
        let pow = 10_usize.pow((poetic_literal_spaces_split.len() - i - 1) as u32);
        result += pow * (convert_poetic_string_to_digit(poetic_literal_spaces_split[i].clone()) as usize)
    }
    result
}