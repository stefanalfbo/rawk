use crate::{
    Action, Program, Rule,
    ast::{Expression, Statement},
    token::TokenKind,
};
use std::cell::Cell;

pub struct Evaluator<'a> {
    program: Program<'a>,
    input_lines: Vec<String>,
    current_line_number: Cell<usize>,
    current_line: Option<String>,
}

impl<'a> Evaluator<'a> {
    pub fn new(program: Program<'a>, input_lines: Vec<String>) -> Self {
        Self {
            program,
            input_lines,
            current_line_number: Cell::new(0),
            current_line: None,
        }
    }

    pub fn eval(&mut self) -> Vec<String> {
        let mut output_lines: Vec<String> = Vec::new();

        for rule in self.program.begin_blocks_iter() {
            output_lines.extend(self.eval_begin_rule(rule));
        }

        let rules: Vec<Rule<'a>> = self.program.rules_iter().cloned().collect();
        for rule in rules.iter() {
            output_lines.extend(self.eval_rule(rule));
        }

        for rule in self.program.end_blocks_iter() {
            output_lines.extend(self.eval_end_rule(rule));
        }

        output_lines
    }

    fn eval_rule(&mut self, rule: &Rule) -> Vec<String> {
        let mut output_lines = Vec::new();

        for (i, input_line) in self.input_lines.iter().enumerate() {
            self.current_line_number.set(i + 1);
            self.current_line = Some(input_line.clone());

            if let Rule::Action(action) = rule {
                output_lines.push(self.eval_action(action, Some(input_line)));
            }
        }

        self.current_line = None;
        output_lines
    }

