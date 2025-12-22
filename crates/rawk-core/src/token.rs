#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,
    Number,

    // Keywords.
    Begin,
    End,
    Break,
    Continue,
    Delete,
    Do,
    Else,
    Exit,
    For,
    Function,
    If,
    In,
    Next,
    Print,
    Printf,
    Return,
    While,

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
pub struct Token<'a> {
    pub kind: TokenKind,
    pub literal: &'a str,
}

pub fn lookup_keyword<'a>(ident: &'a str) -> Token<'a> {
    match ident {
        "BEGIN" => Token {
            kind: TokenKind::Begin,
            literal: "BEGIN",
        },
        "END" => Token {
            kind: TokenKind::End,
            literal: "END",
        },
        "break" => Token {
            kind: TokenKind::Break,
            literal: "break",
        },
        "continue" => Token {
            kind: TokenKind::Continue,
            literal: "continue",
        },
        "delete" => Token {
            kind: TokenKind::Delete,
            literal: "delete",
        },
        "do" => Token {
            kind: TokenKind::Do,
            literal: "do",
        },
        "else" => Token {
            kind: TokenKind::Else,
            literal: "else",
        },
        "exit" => Token {
            kind: TokenKind::Exit,
            literal: "exit",
        },
        "for" => Token {
            kind: TokenKind::For,
            literal: "for",
        },
        "function" => Token {
            kind: TokenKind::Function,
            literal: "function",
        },
        "if" => Token {
            kind: TokenKind::If,
            literal: "if",
        },
        "in" => Token {
            kind: TokenKind::In,
            literal: "in",
        },
        "next" => Token {
            kind: TokenKind::Next,
            literal: "next",
        },
        "print" => Token {
            kind: TokenKind::Print,
            literal: "print",
        },
        "printf" => Token {
            kind: TokenKind::Printf,
            literal: "printf",
        },
        "return" => Token {
            kind: TokenKind::Return,
            literal: "return",
        },
        "while" => Token {
            kind: TokenKind::While,
            literal: "while",
        },
        _ => Token {
            kind: TokenKind::Illegal,
            literal: "<illegal>",
        },
    }
}
