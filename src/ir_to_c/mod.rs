use std::collections::HashSet;

use num::BigUint;

use crate::ast_to_ir::ir::{self, Instruction};

pub fn convert_ir_to_c(ir: Vec<ir::Instruction>, variables: HashSet<ir::Variable>) -> String {
    let mut c_code = String::new();
    c_code.push_str("#include <stdio.h>\n");
    c_code.push_str("#include <stdlib.h>\n");
    c_code.push_str(get_goto_macro(&ir).as_str());
    c_code.push_str("int string_to_number(char *string) {
        int result = 0;
        while (*string != '\\0') {
            result = result * 1000 + *string;
            string++;
        }
        return result;
    }\n");
    c_code.push_str("int number_to_string(int number, char* output) {
        int i = 0;
        while (number > 0) {
            output[i] = (number % 1000) % 128;
            number /= 1000;
            i++;
        }
        output[i] = '\\0';
        return i;
    }\n");
    c_code.push_str("int main() {\n");
    c_code.push_str("char *input;\n");
    c_code.push_str("char *output;\n");
    variables.iter().for_each(|var| c_code.push_str(format!("int {} = 0;\n", ir_variable_to_c_variable(var)).as_str()));
    for instruction in ir {
        c_code.push_str(&instruction_to_c(instruction));
    }
    c_code.push_str("}");
    c_code
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
            get_c_for_print_string_instruction(variable)
        }
        Instruction::InputInstruction(variable) => {
            get_c_for_input_insruction(variable)
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

fn get_c_for_input_insruction(variable: ir::Variable) -> String {
    format!("\
    input = (char *) malloc(100 * sizeof(char)); \n\
    if (input == NULL) {{ \n\
        printf(\"Memory allocation failed\\n\"); \n\
        return 1; \n\
    }} \n\
    get_input(input); \n\
    {} = string_to_number(input); \n\
    free(input);\n", ir_variable_to_c_variable(&variable))
}

fn get_c_for_print_string_instruction(variable: ir::Variable) -> String {
    format!("\
    output = (char *) malloc(100 * sizeof(char)); \n\
    if (output == NULL) {{ \n\
        printf(\"Memory allocation failed\\n\"); \n\
        return 1; \n\
    }} \n\
    number_to_string({}, output); \n\
    printf(\"%s\", output); \n\
    free(output);\n", ir_variable_to_c_variable(&variable))
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