#[derive(Debug)]
pub struct Program(pub Vec<Block>);

#[derive(Debug)]
pub enum Statement {
    AssignmentStatement(Variable, VariableOrNumberLiteral),
    AddStatement(Variable, VariableOrNumberLiteral),
    SubStatement(Variable, VariableOrNumberLiteral),
    PrintNumberStatement(Variable),
    PrintCharacterStatement(Variable),
    Num(i32)
}

#[derive(Debug)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug)]
pub struct Variable(pub String);

#[derive(Debug)]
pub struct VariableOrNumberLiteral(pub String);