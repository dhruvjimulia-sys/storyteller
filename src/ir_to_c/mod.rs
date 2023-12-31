mod codegen_utils;
use std::collections::HashSet;
use num::BigUint;
use crate::ast_to_ir::ir::{self, Instruction};


pub fn convert_ir_to_c(ir: Vec<ir::Instruction>, variables: HashSet<ir::Variable>) -> String {
    let mut c_code = String::new();
    generate_imports(&mut c_code);
    generate_macros(&mut c_code, &ir);
    codegen_utils::generate_helper_functions(&mut c_code);
    generate_main_function(&mut c_code, variables, ir);
    c_code
}

fn generate_main_function(c_code: &mut String, variables: HashSet<ir::Variable>, ir: Vec<Instruction>) {
    generate_main_scope_entry(c_code);
    generate_variable_initializations(c_code, variables);
    for instruction in ir {
        c_code.push_str(&instruction_to_c(instruction));
    }
    generate_scope_exit(c_code);
}

fn generate_scope_exit(c_code: &mut String) {
    c_code.push_str("}");
}

fn generate_main_scope_entry(c_code: &mut String) {
    c_code.push_str("int main() {\n");
}

fn generate_macros(c_code: &mut String, ir: &Vec<Instruction>) {
    c_code.push_str(get_goto_macro(ir).as_str());
}

fn generate_variable_initializations(c_code: &mut String, variables: HashSet<ir::Variable>) {
    c_code.push_str("char *input = NULL;\n");
    c_code.push_str("char *output = NULL;\n");
    c_code.push_str("int bufferSize = NULL;\n");
    variables.iter().for_each(|var| c_code.push_str(format!("long long int {} = 0;\n", ir_variable_to_c_variable(var)).as_str()));
}

fn generate_imports(c_code: &mut String) {
    c_code.push_str("#include <stdio.h>\n");
    c_code.push_str("#include <stdlib.h>\n");
    c_code.push_str("#include <string.h>\n");
}

fn instruction_to_c(instruction: ir::Instruction) -> String {
    match instruction {
        Instruction::AssignmentInstruction(lhs, rhs) => {
            format!("{} = {};\n", ir_variable_to_c_variable(&lhs), ir_expression_to_c(rhs))
        }
        Instruction::AddInstruction(lhs, rhs) => {
            format!("{} += {};\n", ir_variable_to_c_variable(&lhs), ir_expression_to_c(rhs))
        }
        Instruction::SubInstruction(lhs, rhs) => {
            format!("{} -= {};\n", ir_variable_to_c_variable(&lhs), ir_expression_to_c(rhs))
        }
        Instruction::PrintNumberInstruction(variable) => {
            format!("printf(\"%d\", {});\n", ir_variable_to_c_variable(&variable))
        }
        Instruction::PrintStringInstruction(variable) => {
            codegen_utils::get_c_for_print_string_instruction(variable)
        }
        Instruction::InputInstruction(variable) => {
            codegen_utils::get_c_for_input_insruction(variable)
        }
        Instruction::ExitInstruction => {
            "exit(0);\n".to_string()
        }
        Instruction::GotoInstruction(label) => {
            match label {
                ir::Expression::NumberLiteral(num) => format!("goto {};", convert_to_label(num)),
                ir::Expression::Variable(var) => format!("GOTO_VAR({});\n", ir_variable_to_c_variable(&ir::Variable(var)))
            }
        }
        Instruction::IfInstruction(condition, inner_instruction) => {
            format!("if ({}) {{\n{}}}\n", condition_to_c(condition), instruction_to_c(*inner_instruction))
        }
        Instruction::Label(label) => {
            format!("{}:\n", convert_to_label(label))
        }
    }
}

fn convert_to_label(label: BigUint) -> String {
    format!("label_{}", label)
}

fn ir_variable_to_c_variable(variable: &ir::Variable) -> String {
    format!("var_{}", variable.0.replace(" ", "_"))
}

fn ir_expression_to_c(expression: ir::Expression) -> String {
    match expression {
        ir::Expression::Variable(variable) => {
            ir_variable_to_c_variable(&ir::Variable(variable))
        }
        ir::Expression::NumberLiteral(number) => {
            number.to_string()
        }
    }
}

fn condition_to_c(condition: ir::Condition) -> String {
    match condition {
        ir::Condition::EqualTo(lhs, rhs) => {
            format!("{} == {}", ir_expression_to_c(lhs), ir_expression_to_c(rhs))
        }
        ir::Condition::NotEqualTo(lhs, rhs) => {
            format!("{} != {}", ir_expression_to_c(lhs), ir_expression_to_c(rhs))
        }
        ir::Condition::GreaterThan(lhs, rhs) => {
            format!("{} > {}", ir_expression_to_c(lhs), ir_expression_to_c(rhs))
        }
        ir::Condition::LessThan(lhs, rhs) => {
            format!("{} < {}", ir_expression_to_c(lhs), ir_expression_to_c(rhs))
        }
    }
}

fn get_goto_macro(ir: &Vec<ir::Instruction>) -> String {
    let mut labels = HashSet::new();
    for instruction in ir {
        match instruction {
            Instruction::Label(label) => {
                labels.insert(label);
            }
            _ => {}
        }
    }
    let mut result = String::new();
    result.push_str("#define GOTO_VAR(var) \\\n");
    result.push_str("do { \\\n");
    let mut first = true;
    for label in labels {
        if first {
            first = false;
        } else {
            result.push_str("else ");
        }
        result.push_str(format!("if (var == {}) goto label_{};\\\n", label, label).as_str());
    }
    result.push_str("} while (0)\n");
    result
}