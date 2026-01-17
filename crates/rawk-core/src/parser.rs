use crate::{
    Lexer, Program,
    ast::{Action, Item, Statement},
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
            TokenKind::NewLine => {
                self.next_token();
                self.parse_next_item()
            }
            TokenKind::Eof => None,
            TokenKind::LeftCurlyBrace => Some(self.parse_action()),
            _ => panic!(
                "parse_next_item not yet implemented, found token: {:?}",
                self.current_token
            ),
        }
    }

    fn parse_action(&mut self) -> Item<'a> {
        self.next_token(); // consume '{'

        let mut statements = Vec::new();
        while self.current_token.kind == TokenKind::NewLine {
            self.next_token();
        }
        if self.current_token.kind == TokenKind::Print {
            statements.push(Statement::Print(vec![]));
        }

        while self.current_token.kind != TokenKind::RightCurlyBrace
            && self.current_token.kind != TokenKind::Eof
        {
            self.next_token();
        }

        Item::PatternAction {
            pattern: None,
            action: Some(Action { statements }),
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
    fn parse_action_without_pattern() {
        let mut parser = Parser::new(Lexer::new("{ print }"));

        let program = parser.parse_program();

        assert_eq!(program.len(), 1);
        assert_eq!("{ print }", program.to_string());
    }

    #[test]
    fn parse_action_with_leading_newlines() {
        let mut parser = Parser::new(Lexer::new("\n\n{ print }"));

        let program = parser.parse_program();

        assert_eq!(program.len(), 1);
        assert_eq!("{ print }", program.to_string());
    }
}
