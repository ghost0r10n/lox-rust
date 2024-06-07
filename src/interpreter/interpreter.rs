use crate::{
    parser::{expression::Expression, visitor::Visitor},
    scanner::token_type::TokenType,
    utils::literal_value::LiteralValue,
};

pub struct Interpreter;

impl Visitor<LiteralValue> for Interpreter {
    fn visit(&self, expression: &Expression) -> LiteralValue {
        match expression {
            Expression::Unary { operator, right } => {
                let right_evaluated: LiteralValue = self.evaluate(right.clone());

                match operator.token_type {
                    TokenType::Minus => right_evaluated,
                }
            }
            Expression::Literal { value } => value,
            Expression::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(operator.lexame.to_owned(), &[left, right]),
            Expression::Grouping { expression } => self.evaluate(Expression::Grouping {
                expression: expression.clone(),
            }),
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expression: Box<Expression>) -> LiteralValue {
        return expression.accept(self);
    }
}
