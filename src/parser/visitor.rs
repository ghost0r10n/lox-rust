use super::{expression::Expression, statement::Statement};

pub trait VisitorExpression<R> {
    fn visit(&mut self, expression: &Expression) -> R;
}

pub trait VisitorStatement<R> {
    fn visit(&mut self, expression: &Statement) -> R;
}
