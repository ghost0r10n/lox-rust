use super::{literal_value::LiteralValue, visitor::Visitor};

pub struct ExpressionVisitor;

impl Visitor<()> for ExpressionVisitor {
    fn visit_binary_expr(
        &self,
        left: &super::expression::Expression,
        operator: &crate::scanner::token::Token,
        right: &super::expression::Expression,
    ) {
        left.accept(self);
        right.accept(self);
    }

    fn visit_grouping_expr(&self, expression: &super::expression::Expression) {
        expression.accept(self);
    }

    fn visit_unary_expr(
        &self,
        operator: &crate::scanner::token::Token,
        right: &super::expression::Expression,
    ) {
        right.accept(self);
    }

    fn visit_literal_expr(&self, value: &super::literal_value::LiteralValue) {
        match value {
            LiteralValue::Float(_v) => println!("float literal"),
            LiteralValue::String(_v) => println!("string literal"),
            LiteralValue::None => println!("nil"),
        }
    }
}
