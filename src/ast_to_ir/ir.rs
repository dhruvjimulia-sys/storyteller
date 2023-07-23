use num::BigUint;

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
    LessThan(Expression, Expression),
    GreaterThanOrEqualTo(Expression, Expression),
    LessThanOrEqualTo(Expression, Expression)
}