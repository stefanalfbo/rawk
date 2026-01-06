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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub start: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub literal: &'a str,
    pub span: Location,
}

impl Token<'_> {
    pub fn new<'a>(kind: TokenKind, literal: &'a str, start: usize) -> Token<'a> {
        Token {
            kind,
            literal,
            span: Location { start },
        }
    }
}

pub fn lookup_keyword<'a>(ident: &'a str) -> Option<TokenKind> {
    match ident {
        "BEGIN" => Some(TokenKind::Begin),
        "END" => Some(TokenKind::End),
        "break" => Some(TokenKind::Break),
        "continue" => Some(TokenKind::Continue),
        "delete" => Some(TokenKind::Delete),
        "do" => Some(TokenKind::Do),
        "else" => Some(TokenKind::Else),
        "exit" => Some(TokenKind::Exit),
        "for" => Some(TokenKind::For),
        "function" => Some(TokenKind::Function),
        "if" => Some(TokenKind::If),
        "in" => Some(TokenKind::In),
        "next" => Some(TokenKind::Next),
        "print" => Some(TokenKind::Print),
        "printf" => Some(TokenKind::Printf),
        "return" => Some(TokenKind::Return),
        "while" => Some(TokenKind::While),
        _ => None,
    }
}

pub fn lookup_functions<'a>(ident: &'a str) -> Option<TokenKind> {
    match ident {
        "atan2" => Some(TokenKind::Atan2),
        "close" => Some(TokenKind::Close),
        "cos" => Some(TokenKind::Cos),
        "exp" => Some(TokenKind::Exp),
        "gsub" => Some(TokenKind::Gsub),
        "index" => Some(TokenKind::Index),
        "int" => Some(TokenKind::Int),
        "length" => Some(TokenKind::Length),
        "log" => Some(TokenKind::Log),
        "match" => Some(TokenKind::Match),
        "rand" => Some(TokenKind::Rand),
        "sin" => Some(TokenKind::Sin),
        "split" => Some(TokenKind::Split),
        "sprintf" => Some(TokenKind::Sprintf),
        "sqrt" => Some(TokenKind::Sqrt),
        "srand" => Some(TokenKind::Srand),
        "sub" => Some(TokenKind::Sub),
        "substr" => Some(TokenKind::Substr),
        "system" => Some(TokenKind::System),
        "tolower" => Some(TokenKind::ToLower),
        "toupper" => Some(TokenKind::ToUpper),
        _ => None,
    }
}
