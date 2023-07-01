#[derive(Debug)]
pub struct Program(pub Vec<Block>);

#[derive(Debug)]
pub enum Statement {
    AssignmentStatement(Variable, Expression),
    AddStatement(Variable, Expression),
    SubStatement(Variable, Expression),
    PrintNumberStatement(Variable),
    PrintCharacterStatement(Variable),
    Num(i32)
}

#[derive(Debug)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug)]
struct Variable(String);

#[derive(Debug)]
enum Expression {
    NumberLiteral(i32),
    Variable(Variable)
}