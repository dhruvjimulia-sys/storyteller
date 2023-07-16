#[derive(Debug)]
pub struct LexerBlock(pub Vec<LexerToken>);

#[derive(Debug)]
pub struct LexerOutput(pub Vec<LexerBlock>);

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum LexerToken {
    Comma,
    Quote,
    Period,
    Text(String)
}