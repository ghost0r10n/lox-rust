use crate::scanner::token::Token;

use super::literal_value::LiteralValue;

pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
}

impl Expression {
    fn new_binary(left: Expression, operator: Token, right: Expression) -> Expression {
        Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    fn new_grouping(expression: Expression) -> Expression {
        Expression::Grouping {
            expression: Box::new(expression),
        }
    }
    fn new_literal(literal_value: LiteralValue) -> Expression {
        Expression::Literal {
            value: literal_value,
        }
    }
    fn new_unary(operator: Token, right: Expression) -> Expression {
        Expression::Unary {
            right: Box::new(right),
            operator,
        }
    }
}