    fn eval_begin_rule(&self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::Begin(action) => vec![self.eval_action(action, None)],
            _ => Vec::new(),
        }
    }

    fn eval_end_rule(&self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::End(action) => vec![self.eval_action(action, None)],
            _ => Vec::new(),
        }
    }

    fn eval_action(&self, action: &Action, input_line: Option<&str>) -> String {
        if action.statements.len() == 1 {
            let statement = &action.statements[0];

            match statement {
                Statement::Print(expressions) => {
                    if expressions.is_empty() {
                        return input_line.unwrap_or("").to_string();
                    }

                    let parts = expressions
                        .iter()
                        .map(|expr| self.eval_expression(expr))
                        .collect::<Vec<String>>();
                    parts.join("")
                }
            }
        } else {
            "not implemented".to_string()
        }
    }

    fn eval_expression(&self, expression: &Expression) -> String {
        match expression {
            Expression::String(value) => value.to_string(),
            Expression::Number(value) => value.to_string(),
            Expression::Regex(value) => value.to_string(),
            Expression::Field(inner) => self.eval_field_expression(inner),
            Expression::Identifier(identifier) => self.eval_identifier_expression(identifier),
            Expression::Infix {
                left,
                operator,
                right,
            } => self
                .eval_numeric_infix(left, operator, right)
                .map(|value| value.to_string())
                .unwrap_or_else(|| "not implemented".to_string()),
        }
    }

    fn eval_identifier_expression(&self, identifier: &str) -> String {
        match identifier {
            "NF" => {
                let line = match self.current_line.as_deref() {
                    Some(value) => value,
                    None => return "0".to_string(),
                };

                let field_count = line.split_whitespace().count();
                field_count.to_string()
            }
            "NR" => match self.current_line.as_ref() {
                Some(_) => self.current_line_number.get().to_string(),
                None => "0".to_string(),
            },
            _ => "".to_string(),
        }
    }

    fn eval_field_expression(&self, expression: &Expression<'_>) -> String {
        let line = match self.current_line.as_deref() {
            Some(value) => value,
            None => return String::new(),
        };

        let index = match self.eval_numeric_expression(expression) {
            Some(value) => value as i64,
            None => return String::new(),
        };

        if index == 0 {
            return line.to_string();
        }

        if index < 0 {
            return String::new();
        }

        line.split_whitespace()
            .nth((index - 1) as usize)
            .unwrap_or("")
            .to_string()
    }

    fn eval_numeric_infix(
        &self,
        left: &Expression<'_>,
        operator: &crate::token::Token<'_>,
        right: &Expression<'_>,
    ) -> Option<f64> {
        let left_value = self.eval_numeric_expression(left)?;
        let right_value = self.eval_numeric_expression(right)?;

        match operator.kind {
            TokenKind::Plus => Some(left_value + right_value),
            TokenKind::Minus => Some(left_value - right_value),
            TokenKind::Asterisk => Some(left_value * right_value),
            TokenKind::Division => Some(left_value / right_value),
            TokenKind::Percent => Some(left_value % right_value),
            TokenKind::Caret => Some(left_value.powf(right_value)),
            _ => None,
        }
    }

    fn eval_numeric_expression(&self, expression: &Expression<'_>) -> Option<f64> {
        match expression {
            Expression::Number(value) => Some(*value),
            Expression::Identifier(identifier) => self
                .eval_identifier_expression(identifier)
                .parse::<f64>()
                .ok(),
            Expression::Field(inner) => self.eval_field_expression(inner).parse::<f64>().ok(),
            Expression::Infix {
                left,
                operator,
                right,
            } => self.eval_numeric_infix(left, operator, right),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Lexer, Parser};

    use super::*;

    #[test]
    fn eval_print_action_outputs_input_line() {
        let lexer = Lexer::new("{ print }");
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["hello, world!".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "hello, world!");
    }

    #[test]
    fn eval_begin_print_string_literal() {
        let lexer = Lexer::new(r#"BEGIN { print "hello" }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["hello".to_string()]);
    }

    #[test]
    fn eval_end_print_string_literal() {
        let lexer = Lexer::new(r#"END { print "42" } { print "hello" }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one row".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["hello".to_string(), "42".to_string()]);
    }

    #[test]
    fn eval_print_numeric_plus_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 1 + 2 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["3".to_string()]);
    }

    #[test]
    fn eval_print_numberic_multiply_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 2 * 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["6".to_string()]);
    }

    #[test]
    fn eval_print_numeric_mod_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 5 % 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["2".to_string()]);
    }

    #[test]
    fn eval_print_numeric_div_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 5 / 5 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["1".to_string()]);
    }

    #[test]
    fn eval_print_numeric_caret_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 2 ^ 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["8".to_string()]);
    }

    #[test]
    fn eval_print_numeric_minus_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 5 - 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["2".to_string()]);
    }

    #[test]
    fn eval_print_string_and_number_expressions() {
        let lexer = Lexer::new(r#"BEGIN { print "Value:" 42 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["Value:42".to_string()]);
    }

    #[test]
    fn eval_print_expression_with_parantheses() {
        let lexer = Lexer::new(r#"BEGIN { print (1 + 2) * 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["9".to_string()]);
    }

    #[test]
    fn eval_print_multiplication_has_higher_precedence_than_addition() {
        let lexer = Lexer::new(r#"BEGIN { print 1 + 2 * 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["7".to_string()]);
    }

    #[test]
    fn eval_print_power_is_right_associative() {
        let lexer = Lexer::new(r#"BEGIN { print 2 ^ 3 ^ 2 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["512".to_string()]);
    }

    #[test]
    fn eval_print_minus_is_left_associative() {
        let lexer = Lexer::new(r#"BEGIN { print 5 - 3 - 1 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["1".to_string()]);
    }

    #[test]
    fn eval_print_field_zero_returns_entire_line() {
        let lexer = Lexer::new(r#"{ print $0 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one two".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["one two".to_string()]);
    }

    #[test]
    fn eval_print_field_first_column() {
        let lexer = Lexer::new(r#"{ print $1, $3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one     two three".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["one three".to_string()]);
    }

    #[test]
    fn eval_print_number_of_fields_identifier() {
        let lexer = Lexer::new(r#"{ print NF, $1 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one two three".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["3 one".to_string()]);
    }

    #[test]
    fn eval_print_number_of_fields_on_empty_line() {
        let lexer = Lexer::new(r#"{ print NF }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["0".to_string()]);
    }

    #[test]
    fn eval_print_uninitialized_identifier() {
        let lexer = Lexer::new(r#"{ print XYZ }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one two".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["".to_string()]);
    }

    #[test]
    fn eval_print_use_number_of_fields_in_field_expression() {
        let lexer = Lexer::new(r#"{ print $NF }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one two three".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["three".to_string()]);
    }

    #[test]
    fn eval_print_line_numbers() {
        let lexer = Lexer::new(r#"{ print NR, $0 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one".to_string(), "two".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["1 one".to_string(), "2 two".to_string()]);
    }
}
