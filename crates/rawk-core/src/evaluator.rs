use crate::{
    Action, Program, Rule,
    ast::{Expression, Statement},
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

        for rule in self.program.iter() {
            output_lines.extend(self.eval_rule(rule));
        }

        output_lines
    }

    fn eval_rule(&self, rule: &Rule) -> Vec<String> {
        let mut output_lines = Vec::new();

        for input_line in &self.input_lines {
            match rule {
                Rule::Action(action) => {
                    output_lines.push(eval_action(action, Some(input_line)));
                }
                _ => {}
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
                return parts.join(" ");
            }
        }
    } else {
        return format!("not implemented");
    }
}

fn eval_expression(expression: &Expression, _input_line: Option<&str>) -> String {
    match expression {
        Expression::String(value) => value.to_string(),
        Expression::Number(value) => value.to_string(),
        Expression::Field(_inner) => "not implemented".to_string(),
        Expression::Infix { .. } => "not implemented".to_string(),
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
}
