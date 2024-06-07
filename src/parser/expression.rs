use std::fmt;

use super::visitor::Visitor;
use crate::scanner::token::Token;
use crate::utils::literal_value::LiteralValue;

#[derive(Clone)]
pub enum Expression {
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Literal {
        value: LiteralValue,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Unary { operator, right } => write!(f, "Unary ({} {})", operator, right),
            Expression::Literal { value } => write!(f, "Literal {}", value),
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(f, "Binary({} {} {})", left, operator, right),
            Expression::Grouping { expression } => write!(f, "(group {})", expression),
        }
    }
}

impl Expression {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expression::Unary { operator, right } => visitor.visit(&Expression::Unary {
                operator: operator.clone(),
                right: right.clone(),
            }),
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit(&Expression::Binary {
                left: left.clone(),
                operator: operator.clone(),
                right: right.clone(),
            }),
            Expression::Grouping { expression } => visitor.visit(&expression),
            Expression::Literal { value } => visitor.visit(&Expression::Literal {
                value: value.clone(),
            }),
        }
    }
}
