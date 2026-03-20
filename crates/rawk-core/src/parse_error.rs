use crate::token::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    ExpectedRule,
    ExpectedStatement,
    ExpectedIdentifier,
    ExpectedLeftBrace,
    ExpectedRightParen,
    MissingPrintfFormatString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError<'a> {
    pub kind: ParseErrorKind,
    pub token: Token<'a>,
}

impl std::fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ParseErrorKind::ExpectedRule => write!(
                f,
                "unexpected token {:?} ({:?}) at byte {}: expected rule",
                self.token.kind, self.token.literal, self.token.span.start
            ),
            ParseErrorKind::ExpectedStatement => write!(
                f,
                "unexpected token {:?} ({:?}) at byte {}: expected statement",
                self.token.kind, self.token.literal, self.token.span.start
            ),
            ParseErrorKind::ExpectedIdentifier => write!(
                f,
                "unexpected token {:?} ({:?}) at byte {}: expected identifier",
                self.token.kind, self.token.literal, self.token.span.start
            ),
            ParseErrorKind::ExpectedLeftBrace => write!(
                f,
                "unexpected token {:?} ({:?}) at byte {}: expected left brace",
                self.token.kind, self.token.literal, self.token.span.start
            ),
            ParseErrorKind::ExpectedRightParen => write!(
                f,
                "unexpected token {:?} ({:?}) at byte {}: expected right paren",
                self.token.kind, self.token.literal, self.token.span.start
            ),
            ParseErrorKind::MissingPrintfFormatString => write!(
                f,
                "printf requires a format string at byte {}",
                self.token.span.start
            ),
        }
    }
}

impl std::error::Error for ParseError<'_> {}
