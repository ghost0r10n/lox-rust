use super::expression::Expression;

pub trait Visitor<R> {
    fn visit(&self, expression: &Expression) -> R;
}
