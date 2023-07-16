#[derive(Debug)]
pub struct LexerStatement(pub Vec<LexerToken>);

#[derive(Debug)]
pub struct LexerBlock(pub Vec<LexerStatement>);

#[derive(Debug)]
pub struct LexerOutput(pub Vec<LexerBlock>);

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum LexerToken {
    Comma,
    Quote,
    Text(String)
}