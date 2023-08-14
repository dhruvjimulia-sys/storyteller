use crate::ast_to_ir::ir::{self};
use super::*;

pub fn generate_helper_functions(c_code: &mut String) {
    generate_string_to_number_function(c_code);
    generate_number_to_string_function(c_code);
    generate_get_input_function(c_code);
}

fn generate_get_input_function(c_code: &mut String) {
    c_code.push_str("\
    void get_input(char *input, int bufferSize) {
        long long int index = 0; 
        long long int ch;
        while ((ch = getchar()) != '\\n' && ch != EOF) {
            if (index >= bufferSize - 1) {
                bufferSize *= 2;
                char *newInput = (char *) realloc (input, bufferSize * sizeof(char));
                if (newInput == NULL) {
                    printf(\"Memory reallocation failed\\n\");
                    free(input);
                    exit(1);
                }
                input = newInput;
            }
            input[index] = ch;
            index++;
        }
        input[index] = '\\0';   
    }\n");
}

fn generate_number_to_string_function(c_code: &mut String) {
    c_code.push_str("\
    long long int number_to_string(long long int number, char* output) {
        long long int i = 0;
        while (number > 0) {
            output[i] = (number % 1000) % 128;
            number /= 1000;
            i++;
        }
        output[i] = '\\0';
        return i;
    }\n");
}

fn generate_string_to_number_function(c_code: &mut String) {
    c_code.push_str("\
    long long int string_to_number(char *string) {
        long long int result = 0;
        for (int i = strlen(string) - 1; i >= 0; i--) {
            result = result * 1000 + string[i];
        }
        return result;
    }\n");
}

pub fn get_c_for_input_insruction(variable: ir::Variable) -> String {
    format!("\
    bufferSize = 100; \n\
    input = (char *) malloc(bufferSize * sizeof(char)); \n\
    if (input == NULL) {{ \n\
        printf(\"Memory allocation failed\\n\"); \n\
        return 1; \n\
    }} \n\
    get_input(input, bufferSize); \n\
    {} = string_to_number(input); \n\
    free(input);\n", ir_variable_to_c_variable(&variable))
}

pub fn get_c_for_print_string_instruction(variable: ir::Variable) -> String {
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