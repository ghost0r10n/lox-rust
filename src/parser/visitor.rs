use super::expression::Expression;
use crate::utils::literal_value::LiteralValue;

pub trait Visitor<R> {
    fn visit(&self, expression: &Expression) -> R;
}
