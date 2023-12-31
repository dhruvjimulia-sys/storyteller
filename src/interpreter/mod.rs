use num::BigUint;
use std::io::{Write, BufRead};
use std::collections::HashMap;
use std::ops::Rem;
use crate::ast_to_ir::ir::{Variable, self};
use crate::errors::runtime_errors::{input_error, variable_not_found, label_not_found, output_error};

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
        None => { variable_not_found(variable.0.to_string()).display(); BigUint::from(0u8) }
    }
}

fn get_expression_value(expression: ir::Expression, variable_values: &mut HashMap<Variable, BigUint>) -> BigUint {
    match expression {
        ir::Expression::NumberLiteral(value) => value,
        ir::Expression::Variable(variable) => get_variable_value(Variable(variable.to_string()), variable_values)
    }
}

fn number_to_string(value: BigUint) -> String {
    let mut result = String::new();
    let mut value = value;
    while value > 0u8.into() {
        let num = value.clone().rem(1000u32).to_u32_digits()[0] % 128u32;
        result.push((num as u8) as char);
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

fn evaluate_condition(condition: ir::Condition, variable_values: &mut HashMap<Variable, BigUint>) -> bool {
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
    }
}

pub fn interpret(ir: Vec<ir::Instruction>, input_stream: &mut dyn std::io::BufRead, output_stream: &mut dyn std::io::Write) {
    let mut variable_values: HashMap<Variable, BigUint> = HashMap::new();
    let labels = get_labels(&ir);
    let mut instruction_pointer = 0;
    while instruction_pointer < ir.len() {
        let instruction = ir[instruction_pointer].clone();
        let new_instruction_pointer = interpret_instruction(instruction, &mut variable_values, &labels, instruction_pointer, input_stream, output_stream);
        match new_instruction_pointer {
            Some(new_instruction_pointer) => instruction_pointer = new_instruction_pointer,
            None => break
        }
    }
}

fn interpret_instruction(instruction: ir::Instruction, variable_values: &mut HashMap<Variable, BigUint>, labels: &HashMap<BigUint, usize>, instruction_pointer: usize, input_stream:&mut dyn BufRead, output_stream: &mut dyn Write) -> Option<usize> {
    match instruction {
        ir::Instruction::AssignmentInstruction(variable, expression) => {
            let expr_value = get_expression_value(expression, variable_values);
            variable_values.insert(variable, expr_value);
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
            match write!(output_stream, "{}", get_variable_value(variable.clone(), variable_values)) {
                Ok(_) => {}
                Err(_) => { output_error().display() }
            };
        }
        ir::Instruction::PrintStringInstruction(variable) => {
            match write!(output_stream, "{}", number_to_string(get_variable_value(variable.clone(), variable_values))) {
                Ok(_) => {}
                Err(_) => { input_error().display() }
            };
        }
        ir::Instruction::InputInstruction(variable) => {
            let mut input = String::new();
            match input_stream.read_line(&mut input) {
                Ok(_) => {},
                Err(_) => { input_error().display() }
            };
            let num_input = string_to_number(input.trim());
            variable_values.insert(variable.clone(), num_input);
        }
        ir::Instruction::ExitInstruction => {
            return None;
        }
        ir::Instruction::GotoInstruction(expression) => {
            let new_instruction_pointer = match labels.get(
                &get_expression_value(expression, variable_values)
            ) {
                Some(value) => *value,
                None => { label_not_found().display(); return None; }
            };
            return Some(new_instruction_pointer);
        }
        ir::Instruction::IfInstruction(condition, statement) => {
            if evaluate_condition(condition, variable_values) {
                return interpret_instruction(*statement, variable_values, labels, instruction_pointer, input_stream, output_stream)
            }
        }
        ir::Instruction::Label(_) => {}
    }
    Some(instruction_pointer + 1)
}