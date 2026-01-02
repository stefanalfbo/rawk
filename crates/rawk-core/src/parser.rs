use crate::{
    Lexer, Program,
    ast::Item,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn is_eof(&self) -> bool {
        self.current_token.kind == TokenKind::Eof
    }

    fn parse_next_item(&mut self) -> Option<Item<'a>> {
        match &self.current_token.kind {
            TokenKind::Eof => None,
            _ => panic!(
                "parse_next_item not yet implemented, found token: {:?}",
                self.current_token
            ),
        }
    }

    pub fn parse_program(&mut self) -> Program<'_> {
        let mut program = Program::new();

        while !self.is_eof() {
            match self.parse_next_item() {
                Some(item) => program.add_item(item),
                None => {}
            }
            self.next_token();
        }

        program
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

    #[test]
    fn parse_empty_program() {
        let mut parser = Parser::new(Lexer::new(""));

        let program = parser.parse_program();

        assert_eq!(program.len(), 0);
    }

    #[test]
    #[should_panic(expected = "parse_next_item not yet implemented")]
    fn panic_on_unimplemented_parse_next_item() {
        let mut parser = Parser::new(Lexer::new("42"));

        let _ = parser.parse_next_item();
    }
}
