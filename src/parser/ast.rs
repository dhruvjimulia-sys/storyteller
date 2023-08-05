#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Block>);

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    AssignmentStatement(Variable, VariableOrNumberLiteral),
    AddStatement(Variable, VariableOrNumberLiteral),
    SubStatement(Variable, VariableOrNumberLiteral),
    PrintNumberStatement(Variable),
    PrintStringStatement(Variable),
    InputStatement(Variable),
    ExitStatement,
    GotoStatement(VariableOrNumberLiteral),
    IfStatement(Condition, Box<Statement>),
    Comment
}

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    EqualTo(VariableOrNumberLiteral, VariableOrNumberLiteral),
    NotEqualTo(VariableOrNumberLiteral, VariableOrNumberLiteral),
    GreaterThan(VariableOrNumberLiteral, VariableOrNumberLiteral),
    LessThan(VariableOrNumberLiteral, VariableOrNumberLiteral),
}

#[derive(Debug, PartialEq)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, PartialEq, Clone)]
pub struct Variable(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct VariableOrNumberLiteral(pub String);

pub trait ASTExpression {
    fn get_name(self) -> String;
}

impl ASTExpression for Variable {
    fn get_name(self) -> String {
        match self {
            Variable(name) => { name }
        }
    }
}

impl ASTExpression for VariableOrNumberLiteral {
    fn get_name(self) -> String {
        match self {
            VariableOrNumberLiteral(name) => { name }
        }
    }
}