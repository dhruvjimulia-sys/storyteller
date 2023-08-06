use num::BigUint;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
    NumberLiteral(BigUint),
    Variable(String)
}

#[derive(Debug, Clone)]
pub enum Instruction {
    AssignmentInstruction(Variable, Expression),
    AddInstruction(Variable, Expression),
    SubInstruction(Variable, Expression),
    PrintNumberInstruction(Variable),
    PrintCharacterInstruction(Variable),
    InputInstruction(Variable),
    ExitInstruction,
    GotoInstruction(Expression),
    IfInstruction(Condition, Box<Instruction>),
    Label(BigUint)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Variable(pub String);

#[derive(Debug, Clone)]
pub enum Condition {
    EqualTo(Expression, Expression),
    NotEqualTo(Expression, Expression),
    GreaterThan(Expression, Expression),
    LessThan(Expression, Expression)
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::AssignmentInstruction(variable, expression) => {
                write!(f, "{} = {}", variable.0, expression)
            }
            Instruction::AddInstruction(variable, expression) => {
                write!(f, "{} += {}", variable.0, expression)
            }
            Instruction::SubInstruction(variable, expression) => {
                write!(f, "{} -= {}", variable.0, expression)
            }
            Instruction::PrintNumberInstruction(variable) => {
                write!(f, "print {}", variable.0)
            }
            Instruction::PrintCharacterInstruction(variable) => {
                write!(f, "printc {}", variable.0)
            }
            Instruction::InputInstruction(variable) => {
                write!(f, "input {}", variable.0)
            }
            Instruction::ExitInstruction => {
                write!(f, "exit")
            }
            Instruction::GotoInstruction(expression) => {
                write!(f, "goto {}", expression)
            }
            Instruction::IfInstruction(condition, instruction) => {
                write!(f, "if {} {}", condition, instruction)
            }
            Instruction::Label(label) => {
                write!(f, "label {}:", label)
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::NumberLiteral(number) => {
                write!(f, "{}", number)
            }
            Expression::Variable(variable) => {
                write!(f, "{}", variable)
            }
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Condition::EqualTo(left, right) => {
                write!(f, "{} == {}", left, right)
            }
            Condition::NotEqualTo(left, right) => {
                write!(f, "{} != {}", left, right)
            }
            Condition::GreaterThan(left, right) => {
                write!(f, "{} > {}", left, right)
            }
            Condition::LessThan(left, right) => {
                write!(f, "{} < {}", left, right)
            }
        }
    }
}