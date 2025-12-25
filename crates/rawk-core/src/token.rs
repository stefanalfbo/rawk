#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Illegal,
    Eof,
    Number,
    String,

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

    // Built-in functions.
    Atan2,
    Close,
    Cos,
    Exp,
    Gsub,
    Index,
    Int,
    Length,
    Log,
    Match,
    Rand,
    Sin,
    Split,
    Sprintf,
    Sqrt,
    Srand,
    Sub,
    Substr,
    System,
    ToLower,
    ToUpper,

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
    Division,
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
    Assign,

    // Two-character tokens.
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    PowerAssign,
    Or,
    And,
    NoMatch,
    Equal,
    LessThanOrEqual,
    GreaterThanOrEqual,
    NotEqual,
    Increment,
    Decrement,
    Append,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub literal: &'a str,
}

pub fn lookup_keyword<'a>(ident: &'a str) -> Option<Token<'a>> {
    match ident {
        "BEGIN" => Some(Token {
            kind: TokenKind::Begin,
            literal: "BEGIN",
        }),
        "END" => Some(Token {
            kind: TokenKind::End,
            literal: "END",
        }),
        "break" => Some(Token {
            kind: TokenKind::Break,
            literal: "break",
        }),
        "continue" => Some(Token {
            kind: TokenKind::Continue,
            literal: "continue",
        }),
        "delete" => Some(Token {
            kind: TokenKind::Delete,
            literal: "delete",
        }),
        "do" => Some(Token {
            kind: TokenKind::Do,
            literal: "do",
        }),
        "else" => Some(Token {
            kind: TokenKind::Else,
            literal: "else",
        }),
        "exit" => Some(Token {
            kind: TokenKind::Exit,
            literal: "exit",
        }),
        "for" => Some(Token {
            kind: TokenKind::For,
            literal: "for",
        }),
        "function" => Some(Token {
            kind: TokenKind::Function,
            literal: "function",
        }),
        "if" => Some(Token {
            kind: TokenKind::If,
            literal: "if",
        }),
        "in" => Some(Token {
            kind: TokenKind::In,
            literal: "in",
        }),
        "next" => Some(Token {
            kind: TokenKind::Next,
            literal: "next",
        }),
        "print" => Some(Token {
            kind: TokenKind::Print,
            literal: "print",
        }),
        "printf" => Some(Token {
            kind: TokenKind::Printf,
            literal: "printf",
        }),
        "return" => Some(Token {
            kind: TokenKind::Return,
            literal: "return",
        }),
        "while" => Some(Token {
            kind: TokenKind::While,
            literal: "while",
        }),
        _ => None,
    }
}

pub fn lookup_functions<'a>(ident: &'a str) -> Option<Token<'a>> {
    match ident {
        "atan2" => Some(Token {
            kind: TokenKind::Atan2,
            literal: "atan2",
        }),
        "close" => Some(Token {
            kind: TokenKind::Close,
            literal: "close",
        }),
        "cos" => Some(Token {
            kind: TokenKind::Cos,
            literal: "cos",
        }),
        "exp" => Some(Token {
            kind: TokenKind::Exp,
            literal: "exp",
        }),
        "gsub" => Some(Token {
            kind: TokenKind::Gsub,
            literal: "gsub",
        }),
        "index" => Some(Token {
            kind: TokenKind::Index,
            literal: "index",
        }),
        "int" => Some(Token {
            kind: TokenKind::Int,
            literal: "int",
        }),
        "length" => Some(Token {
            kind: TokenKind::Length,
            literal: "length",
        }),
        "log" => Some(Token {
            kind: TokenKind::Log,
            literal: "log",
        }),
        "match" => Some(Token {
            kind: TokenKind::Match,
            literal: "match",
        }),
        "rand" => Some(Token {
            kind: TokenKind::Rand,
            literal: "rand",
        }),
        "sin" => Some(Token {
            kind: TokenKind::Sin,
            literal: "sin",
        }),
        "split" => Some(Token {
            kind: TokenKind::Split,
            literal: "split",
        }),
        "sprintf" => Some(Token {
            kind: TokenKind::Sprintf,
            literal: "sprintf",
        }),
        "sqrt" => Some(Token {
            kind: TokenKind::Sqrt,
            literal: "sqrt",
        }),
        "srand" => Some(Token {
            kind: TokenKind::Srand,
            literal: "srand",
        }),
        "sub" => Some(Token {
            kind: TokenKind::Sub,
            literal: "sub",
        }),
        "substr" => Some(Token {
            kind: TokenKind::Substr,
            literal: "substr",
        }),
        "system" => Some(Token {
            kind: TokenKind::System,
            literal: "system",
        }),
        "tolower" => Some(Token {
            kind: TokenKind::ToLower,
            literal: "tolower",
        }),
        "toupper" => Some(Token {
            kind: TokenKind::ToUpper,
            literal: "toupper",
        }),
        _ => None,
    }
}
