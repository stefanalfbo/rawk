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
pub struct Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a> {
    Action,
    PatternAction {
        pattern: Option<Expression<'a>>,
        action: Option<Action>,
    },
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Action => write!(f, "<action>"),
            Item::PatternAction { pattern, action: _ } => {
                if let Some(expr) = pattern {
                    write!(f, "{}", expr)
                } else {
                    write!(f, "<no pattern>")
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
    fn test_program_creation() {
        let expected_string = "$3 > 5";
        let program: Program<'static> = Program {
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
}
