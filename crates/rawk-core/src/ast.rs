use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Program<'a> {
    begin_blocks: Vec<Rule<'a>>,
    rules: Vec<Rule<'a>>,
    end_blocks: Vec<Rule<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Program {
            begin_blocks: vec![],
            rules: vec![],
            end_blocks: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.rules.len() + self.begin_blocks.len() + self.end_blocks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn add_begin_block(&mut self, rule: Rule<'a>) {
        self.begin_blocks.push(rule);
    }

    pub fn add_end_block(&mut self, rule: Rule<'a>) {
        self.end_blocks.push(rule);
    }

    pub fn add_rule(&mut self, rule: Rule<'a>) {
        self.rules.push(rule);
    }

    pub fn begin_blocks_iter(&self) -> std::slice::Iter<'_, Rule<'a>> {
        self.begin_blocks.iter()
    }

    pub fn end_blocks_iter(&self) -> std::slice::Iter<'_, Rule<'a>> {
        self.end_blocks.iter()
    }

    pub fn rules_iter(&self) -> std::slice::Iter<'_, Rule<'a>> {
        self.rules.iter()
    }
}

impl<'a> Default for Program<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> fmt::Display for Program<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rule in &self.begin_blocks {
            write!(f, "{rule}")?;
        }

        // Add space between begin blocks and rules if both exist
        if !self.begin_blocks.is_empty() && !self.rules.is_empty() {
            write!(f, " ")?;
        }

        for rule in &self.rules {
            write!(f, "{rule}")?;
        }

        // Add space between rules and end blocks if both exist
        if !self.rules.is_empty() && !self.end_blocks.is_empty() {
            write!(f, " ")?;
        }

        for rule in &self.end_blocks {
            write!(f, "{rule}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Print(Vec<Expression<'a>>),
    Printf(Vec<Expression<'a>>),
    Assignment {
        identifier: &'a str,
        value: Expression<'a>,
    },
    AddAssignment {
        identifier: &'a str,
        value: Expression<'a>,
    },
    PreIncrement {
        identifier: &'a str,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Action<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rule<'a> {
    Begin(Action<'a>),
    Action(Action<'a>),
    PatternAction {
        pattern: Option<Expression<'a>>,
        action: Option<Action<'a>>,
    },
    End(Action<'a>),
}

impl<'a> fmt::Display for Rule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Begin(action) => write!(f, "BEGIN {}", action),
            Rule::Action(action) => write!(f, "{}", action),
            Rule::PatternAction { pattern, action } => match (pattern, action) {
                (Some(expr), Some(action)) => write!(f, "{} {}", expr, action),
                (Some(expr), None) => write!(f, "{}", expr),
                (None, Some(action)) => write!(f, "{}", action),
                (None, None) => write!(f, ""),
            },
            Rule::End(action) => write!(f, "END {}", action),
        }
    }
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.statements.is_empty() {
            write!(f, "{{}}")
        } else {
            write!(
                f,
                "{{ {} }}",
                self.statements
                    .iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<Vec<String>>()
                    .join("; ")
            )
        }
    }
}

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Print(expressions) => {
                if expressions.is_empty() {
                    write!(f, "print")
                } else {
                    write!(
                        f,
                        "print {}",
                        expressions
                            .iter()
                            .filter(|expr| *expr != &Expression::String(" "))
                            .map(|expr| expr.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
            Statement::Printf(expressions) => {
                if expressions.is_empty() {
                    write!(f, "printf")
                } else {
                    write!(
                        f,
                        "printf {}",
                        expressions
                            .iter()
                            .map(|expr| expr.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
            Statement::Assignment { identifier, value } => write!(f, "{identifier} = {value}"),
            Statement::AddAssignment { identifier, value } => {
                write!(f, "{identifier} += {value}")
            }
            Statement::PreIncrement { identifier } => write!(f, "++{identifier}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Number(f64),
    String(&'a str),
    Regex(&'a str),
    Field(Box<Expression<'a>>),
    Identifier(&'a str),
    // non_unary_expr
    Infix {
        left: Box<Expression<'a>>,
        operator: Token<'a>,
        right: Box<Expression<'a>>,
    },
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::String(value) => write!(f, "\"{}\"", value),
            Expression::Regex(value) => write!(f, "/{}/", value),
            Expression::Field(expr) => write!(f, "${}", expr),
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                write!(f, "{} {} {}", left, operator.literal, right)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    #[test]
    fn test_empty_program_creation() {
        let program = Program::default();

        assert!(program.is_empty());
    }

    #[test]
    fn test_add_block_to_program() {
        let mut program = Program::new();

        let rule = Rule::Action(Action {
            statements: vec![Statement::Print(vec![])],
        });
        program.add_begin_block(rule);

        assert_eq!(program.begin_blocks.len(), 1);
    }

    #[test]
    fn test_add_rule_to_program() {
        let mut program = Program::new();

        let rule = Rule::Action(Action {
            statements: vec![Statement::Print(vec![])],
        });
        program.add_rule(rule);

        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_program_creation() {
        let expected_string = "$3 > 5";
        let program = Program {
            begin_blocks: vec![],
            rules: vec![Rule::PatternAction {
                pattern: Some(Expression::Infix {
                    left: Box::new(Expression::Field(Box::new(Expression::Number(3.0)))),
                    operator: Token::new(TokenKind::GreaterThan, ">", 3),
                    right: Box::new(Expression::Number(5.0)),
                }),
                action: None,
            }],
            end_blocks: vec![],
        };

        assert_eq!(expected_string, program.to_string());
    }

    #[test]
    fn test_begin_block_program_creation() {
        let expected_string = "BEGIN { print }";
        let program = Program {
            begin_blocks: vec![Rule::Begin(Action {
                statements: vec![Statement::Print(vec![])],
            })],
            rules: vec![],
            end_blocks: vec![],
        };

        assert!(program.len() == 1);
        assert_eq!(expected_string, program.to_string());
    }

    #[test]
    fn test_end_block_program_creation() {
        let expected_string = "END { print }";
        let program = Program {
            begin_blocks: vec![],
            rules: vec![],
            end_blocks: vec![Rule::End(Action {
                statements: vec![Statement::Print(vec![])],
            })],
        };

        assert!(program.len() == 1);
        assert_eq!(expected_string, program.to_string());
    }

    #[test]
    fn test_action_without_pattern_program_creation() {
        let expected_string = "{ print }";
        let program = Program {
            begin_blocks: vec![],
            rules: vec![Rule::PatternAction {
                pattern: None,
                action: Some(Action {
                    statements: vec![Statement::Print(vec![])],
                }),
            }],
            end_blocks: vec![],
        };

        assert!(program.len() == 1);
        assert_eq!(expected_string, program.to_string());
    }

    #[test]
    fn test_program_with_begin_body_and_end_blocks() {
        let expected_string =
            "BEGIN { print } $1 == 42 { print NF, $2, $3 } END { print \"hello\" }";
        let program = Program {
            begin_blocks: vec![Rule::Begin(Action {
                statements: vec![Statement::Print(vec![])],
            })],
            rules: vec![Rule::PatternAction {
                pattern: Some(Expression::Infix {
                    left: Box::new(Expression::Field(Box::new(Expression::Number(1.0)))),
                    operator: Token::new(TokenKind::Equal, "==", 7),
                    right: Box::new(Expression::Number(42.0)),
                }),
                action: Some(Action {
                    statements: vec![Statement::Print(vec![
                        Expression::Identifier("NF"),
                        Expression::String(" "),
                        Expression::Field(Box::new(Expression::Number(2.0))),
                        Expression::String(" "),
                        Expression::Field(Box::new(Expression::Number(3.0))),
                    ])],
                }),
            }],
            end_blocks: vec![Rule::End(Action {
                statements: vec![Statement::Print(vec![Expression::String("hello".into())])],
            })],
        };

        assert_eq!(expected_string, program.to_string());
    }

    #[test]
    fn test_print_regex_expression() {
        let expr = Expression::Regex("^[a-z]+$");

        assert_eq!("/^[a-z]+$/", expr.to_string());
    }

    #[test]
    fn test_assignment_statement_display() {
        let statement = Statement::Assignment {
            identifier: "pop",
            value: Expression::Infix {
                left: Box::new(Expression::Identifier("pop")),
                operator: Token::new(TokenKind::Plus, "+", 0),
                right: Box::new(Expression::Field(Box::new(Expression::Number(3.0)))),
            },
        };

        assert_eq!("pop = pop + $3", statement.to_string());
    }

    #[test]
    fn test_add_assignment_statement_display() {
        let statement = Statement::AddAssignment {
            identifier: "pop",
            value: Expression::Field(Box::new(Expression::Number(3.0))),
        };

        assert_eq!("pop += $3", statement.to_string());
    }

    #[test]
    fn test_pre_increment_statement_display() {
        let statement = Statement::PreIncrement { identifier: "n" };

        assert_eq!("++n", statement.to_string());
    }

    #[test]
    fn test_action_with_new_statements_display() {
        let action = Action {
            statements: vec![
                Statement::AddAssignment {
                    identifier: "pop",
                    value: Expression::Field(Box::new(Expression::Number(3.0))),
                },
                Statement::PreIncrement { identifier: "n" },
            ],
        };

        assert_eq!("{ pop += $3; ++n }", action.to_string());
    }
}
