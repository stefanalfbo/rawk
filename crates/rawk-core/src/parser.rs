use crate::{
    Lexer, Program,
    ast::{Action, Expression, Rule, Statement},
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

    fn parse_next_rule(&mut self) -> Option<Rule<'a>> {
        match &self.current_token.kind {
            TokenKind::Begin => {
                self.next_token();
                match self.parse_action() {
                    Rule::Action(action) => Some(Rule::Begin(action)),
                    _ => panic!("Expected action after BEGIN"),
                }
            }
            TokenKind::NewLine => {
                self.next_token();
                self.parse_next_rule()
            }
            TokenKind::Eof => None,
            TokenKind::LeftCurlyBrace => Some(self.parse_action()),
            TokenKind::End => {
                self.next_token();
                match self.parse_action() {
                    Rule::Action(action) => Some(Rule::End(action)),
                    _ => panic!("Expected action after END"),
                }
            }
            _ => panic!(
                "parse_next_rule not yet implemented, found token: {:?}",
                self.current_token
            ),
        }
    }

    fn parse_action(&mut self) -> Rule<'a> {
        self.next_token(); // consume '{'

        let pattern = None;

        let mut statements = Vec::new();
        while self.current_token.kind == TokenKind::NewLine {
            self.next_token();
        }

        if self.current_token.kind == TokenKind::Print {
            let print_statement = self.parse_print_function();
            statements.push(print_statement);
        }

        while self.current_token.kind != TokenKind::RightCurlyBrace
            && self.current_token.kind != TokenKind::Eof
        {
            self.next_token();
        }

        if pattern.is_some() {
            Rule::PatternAction {
                pattern,
                action: Some(Action { statements }),
            }
        } else {
            Rule::Action(Action { statements })
        }
    }

    fn parse_print_function(&mut self) -> Statement<'a> {
        let mut expressions = Vec::new();
        self.next_token();

        while self.current_token.kind != TokenKind::RightCurlyBrace
            && self.current_token.kind != TokenKind::Eof
        {
            match self.current_token.kind {
                TokenKind::String => {
                    expressions.push(Expression::String(self.current_token.literal));
                }
                TokenKind::Number => {
                    if let Ok(value) = self.current_token.literal.parse::<f64>() {
                        expressions.push(Expression::Number(value));
                    }
                }
                TokenKind::Comma => {}
                _ => {}
            }
            self.next_token();
        }

        Statement::Print(expressions)
    }

    pub fn parse_program(&mut self) -> Program<'_> {
        let mut program = Program::new();

        while !self.is_eof() {
            match self.parse_next_rule() {
                Some(Rule::Begin(action)) => program.add_begin_block(Rule::Begin(action)),
                Some(rule) => program.add_rule(rule),
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

    #[test]
    fn parse_begin_block() {
        let mut parser = Parser::new(Lexer::new("BEGIN { print }"));

        let program = parser.parse_program();

        assert_eq!(program.len(), 1);
        assert_eq!("BEGIN { print }", program.to_string());
    }

    #[test]
    fn parse_end_block() {
        let mut parser = Parser::new(Lexer::new("END { print 42 }"));

        let program = parser.parse_program();

        assert_eq!(program.len(), 1);
        assert_eq!("END { print 42 }", program.to_string());
    }
}
