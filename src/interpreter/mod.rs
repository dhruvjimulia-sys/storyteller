use num::BigUint;
use crate::ast_to_ir::ir::{Variable, self};
use std::collections::HashMap;
use std::ops::Rem;

fn get_labels(ir: &Vec<ir::Instruction>) -> HashMap<BigUint, usize> {
    let mut labels = HashMap::new();
    ir.iter().enumerate().for_each(|(i, instruction)| {
        match instruction {
            ir::Instruction::Label(label) => {
                labels.insert(label.clone(), i);
            }
            _ => {}
        }
    });
    labels
}

fn get_variable_value(variable: Variable, variable_values: &mut HashMap<Variable, BigUint>) -> BigUint {
    match variable_values.get(&variable) {
        Some(value) => value.clone(),
        None => panic!("Variable {} not found", variable.0)
    }
}

fn get_expression_value(expression: &ir::Expression, variable_values: &mut HashMap<Variable, BigUint>) -> BigUint {
    match expression {
        ir::Expression::NumberLiteral(value) => value.clone(),
        ir::Expression::Variable(variable) => get_variable_value(Variable(variable.to_string()), variable_values)
    }
}

fn number_to_string(value: BigUint) -> String {
    // e.g. 123124125 -> "abc"
    let mut result = String::new();
    let mut value = value;
    while value > 0u8.into() {
        let remainder = value.clone().rem(1000u32);
        result.push((remainder.to_u32_digits()[0] as u8) as char);
        value = value / 1000u32;
    }
    result.chars().rev().collect()
}

fn string_to_number(input: &str) -> BigUint {
    let mut result = BigUint::from(0u8);
    for c in input.chars() {
        result = result * 1000u16 + c as u8;
    }
    result
}

fn evaluate_condition(condition: &ir::Condition, variable_values: &mut HashMap<Variable, BigUint>) -> bool {
    match condition {
        ir::Condition::EqualTo(lhs, rhs) => {
            get_expression_value(lhs, variable_values) == get_expression_value(rhs, variable_values)
        }
        ir::Condition::NotEqualTo(lhs, rhs) => {
            get_expression_value(lhs, variable_values) != get_expression_value(rhs, variable_values)
        }
        ir::Condition::GreaterThan(lhs, rhs) => {
            get_expression_value(lhs, variable_values) > get_expression_value(rhs, variable_values)
        }
        ir::Condition::LessThan(lhs, rhs) => {
            get_expression_value(lhs, variable_values) < get_expression_value(rhs, variable_values)
        }
        ir::Condition::GreaterThanOrEqualTo(lhs, rhs) => {
            get_expression_value(lhs, variable_values) >= get_expression_value(rhs, variable_values)
        }
        ir::Condition::LessThanOrEqualTo(lhs, rhs) => {
            get_expression_value(lhs, variable_values) <= get_expression_value(rhs, variable_values)
        }
    }
}

pub fn interpret(ir: Vec<ir::Instruction>) {
    let mut variable_values: HashMap<Variable, BigUint> = HashMap::new();
    let labels = get_labels(&ir);
    let instruction_pointer = 0;
    interpret_helper(&ir, &mut variable_values, &labels, instruction_pointer);
}

fn interpret_helper(ir: &Vec<ir::Instruction>, variable_values: &mut HashMap<Variable, BigUint>, labels: &HashMap<BigUint, usize>, instruction_pointer: usize) {
    let instruction = match ir.get(instruction_pointer) {
        Some(instruction) => instruction,
        None => return
    };
    interpret_instruction(ir, instruction, variable_values, labels, instruction_pointer);
}

fn interpret_instruction(ir: &Vec<ir::Instruction>, instruction: &ir::Instruction, variable_values: &mut HashMap<Variable, BigUint>, labels: &HashMap<BigUint, usize>, instruction_pointer: usize) {
    match instruction {
        ir::Instruction::AssignmentInstruction(variable, expression) => {
            let expr_value = get_expression_value(expression, variable_values);
            variable_values.insert(variable.clone(), expr_value);
        }
        ir::Instruction::AddInstruction(variable, expression) => {
            let new_value = get_variable_value(variable.clone(), variable_values) + get_expression_value(expression, variable_values);
            variable_values.insert(variable.clone(), new_value);
        }
        ir::Instruction::SubInstruction(variable, expression) => {
            let new_value = get_variable_value(variable.clone(), variable_values) - get_expression_value(expression, variable_values);
            variable_values.insert(variable.clone(), new_value);
        }
        ir::Instruction::PrintNumberInstruction(variable) => {
            print!("{}", get_variable_value(variable.clone(), variable_values));
        }
        ir::Instruction::PrintCharacterInstruction(variable) => {
            print!("{}", number_to_string(get_variable_value(variable.clone(), variable_values)));
        }
        ir::Instruction::InputInstruction(variable) => {
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(error) => panic!("InputError: {}", error)
            }
            let input = string_to_number(input.trim());
            variable_values.insert(variable.clone(), input);
        }
        ir::Instruction::ExitInstruction => {
            return;
        }
        ir::Instruction::GotoInstruction(expression) => {
            let new_instruction_pointer = match labels.get(
                &get_expression_value(expression, variable_values)
            ) {
                Some(value) => *value,
                None => panic!("Label not found")
            };
            interpret_helper(ir, variable_values, labels, new_instruction_pointer);
            return;
        }
        ir::Instruction::IfInstruction(condition, statement) => {
            if evaluate_condition(condition, variable_values) {
                interpret_instruction(ir, statement, variable_values, labels, instruction_pointer);
            }
        }
        ir::Instruction::Label(_) => {}
    }
    interpret_helper(&ir, variable_values, labels, instruction_pointer + 1);
}