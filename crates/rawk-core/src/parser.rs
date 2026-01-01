use crate::{
    Lexer, Program,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token {
                kind: TokenKind::Illegal,
                literal: "<illegal>",
            },
            peek_token: Token {
                kind: TokenKind::Illegal,
                literal: "",
            },
        };

        // Read two tokens, so currentToken and peekToken are both set
        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program<'_> {
        Program { items: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_parser() {
        let parser = Parser::new(Lexer::new("42 == 42"));

        assert_eq!(parser.current_token.literal, "42");
        assert_eq!(parser.peek_token.literal, "==");
    }
}
