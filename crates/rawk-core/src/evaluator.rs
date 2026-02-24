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
    field_separator: String,
}

impl<'a> Evaluator<'a> {
    pub fn new(program: Program<'a>, input_lines: Vec<String>) -> Self {
        Self {
            program,
            input_lines,
            current_line_number: Cell::new(0),
            current_line: None,
            field_separator: " ".to_string(),
        }
    }

    pub fn eval(&mut self) -> Vec<String> {
        let mut output_lines: Vec<String> = Vec::new();

        let begin_rules: Vec<Rule<'a>> = self.program.begin_blocks_iter().cloned().collect();
        for rule in begin_rules.iter() {
            output_lines.extend(self.eval_begin_rule(rule));
        }

        let rules: Vec<Rule<'a>> = self.program.rules_iter().cloned().collect();
        for rule in rules.iter() {
            output_lines.extend(self.eval_rule(rule));
        }

        self.current_line_number.set(self.input_lines.len());
        self.current_line = None;

        let end_rules: Vec<Rule<'a>> = self.program.end_blocks_iter().cloned().collect();
        for rule in end_rules.iter() {
            output_lines.extend(self.eval_end_rule(rule));
        }

        output_lines
    }

    fn eval_rule(&mut self, rule: &Rule) -> Vec<String> {
        let mut output_lines = Vec::new();

        let input_lines = self.input_lines.clone();
        for (i, input_line) in input_lines.iter().enumerate() {
            self.current_line_number.set(i + 1);
            self.current_line = Some(input_line.clone());

            if let Rule::Action(action) = rule {
                output_lines.extend(self.eval_action(action, Some(input_line)));
            }
        }

        self.current_line = None;
        output_lines
    }

    fn eval_begin_rule(&mut self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::Begin(action) => self.eval_action(action, None),
            _ => Vec::new(),
        }
    }

    fn eval_end_rule(&mut self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::End(action) => self.eval_action(action, None),
            _ => Vec::new(),
        }
    }

    fn eval_action(&mut self, action: &Action, input_line: Option<&str>) -> Vec<String> {
        let mut output = Vec::new();

        for statement in &action.statements {
            if let Some(line) = self.eval_statement(statement, input_line) {
                output.push(line);
            }
        }

        output
    }

    fn eval_statement(&mut self, statement: &Statement<'_>, input_line: Option<&str>) -> Option<String> {
        match statement {
            Statement::Print(expressions) => Some(self.eval_print(expressions, input_line)),
            Statement::Printf(expressions) => Some(self.eval_printf(expressions)),
            Statement::Assignment { identifier, value } => {
                self.eval_assignment(identifier, value);
                None
            }
        }
    }

    fn eval_print(&self, expressions: &[Expression<'_>], input_line: Option<&str>) -> String {
        if expressions.is_empty() {
            return input_line.unwrap_or("").to_string();
        }

        let parts = expressions
            .iter()
            .map(|expr| self.eval_expression(expr))
            .collect::<Vec<String>>();
        parts.join("")
    }

    fn eval_printf(&self, expressions: &[Expression<'_>]) -> String {
        if expressions.is_empty() {
            return String::new();
        }

        let format = self.eval_expression(&expressions[0]);
        let format = unescape_awk_string(&format);
        let args: Vec<String> = expressions
            .iter()
            .skip(1)
            .map(|expr| self.eval_expression(expr))
            .collect();

        let rendered = format_printf(&format, &args);
        rendered.trim_end_matches(['\r', '\n']).to_string()
    }

    fn eval_assignment(&mut self, identifier: &str, value: &Expression<'_>) {
        if identifier == "FS" {
            self.field_separator = unescape_awk_string(&self.eval_expression(value));
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

                let field_count = self.split_fields(line).len();
                field_count.to_string()
            }
            "NR" => match self.current_line.as_ref() {
                Some(_) => self.current_line_number.get().to_string(),
                None => self.current_line_number.get().to_string(),
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

        self.split_fields(line)
            .into_iter()
            .nth((index - 1) as usize)
            .unwrap_or_default()
    }

    fn split_fields(&self, line: &str) -> Vec<String> {
        if self.field_separator == " " {
            line.split_whitespace().map(str::to_string).collect()
        } else {
            line.split(&self.field_separator)
                .map(str::to_string)
                .collect()
        }
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

fn unescape_awk_string(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => output.push('\n'),
            Some('t') => output.push('\t'),
            Some('r') => output.push('\r'),
            Some('\\') => output.push('\\'),
            Some('"') => output.push('"'),
            Some(other) => {
                output.push('\\');
                output.push(other);
            }
            None => output.push('\\'),
        }
    }

    output
}

fn format_printf(format: &str, args: &[String]) -> String {
    let mut result = String::new();
    let mut chars = format.chars().peekable();
    let mut arg_index = 0usize;

    while let Some(ch) = chars.next() {
        if ch != '%' {
            result.push(ch);
            continue;
        }

        if chars.peek() == Some(&'%') {
            chars.next();
            result.push('%');
            continue;
        }

        let mut left_justify = false;
        if chars.peek() == Some(&'-') {
            left_justify = true;
            chars.next();
        }

        let mut width: usize = 0;
        while let Some(next) = chars.peek() {
            if next.is_ascii_digit() {
                width = (width * 10) + (*next as usize - '0' as usize);
                chars.next();
            } else {
                break;
            }
        }

        let specifier = match chars.next() {
            Some(value) => value,
            None => {
                result.push('%');
                break;
            }
        };

        let arg = args.get(arg_index).cloned().unwrap_or_default();
        arg_index += 1;

        let formatted = match specifier {
            's' => arg,
            'd' => arg
                .parse::<f64>()
                .map(|value| value.trunc() as i64)
                .unwrap_or(0)
                .to_string(),
            _ => {
                result.push('%');
                if left_justify {
                    result.push('-');
                }
                if width > 0 {
                    result.push_str(&width.to_string());
                }
                result.push(specifier);
                continue;
            }
        };

        if width <= formatted.len() {
            result.push_str(&formatted);
            continue;
        }

        let padding_len = width - formatted.len();
        let padding = " ".repeat(padding_len);
        if left_justify {
            result.push_str(&formatted);
            result.push_str(&padding);
        } else {
            result.push_str(&padding);
            result.push_str(&formatted);
        }
    }

    result
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

    #[test]
    fn eval_printf_with_width_and_alignment() {
        let lexer = Lexer::new(r#"{ printf "[%10s] [%-16d]\n", $1, $3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["USSR 8649 275 Asia".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["[      USSR] [275             ]".to_string()]);
    }
}
