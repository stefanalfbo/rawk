use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Program<'a> {
    begin_blocks: Vec<Item<'a>>,
    items: Vec<Item<'a>>,
    end_blocks: Vec<Item<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Program {
            begin_blocks: vec![],
            items: vec![],
            end_blocks: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.items.len() + self.begin_blocks.len() + self.end_blocks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add_begin_block(&mut self, item: Item<'a>) {
        self.begin_blocks.push(item);
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        self.items.push(item);
    }

    pub fn begin_blocks_iter(&self) -> std::slice::Iter<'_, Item<'a>> {
        self.begin_blocks.iter()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Item<'a>> {
        self.items.iter()
    }
}

impl<'a> Default for Program<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> fmt::Display for Program<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.begin_blocks {
            write!(f, "{item}")?;
        }

        // Add space between begin blocks and main items if both exist
        if !self.begin_blocks.is_empty() && !self.items.is_empty() {
            write!(f, " ")?;
        }

        for item in &self.items {
            write!(f, "{item}")?;
        }

        // Add space between main items and end blocks if both exist
        if !self.items.is_empty() && !self.end_blocks.is_empty() {
            write!(f, " ")?;
        }

        for item in &self.end_blocks {
            write!(f, "{item}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Print(Vec<Expression<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Action<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a> {
    Begin(Action<'a>),
    Action(Action<'a>),
    PatternAction {
        pattern: Option<Expression<'a>>,
        action: Option<Action<'a>>,
    },
    End(Action<'a>),
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Begin(action) => write!(f, "BEGIN {}", action),
            Item::Action(action) => write!(f, "{}", action),
            Item::PatternAction { pattern, action } => match (pattern, action) {
                (Some(expr), Some(action)) => write!(f, "{} {}", expr, action),
                (Some(expr), None) => write!(f, "{}", expr),
                (None, Some(action)) => write!(f, "{}", action),
                (None, None) => write!(f, ""),
            },
            Item::End(action) => write!(f, "END {}", action),
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
                            .map(|expr| expr.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Number(f64),
    String(&'a str),
    Field(Box<Expression<'a>>),
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
            Expression::Field(expr) => write!(f, "${}", expr),
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

        let item = Item::Action(Action {
            statements: vec![Statement::Print(vec![])],
        });
        program.add_begin_block(item);

        assert_eq!(program.begin_blocks.len(), 1);
    }

    #[test]
    fn test_add_item_to_program() {
        let mut program = Program::new();

        let item = Item::Action(Action {
            statements: vec![Statement::Print(vec![])],
        });
        program.add_item(item);

        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_program_creation() {
        let expected_string = "$3 > 5";
        let program = Program {
            begin_blocks: vec![],
            items: vec![Item::PatternAction {
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
            begin_blocks: vec![Item::Begin(Action {
                statements: vec![Statement::Print(vec![])],
            })],
            items: vec![],
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
            items: vec![],
            end_blocks: vec![Item::End(Action {
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
            items: vec![Item::PatternAction {
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
        let expected_string = "BEGIN { print } $1 == 42 { print $2 } END { print \"hello\" }";
        let program = Program {
            begin_blocks: vec![Item::Begin(Action {
                statements: vec![Statement::Print(vec![])],
            })],
            items: vec![Item::PatternAction {
                pattern: Some(Expression::Infix {
                    left: Box::new(Expression::Field(Box::new(Expression::Number(1.0)))),
                    operator: Token::new(TokenKind::Equal, "==", 7),
                    right: Box::new(Expression::Number(42.0)),
                }),
                action: Some(Action {
                    statements: vec![Statement::Print(vec![Expression::Field(Box::new(
                        Expression::Number(2.0),
                    ))])],
                }),
            }],
            end_blocks: vec![Item::End(Action {
                statements: vec![Statement::Print(vec![Expression::String("hello".into())])],
            })],
        };

        assert_eq!(expected_string, program.to_string());
    }
}
