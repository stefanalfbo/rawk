use crate::{
    Action, Program, Rule,
    ast::{Expression, Statement},
    token::TokenKind,
};

pub struct Evaluator<'a> {
    program: Program<'a>,
    input_lines: Vec<String>,
}

impl<'a> Evaluator<'a> {
    pub fn new(program: Program<'a>, input_lines: Vec<String>) -> Self {
        Self {
            program,
            input_lines,
        }
    }

    pub fn eval(&mut self) -> Vec<String> {
        let mut output_lines: Vec<String> = Vec::new();

        for rule in self.program.begin_blocks_iter() {
            output_lines.extend(self.eval_begin_rule(rule));
        }

        for rule in self.program.rules_iter() {
            output_lines.extend(self.eval_rule(rule));
        }

        for rule in self.program.end_blocks_iter() {
            output_lines.extend(self.eval_end_rule(rule));
        }

        output_lines
    }

    fn eval_rule(&self, rule: &Rule) -> Vec<String> {
        let mut output_lines = Vec::new();

        for input_line in &self.input_lines {
            if let Rule::Action(action) = rule {
                output_lines.push(eval_action(action, Some(input_line)));
            }
        }

        output_lines
    }

    fn eval_begin_rule(&self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::Begin(action) => vec![eval_action(action, None)],
            _ => Vec::new(),
        }
    }

    fn eval_end_rule(&self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::End(action) => vec![eval_action(action, None)],
            _ => Vec::new(),
        }
    }
}

fn eval_action(action: &Action, input_line: Option<&str>) -> String {
    if action.statements.len() == 1 {
        let statement = &action.statements[0];

        match statement {
            Statement::Print(expressions) => {
                if expressions.is_empty() {
                    return input_line.unwrap_or("").to_string();
                }

                let parts = expressions
                    .iter()
                    .map(|expr| eval_expression(expr, input_line))
                    .collect::<Vec<String>>();
                parts.join("")
            }
        }
    } else {
        "not implemented".to_string()
    }
}

fn eval_expression(expression: &Expression, _input_line: Option<&str>) -> String {
    match expression {
        Expression::String(value) => value.to_string(),
        Expression::Number(value) => value.to_string(),
        Expression::Regex(value) => value.to_string(),
        Expression::Field(_inner) => "not implemented".to_string(),
        Expression::Infix {
            left,
            operator,
            right,
        } => eval_numeric_infix(left, operator, right)
            .map(|value| value.to_string())
            .unwrap_or_else(|| "not implemented".to_string()),
    }
}

fn eval_numeric_infix(
    left: &Expression<'_>,
    operator: &crate::token::Token<'_>,
    right: &Expression<'_>,
) -> Option<f64> {
    let left_value = eval_numeric_expression(left)?;
    let right_value = eval_numeric_expression(right)?;

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

fn eval_numeric_expression(expression: &Expression<'_>) -> Option<f64> {
    match expression {
        Expression::Number(value) => Some(*value),
        Expression::Infix {
            left,
            operator,
            right,
        } => eval_numeric_infix(left, operator, right),
        _ => None,
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
}
