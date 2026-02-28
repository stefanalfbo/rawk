use crate::{
    Lexer, Program,
    ast::{Action, Expression, Rule, Statement},
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
}

enum CommaBehavior {
    InsertSpaceExpression,
    SkipComma,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        // Enable regex parsing for the first token since it could be a pattern
        lexer.set_allow_regex(true);
        let current_token = lexer.next_token();
        lexer.set_allow_regex(false);

        Parser {
            lexer,
            current_token,
        }
    }

    fn next_token(&mut self) {
        self.next_token_with_regex(false);
    }

    fn next_token_with_regex(&mut self, allow_regex: bool) {
        self.lexer.set_allow_regex(allow_regex);
        self.current_token = self.lexer.next_token();
        self.lexer.set_allow_regex(false);
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
                self.next_token_with_regex(true);
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
            TokenKind::Regex
            | TokenKind::String
            | TokenKind::Number
            | TokenKind::DollarSign
            | TokenKind::LeftParen
            | TokenKind::Identifier => self.parse_pattern_rule(),
            _ => panic!(
                "parse_next_rule not yet implemented, found token: {:?}",
                self.current_token
            ),
        }
    }

    fn parse_pattern_rule(&mut self) -> Option<Rule<'a>> {
        let mut pattern = self.parse_expression();
        if self.current_token.kind == TokenKind::Comma {
            let operator = self.current_token.clone();
            self.next_token_with_regex(true);
            let right = self.parse_expression();
            pattern = Expression::Infix {
                left: Box::new(pattern),
                operator,
                right: Box::new(right),
            };
        }
        let pattern = Some(pattern);

        if self.current_token.kind == TokenKind::LeftCurlyBrace {
            match self.parse_action() {
                Rule::Action(action) => Some(Rule::PatternAction {
                    pattern,
                    action: Some(action),
                }),
                _ => panic!("Expected action after pattern"),
            }
        } else {
            Some(Rule::PatternAction {
                pattern,
                action: None,
            })
        }
    }

    fn parse_action(&mut self) -> Rule<'a> {
        self.next_token(); // consume '{'

        let pattern = None;

        let mut statements = Vec::new();
        while self.current_token.kind != TokenKind::RightCurlyBrace
            && self.current_token.kind != TokenKind::Eof
        {
            while self.current_token.kind == TokenKind::NewLine
                || self.current_token.kind == TokenKind::Semicolon
            {
                self.next_token();
            }

            if self.current_token.kind == TokenKind::RightCurlyBrace
                || self.current_token.kind == TokenKind::Eof
            {
                break;
            }

            statements.push(self.parse_statement());
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

    fn parse_statement(&mut self) -> Statement<'a> {
        match self.current_token.kind {
            TokenKind::Print => self.parse_print_function(),
            TokenKind::Printf => self.parse_printf_function(),
            TokenKind::Gsub => self.parse_gsub_function(),
            TokenKind::Identifier => self.parse_assignment_statement(),
            TokenKind::Increment => self.parse_pre_increment_statement(),
            _ => todo!(),
        }
    }

    fn parse_assignment_statement(&mut self) -> Statement<'a> {
        let identifier = self.current_token.literal;
        self.next_token();
        if self.current_token.kind == TokenKind::Assign {
            self.next_token();
            let value = self.parse_expression();
            Statement::Assignment { identifier, value }
        } else if self.current_token.kind == TokenKind::AddAssign {
            self.next_token();
            let value = self.parse_expression();
            Statement::AddAssignment { identifier, value }
        } else {
            todo!()
        }
    }

    fn parse_pre_increment_statement(&mut self) -> Statement<'a> {
        self.next_token();
        if self.current_token.kind != TokenKind::Identifier {
            todo!()
        }
        let identifier = self.current_token.literal;
        self.next_token();
        Statement::PreIncrement { identifier }
    }

    fn parse_print_function(&mut self) -> Statement<'a> {
        let expressions = self.parse_expression_list_until_action_end(CommaBehavior::InsertSpaceExpression);

        Statement::Print(expressions)
    }

    fn parse_printf_function(&mut self) -> Statement<'a> {
        let expressions = self.parse_expression_list_until_action_end(CommaBehavior::SkipComma);

        Statement::Printf(expressions)
    }

    fn parse_gsub_function(&mut self) -> Statement<'a> {
        self.next_token();
        if self.current_token.kind != TokenKind::LeftParen {
            todo!()
        }

        self.next_token_with_regex(true);
        let pattern = self.parse_expression();

        if self.current_token.kind != TokenKind::Comma {
            todo!()
        }
        self.next_token();
        let replacement = self.parse_expression();

        if self.current_token.kind == TokenKind::Comma {
            todo!()
        }

        if self.current_token.kind != TokenKind::RightParen {
            todo!()
        }
        self.next_token();

        Statement::Gsub {
            pattern,
            replacement,
        }
    }

    fn parse_expression_list_until_action_end(
        &mut self,
        comma_behavior: CommaBehavior,
    ) -> Vec<Expression<'a>> {
        let mut expressions = Vec::new();
        let mut expect_more = false;
        self.next_token();

        loop {
            if self.current_token.kind == TokenKind::RightCurlyBrace
                || self.current_token.kind == TokenKind::Eof
            {
                break;
            }

            if self.current_token.kind == TokenKind::NewLine
                || self.current_token.kind == TokenKind::Semicolon
            {
                if expect_more {
                    self.next_token();
                    continue;
                }
                break;
            }

            if self.current_token.kind == TokenKind::Comma {
                self.next_token();
                if let CommaBehavior::InsertSpaceExpression = comma_behavior {
                    expressions.push(Expression::String(" "));
                }
                expect_more = true;
                continue;
            }

            let expression = self.parse_expression();
            expressions.push(expression);
            expect_more = false;
        }

        expressions
    }

    fn parse_expression(&mut self) -> Expression<'a> {
        self.parse_expression_with_min_precedence(0)
    }

    fn parse_expression_with_min_precedence(&mut self, min_precedence: u8) -> Expression<'a> {
        let mut left = self.parse_primary_expression();

        loop {
            let (left_precedence, right_precedence) =
                match infix_operator_precedence(&self.current_token.kind) {
                    Some(value) => value,
                    None => break,
                };

            if left_precedence < min_precedence {
                break;
            }

            let operator = self.current_token.clone();
            if matches!(
                operator.kind,
                TokenKind::Tilde | TokenKind::NoMatch | TokenKind::And | TokenKind::Or
            ) {
                self.next_token_with_regex(true);
            } else {
                self.next_token();
            }
            let right = self.parse_expression_with_min_precedence(right_precedence);

            left = Expression::Infix {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary_expression(&mut self) -> Expression<'a> {
        match self.current_token.kind {
            TokenKind::String => {
                let expression = Expression::String(self.current_token.literal);
                self.next_token();
                expression
            }
            TokenKind::Regex => {
                let expression = Expression::Regex(self.current_token.literal);
                self.next_token();
                expression
            }
            TokenKind::Number => {
                let expression = if let Ok(value) = self.current_token.literal.parse::<f64>() {
                    Expression::Number(value)
                } else {
                    todo!()
                };
                self.next_token();
                expression
            }
            TokenKind::DollarSign => {
                self.next_token();
                let expression = self.parse_primary_expression();
                Expression::Field(Box::new(expression))
            }
            TokenKind::LeftParen => {
                self.next_token();
                let expression = self.parse_expression();
                if self.current_token.kind == TokenKind::RightParen {
                    self.next_token();
                }
                expression
            }
            TokenKind::Identifier => {
                let identifier = self.current_token.literal;
                self.next_token();
                Expression::Identifier(identifier)
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn parse_program(&mut self) -> Program<'_> {
        let mut program = Program::new();

        while !self.is_eof() {
            match self.parse_next_rule() {
                Some(Rule::Begin(action)) => program.add_begin_block(Rule::Begin(action)),
                Some(Rule::End(action)) => program.add_end_block(Rule::End(action)),
                Some(rule) => program.add_rule(rule),
                None => {}
            }
            self.next_token_with_regex(true);
        }

        program
    }
}

fn infix_operator_precedence(kind: &TokenKind) -> Option<(u8, u8)> {
    match kind {
        TokenKind::Or => Some((1, 2)),
        TokenKind::And => Some((3, 4)),
        TokenKind::Equal
        | TokenKind::NotEqual
        | TokenKind::GreaterThan
        | TokenKind::GreaterThanOrEqual
        | TokenKind::LessThan
        | TokenKind::LessThanOrEqual
        | TokenKind::Tilde
        | TokenKind::NoMatch => Some((5, 6)),
        TokenKind::Plus | TokenKind::Minus => Some((7, 8)),
        TokenKind::Asterisk | TokenKind::Division | TokenKind::Percent => Some((9, 10)),
        TokenKind::Caret => Some((13, 12)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_parser() {
        let mut parser = Parser::new(Lexer::new("42 == 42"));

        assert_eq!(parser.current_token.literal, "42");
        parser.next_token();
        assert_eq!(parser.current_token.literal, "==");
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

    #[test]
    fn parse_regex_pattern_action() {
        let mut parser = Parser::new(Lexer::new("/foo/ { print }"));

        let program = parser.parse_program();

        assert_eq!(program.len(), 1);
        assert_eq!("/foo/ { print }", program.to_string());
    }

    #[test]
    fn parse_print_infix_expression() {
        let mut parser = Parser::new(Lexer::new("BEGIN { print 1 + 2 }"));

        let program = parser.parse_program();
        let mut begin_blocks = program.begin_blocks_iter();
        let rule = begin_blocks.next().expect("expected begin block");

        let statements = match rule {
            Rule::Begin(Action { statements }) => statements,
            _ => panic!("expected begin rule"),
        };

        let exprs = match &statements[0] {
            Statement::Print(expressions) => expressions,
            _ => panic!("expected print statement"),
        };

        match &exprs[0] {
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                assert!(matches!(**left, Expression::Number(1.0)));
                assert_eq!(operator.kind, TokenKind::Plus);
                assert!(matches!(**right, Expression::Number(2.0)));
            }
            _ => panic!("expected infix expression"),
        }
    }

    #[test]
    fn parse_print_parenthesized_expression() {
        let mut parser = Parser::new(Lexer::new("BEGIN { print (1 + 2) * 3 }"));

        let program = parser.parse_program();
        let mut begin_blocks = program.begin_blocks_iter();
        let rule = begin_blocks.next().expect("expected begin block");

        let statements = match rule {
            Rule::Begin(Action { statements }) => statements,
            _ => panic!("expected begin rule"),
        };

        let exprs = match &statements[0] {
            Statement::Print(expressions) => expressions,
            _ => panic!("expected print statement"),
        };

        match &exprs[0] {
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.kind, TokenKind::Asterisk);
                assert!(matches!(**right, Expression::Number(3.0)));
                assert!(matches!(**left, Expression::Infix { .. }));
            }
            _ => panic!("expected infix expression"),
        }
    }

    #[test]
    fn parse_print_multiplication_has_higher_precedence_than_addition() {
        let mut parser = Parser::new(Lexer::new("BEGIN { print 1 + 2 * 3 }"));

        let program = parser.parse_program();
        let mut begin_blocks = program.begin_blocks_iter();
        let rule = begin_blocks.next().expect("expected begin block");

        let statements = match rule {
            Rule::Begin(Action { statements }) => statements,
            _ => panic!("expected begin rule"),
        };

        let exprs = match &statements[0] {
            Statement::Print(expressions) => expressions,
            _ => panic!("expected print statement"),
        };

        match &exprs[0] {
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.kind, TokenKind::Plus);
                assert!(matches!(**left, Expression::Number(1.0)));
                match &**right {
                    Expression::Infix {
                        operator: right_op, ..
                    } => assert_eq!(right_op.kind, TokenKind::Asterisk),
                    _ => panic!("expected nested infix expression"),
                }
            }
            _ => panic!("expected infix expression"),
        }
    }

    #[test]
    fn parse_print_power_is_right_associative() {
        let mut parser = Parser::new(Lexer::new("BEGIN { print 2 ^ 3 ^ 2 }"));

        let program = parser.parse_program();
        let mut begin_blocks = program.begin_blocks_iter();
        let rule = begin_blocks.next().expect("expected begin block");

        let statements = match rule {
            Rule::Begin(Action { statements }) => statements,
            _ => panic!("expected begin rule"),
        };

        let exprs = match &statements[0] {
            Statement::Print(expressions) => expressions,
            _ => panic!("expected print statement"),
        };

        match &exprs[0] {
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.kind, TokenKind::Caret);
                assert!(matches!(**left, Expression::Number(2.0)));
                match &**right {
                    Expression::Infix {
                        operator: right_op, ..
                    } => assert_eq!(right_op.kind, TokenKind::Caret),
                    _ => panic!("expected nested infix expression"),
                }
            }
            _ => panic!("expected infix expression"),
        }
    }

    #[test]
    fn parse_print_minus_is_left_associative() {
        let mut parser = Parser::new(Lexer::new("BEGIN { print 5 - 3 - 1 }"));

        let program = parser.parse_program();
        let mut begin_blocks = program.begin_blocks_iter();
        let rule = begin_blocks.next().expect("expected begin block");

        let statements = match rule {
            Rule::Begin(Action { statements }) => statements,
            _ => panic!("expected begin rule"),
        };

        let exprs = match &statements[0] {
            Statement::Print(expressions) => expressions,
            _ => panic!("expected print statement"),
        };

        match &exprs[0] {
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.kind, TokenKind::Minus);
                match &**left {
                    Expression::Infix {
                        operator: left_op, ..
                    } => assert_eq!(left_op.kind, TokenKind::Minus),
                    _ => panic!("expected nested infix expression"),
                }
                assert!(matches!(**right, Expression::Number(1.0)));
            }
            _ => panic!("expected infix expression"),
        }
    }

    #[test]
    fn parse_print_concatenation() {
        let mut parser = Parser::new(Lexer::new(r#"BEGIN { print "Value:" 42 }"#));

        let program = parser.parse_program();
        let mut begin_blocks = program.begin_blocks_iter();
        let rule = begin_blocks.next().expect("expected begin block");

        let statements = match rule {
            Rule::Begin(Action { statements }) => statements,
            _ => panic!("expected begin rule"),
        };

        let exprs = match &statements[0] {
            Statement::Print(expressions) => expressions,
            _ => panic!("expected print statement"),
        };

        assert_eq!(exprs.len(), 2);
        assert!(matches!(exprs[0], Expression::String("Value:")));
        assert!(matches!(exprs[1], Expression::Number(42.0)));
    }

    #[test]
    fn parse_print_field_expression() {
        let mut parser = Parser::new(Lexer::new("{ print $1 }"));

        let program = parser.parse_program();
        let mut rules = program.rules_iter();
        let rule = rules.next().expect("expected rule");

        let statements = match rule {
            Rule::Action(Action { statements }) => statements,
            _ => panic!("expected action rule"),
        };

        let exprs = match &statements[0] {
            Statement::Print(expressions) => expressions,
            _ => panic!("expected print statement"),
        };

        match &exprs[0] {
            Expression::Field(inner) => assert!(matches!(**inner, Expression::Number(1.0))),
            _ => panic!("expected field expression"),
        }
    }

    #[test]
    fn parse_print_with_commas() {
        let mut parser = Parser::new(Lexer::new(r#"BEGIN { print "Value:", 42, $1 }"#));

        let program = parser.parse_program();

        assert_eq!(r#"BEGIN { print "Value:", 42, $1 }"#, program.to_string());
    }

    #[test]
    fn parse_number_of_fields_identifier() {
        let mut parser = Parser::new(Lexer::new(r#"BEGIN { print NF }"#));

        let program = parser.parse_program();

        assert_eq!(r#"BEGIN { print NF }"#, program.to_string());
    }

    #[test]
    fn parse_printf_with_format_and_arguments() {
        let mut parser = Parser::new(Lexer::new(r#"{ printf "[%10s] [%-16d]\n", $1, $3 }"#));

        let program = parser.parse_program();

        assert_eq!(
            r#"{ printf "[%10s] [%-16d]\n", $1, $3 }"#,
            program.to_string()
        );
    }

    #[test]
    fn parse_add_assignment_and_pre_increment() {
        let mut parser = Parser::new(Lexer::new(r#"/Asia/ { pop += $3; ++n }"#));

        let program = parser.parse_program();

        assert_eq!(r#"/Asia/ { pop += $3; ++n }"#, program.to_string());
    }

    #[test]
    fn parse_regex_match_pattern_action() {
        let mut parser = Parser::new(Lexer::new(r#"$4 ~ /Asia/ { print $1 }"#));

        let program = parser.parse_program();

        assert_eq!(r#"$4 ~ /Asia/ { print $1 }"#, program.to_string());
    }

    #[test]
    fn parse_print_with_line_continuation_after_comma() {
        let mut parser = Parser::new(Lexer::new(
            "END { print \"population of\", n,\\\n\"Asian countries in millions is\", pop }",
        ));

        let program = parser.parse_program();

        assert_eq!(
            "END { print \"population of\", n, \"Asian countries in millions is\", pop }",
            program.to_string()
        );
    }

    #[test]
    fn parse_gsub_statement() {
        let mut parser = Parser::new(Lexer::new(r#"{ gsub(/USA/, "United States"); print }"#));

        let program = parser.parse_program();

        assert_eq!(
            r#"{ gsub(/USA/, "United States"); print }"#,
            program.to_string()
        );
    }
}
