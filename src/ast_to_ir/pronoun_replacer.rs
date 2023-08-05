use crate::parser::ast;
use crate::compiler_errors;
use std::collections::HashSet;
use crate::ast_to_ir::ir;

pub fn replace_pronouns(ast: &ast::Program, variables: &HashSet<ir::Variable>) -> ast::Program {
    fn replace_pronoun_in_var<'a>(variable: ast::Variable, curr: Option<ast::Variable>, pronouns: &'a HashSet<&str>) -> (ast::Variable, Option<ast::Variable>) {
        match variable {
            ast::Variable(name) => {
                if pronouns.contains(&*name.clone()) {
                    match curr {
                        Some(ref curr_var) => {
                            (ast::Variable(curr_var.0.clone()), curr)
                        }
                        None => {
                            compiler_errors::PRONOUN_NO_ANTECEDENT_ERROR.display();
                            (ast::Variable("".to_string()), None)
                        }
                    }
                } else {
                    (ast::Variable(name.clone()), Some(ast::Variable(name)))
                }
            }
        }
    }

    fn replace_pronoun_in_var_or_num_literal<'a>(var_or_num: ast::VariableOrNumberLiteral, curr: Option<ast::Variable>, pronouns: &'a HashSet<&str>, variables: &HashSet<ir::Variable>) -> (ast::VariableOrNumberLiteral, Option<ast::Variable>) {
        match var_or_num {
            ast::VariableOrNumberLiteral(name) => {
                if pronouns.contains(&*name) {
                    match curr {
                        Some(ref curr_var) => {
                            (ast::VariableOrNumberLiteral(curr_var.0.clone()), curr)
                        }
                        None => {
                            compiler_errors::PRONOUN_NO_ANTECEDENT_ERROR.display();
                            (ast::VariableOrNumberLiteral("".to_string()), None)
                        }
                    }
                } else {
                    if variables.contains(&ir::Variable(name.clone())) {
                        (ast::VariableOrNumberLiteral(name.clone()), Some(ast::Variable(name)))
                    } else {
                        (ast::VariableOrNumberLiteral(name.clone()), curr)
                    }
                }
            }
        }
    }

    fn replace_pronouns_in_condition<'a>(curr: Option<ast::Variable>, condition: ast::Condition, pronouns: &'a HashSet<&'a str>, variables: &'a HashSet<ir::Variable>) -> (ast::Condition, Option<ast::Variable>) {
        match condition {
            ast::Condition::EqualTo(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(lhs, curr, &pronouns, variables);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Condition::EqualTo(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            },
            ast::Condition::NotEqualTo(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(lhs, curr, &pronouns, variables);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Condition::NotEqualTo(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            },
            ast::Condition::GreaterThan(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(lhs, curr, &pronouns, variables);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Condition::GreaterThan(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            },
            ast::Condition::GreaterThanOrEqualTo(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(lhs, curr, &pronouns, variables);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Condition::GreaterThanOrEqualTo(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            },
            ast::Condition::LessThanOrEqualTo(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(lhs, curr, &pronouns, variables);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Condition::LessThanOrEqualTo(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            },
            ast::Condition::LessThan(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(lhs, curr, &pronouns, variables);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Condition::LessThan(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            }
        }
    }

    fn replace_pronouns_in_statement<'a>(curr: Option<ast::Variable>, statement: ast::Statement, pronouns: &'a HashSet<&str>, variables: &'a HashSet<ir::Variable>) -> (ast::Statement, Option<ast::Variable>) {
        match statement {
            ast::Statement::AssignmentStatement(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var(lhs, curr, &pronouns);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Statement::AssignmentStatement(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            }
            ast::Statement::AddStatement(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var(lhs, curr, &pronouns);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Statement::AddStatement(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            }
            ast::Statement::SubStatement(lhs, rhs) => {
                let (lhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var(lhs, curr, &pronouns);
                let (rhs_pronoun_replacement, new_curr_var) = replace_pronoun_in_var_or_num_literal(rhs, new_curr_var, &pronouns, variables);
                (ast::Statement::SubStatement(lhs_pronoun_replacement, rhs_pronoun_replacement), new_curr_var)
            }
            ast::Statement::PrintNumberStatement(var) => {
                let (pronoun_replacement, new_curr_var) = replace_pronoun_in_var(var, curr, &pronouns);
                (ast::Statement::PrintNumberStatement(pronoun_replacement), new_curr_var)
            }
            ast::Statement::PrintStringStatement(var) => {
                let (pronoun_replacement, new_curr_var) = replace_pronoun_in_var(var, curr, &pronouns);
                (ast::Statement::PrintStringStatement(pronoun_replacement), new_curr_var)
            }
            ast::Statement::InputStatement(var) => {
                let (pronoun_replacement, new_curr_var) = replace_pronoun_in_var(var, curr, &pronouns);
                (ast::Statement::InputStatement(pronoun_replacement), new_curr_var)
            }
            ast::Statement::IfStatement(condition, inner_statement) => {
                let (condition_with_pronoun_replaced, new_curr_var) = replace_pronouns_in_condition(curr, condition, pronouns, variables);
                let (inner_statement_with_pronoun_replaced, new_curr_var) = replace_pronouns_in_statement(new_curr_var, *inner_statement, pronouns, variables);
                (ast::Statement::IfStatement(condition_with_pronoun_replaced, Box::new(inner_statement_with_pronoun_replaced)), new_curr_var)
            }
            ast::Statement::GotoStatement(var_or_num) => {
                let (label_with_pronoun_replaced, new_curr_var) = replace_pronoun_in_var_or_num_literal(var_or_num, curr, &pronouns, &variables);
                (ast::Statement::GotoStatement(label_with_pronoun_replaced), new_curr_var)
            }
            _ => (statement, curr)
        }
    }

    let pronouns = HashSet::from(["he", "she", "they", "him", "her", "them", "himself", "herself", "themself", "themselves"]);
    let mut curr_var: Option<ast::Variable> = None;
    ast::Program(ast.0.iter().map(|block| {
        ast::Block(block.0.iter().map(|statement| {
            let (statement_with_pronoun_replaced, new_curr_var) = replace_pronouns_in_statement(curr_var.clone(), statement.clone(), &pronouns, variables);
            curr_var = new_curr_var.clone();
            statement_with_pronoun_replaced
        }).collect::<Vec<_>>())
    }).collect::<Vec<_>>())
}