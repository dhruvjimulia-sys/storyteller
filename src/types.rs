#[derive(Debug)]
pub struct LexerStatement(pub String);
#[derive(Debug)]
pub struct LexerBlock(pub Vec<LexerStatement>);
#[derive(Debug)]
pub struct LexerOutput(pub Vec<LexerBlock>);