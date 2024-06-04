use super::visitor::Visitor;
use crate::scanner::token::Token;
use crate::utils::literal_value::LiteralValue;

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

impl Expression {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expression::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expression::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expression::Literal { value } => visitor.visit_literal_expr(value),
        }
    }
}
