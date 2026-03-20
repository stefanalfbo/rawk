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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenKind};

    fn parse_error(kind: ParseErrorKind, token: Token<'static>) -> ParseError<'static> {
        ParseError { kind, token }
    }

    #[test]
    fn display_expected_rule_error() {
        let err = parse_error(
            ParseErrorKind::ExpectedRule,
            Token::new(TokenKind::Else, "else", 12),
        );

        assert_eq!(
            format!("{err}"),
            "unexpected token Else (\"else\") at byte 12: expected rule"
        );
    }

    #[test]
    fn display_expected_statement_error() {
        let err = parse_error(
            ParseErrorKind::ExpectedStatement,
            Token::new(TokenKind::Else, "else", 4),
        );

        assert_eq!(
            format!("{err}"),
            "unexpected token Else (\"else\") at byte 4: expected statement"
        );
    }

    #[test]
    fn display_expected_identifier_error() {
        let err = parse_error(
            ParseErrorKind::ExpectedIdentifier,
            Token::new(TokenKind::Number, "1", 8),
        );

        assert_eq!(
            format!("{err}"),
            "unexpected token Number (\"1\") at byte 8: expected identifier"
        );
    }

    #[test]
    fn display_expected_left_brace_error() {
        let err = parse_error(
            ParseErrorKind::ExpectedLeftBrace,
            Token::new(TokenKind::Print, "print", 6),
        );

        assert_eq!(
            format!("{err}"),
            "unexpected token Print (\"print\") at byte 6: expected left brace"
        );
    }

    #[test]
    fn display_expected_right_paren_error() {
        let err = parse_error(
            ParseErrorKind::ExpectedRightParen,
            Token::new(TokenKind::Print, "print", 10),
        );

        assert_eq!(
            format!("{err}"),
            "unexpected token Print (\"print\") at byte 10: expected right paren"
        );
    }

    #[test]
    fn display_missing_printf_format_string_error() {
        let err = parse_error(
            ParseErrorKind::MissingPrintfFormatString,
            Token::new(TokenKind::RightCurlyBrace, "}", 14),
        );

        assert_eq!(
            format!("{err}"),
            "printf requires a format string at byte 14"
        );
    }
}
