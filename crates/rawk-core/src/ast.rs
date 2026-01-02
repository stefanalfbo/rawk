use std::fmt;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Program<'a> {
    items: Vec<Item<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Program { items: vec![] }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn add_item(&mut self, item: Item<'a>) {
        self.items.push(item);
    }
}

impl<'a> fmt::Display for Program<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.items
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
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
    Action(Action<'a>),
    PatternAction {
        pattern: Option<Expression<'a>>,
        action: Option<Action<'a>>,
    },
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Action(action) => write!(f, "{}", action),
            Item::PatternAction { pattern, action } => match (pattern, action) {
                (Some(expr), Some(action)) => write!(f, "{} {}", expr, action),
                (Some(expr), None) => write!(f, "{}", expr),
                (None, Some(action)) => write!(f, "{}", action),
                (None, None) => write!(f, ""),
            },
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
        let program = Program::new();

        assert_eq!(program.len(), 0);
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
            items: vec![Item::PatternAction {
                pattern: Some(Expression::Infix {
                    left: Box::new(Expression::Field(Box::new(Expression::Number(3.0)))),
                    operator: Token {
                        kind: TokenKind::GreaterThan,
                        literal: ">",
                    },
                    right: Box::new(Expression::Number(5.0)),
                }),
                action: None,
            }],
        };

        assert_eq!(expected_string, program.to_string());
    }

    #[test]
    fn test_action_without_pattern_program_creation() {
        let expected_string = "{ print }";
        let program = Program {
            items: vec![Item::PatternAction {
                pattern: None,
                action: Some(Action {
                    statements: vec![Statement::Print(vec![])],
                }),
            }],
        };

        assert_eq!(expected_string, program.to_string());
    }
}
