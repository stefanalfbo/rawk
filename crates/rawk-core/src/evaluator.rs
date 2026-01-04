use crate::{Action, Expression, Item, Program, ast::Statement};

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

        for item in self.program.iter() {
            output_lines.extend(self.eval_item(item));
        }

        output_lines
    }

    fn eval_item(&self, item: &Item) -> Vec<String> {
        let mut output_lines = Vec::new();

        for input_line in &self.input_lines {
            match item {
                Item::PatternAction { pattern, action } => {
                    output_lines.push(eval_pattern_action(pattern, action, input_line));
                }
                _ => {}
            }
        }

        output_lines
    }
}

fn eval_pattern_action(
    _pattern: &Option<Expression>,
    action: &Option<Action>,
    input_line: &String,
) -> String {
    if let Some(action) = action
        && action.statements.len() == 1
    {
        let statement = &action.statements[0];

        match statement {
            Statement::Print(expressions) => {
                if expressions.is_empty() {
                    return format!("{}", input_line);
                }

                return format!("not implemented");
            }
        }
    } else {
        return format!("not implemented");
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
}
