#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,

    // Delimiters
    LeftCurlyBrace,
    RightCurlyBrace,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: &'static str,
}
