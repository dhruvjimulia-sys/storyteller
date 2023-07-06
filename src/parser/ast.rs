#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Block>);

#[derive(Debug, PartialEq)]
pub enum Statement {
    AssignmentStatement(Variable, VariableOrNumberLiteral),
    AddStatement(Variable, VariableOrNumberLiteral),
    SubStatement(Variable, VariableOrNumberLiteral),
    PrintNumberStatement(Variable),
    PrintCharacterStatement(Variable),
}

#[derive(Debug, PartialEq)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, PartialEq)]
pub struct Variable(pub String);

#[derive(Debug, PartialEq)]
pub struct VariableOrNumberLiteral(pub String);