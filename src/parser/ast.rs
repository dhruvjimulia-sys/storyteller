#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Block>);

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    AssignmentStatement(Variable, VariableOrNumberLiteral),
    AddStatement(Variable, VariableOrNumberLiteral),
    SubStatement(Variable, VariableOrNumberLiteral),
    PrintNumberStatement(Variable),
    PrintCharacterStatement(Variable),
    InputStatement(Variable),
    ExitStatement,
    GotoStatement(VariableOrNumberLiteral),
    IfStatement(Condition, Box<Statement>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    EqualTo(VariableOrNumberLiteral, VariableOrNumberLiteral),
    NotEqualTo(VariableOrNumberLiteral, VariableOrNumberLiteral),
    GreaterThan(VariableOrNumberLiteral, VariableOrNumberLiteral),
    LessThan(VariableOrNumberLiteral, VariableOrNumberLiteral),
    GreaterThanOrEqualTo(VariableOrNumberLiteral, VariableOrNumberLiteral),
    LessThanOrEqualTo(VariableOrNumberLiteral, VariableOrNumberLiteral)
}

#[derive(Debug, PartialEq)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, PartialEq, Clone)]
pub struct Variable(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct VariableOrNumberLiteral(pub String);