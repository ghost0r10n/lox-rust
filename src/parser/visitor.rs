use crate::scanner::token::Token;

use super::{expression::Expression, literal_value::LiteralValue};

pub trait Visitor<R> {
    fn visit_binary_expr(&self, left: &Expression, operator: &Token, right: &Expression) -> R;
    fn visit_grouping_expr(&self, expression: &Expression) -> R;
    fn visit_unary_expr(&self, operator: &Token, right: &Expression) -> R;
    fn visit_literal_expr(&self, value: &LiteralValue) -> R;
}
