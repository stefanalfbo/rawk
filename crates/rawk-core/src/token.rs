#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,

    // One-character tokens.
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftParen,
    RightParen,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Semicolon,
    NewLine,
    Plus,
    Minus,
    Asterisk,
    Percent,
    Caret,
    ExclamationMark,
    GreaterThan,
    LessThan,
    Pipe,
    QuestionMark,
    Colon,
    Tilde,
    DollarSign,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: &'static str,
}
