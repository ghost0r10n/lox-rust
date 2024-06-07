use super::{expression::Expression, visitor::Visitor};
use crate::utils::literal_value::LiteralValue;

pub struct ASTprinter {}

impl Visitor<String> for ASTprinter {
    fn visit(&self, expression: &Expression) -> String {
        match expression {
            Expression::Unary { operator, right } => {
                self.parenthesize(operator.lexame.to_owned(), &[right])
            }
            Expression::Literal { value } => match value {
                LiteralValue::String(v) => return v.to_string(),
                LiteralValue::Float(v) => return v.to_string(),
                LiteralValue::Boolean(v) => return v.to_string(),
                LiteralValue::Nil => return "nil".to_string(),
                LiteralValue::None => return "".to_string(),
            },
            Expression::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(operator.lexame.to_owned(), &[left, right]),
            Expression::Grouping { expression } => {
                self.parenthesize("group".to_string(), &[expression])
            }
        }
    }
}
impl ASTprinter {
    pub fn new() -> Self {
        ASTprinter {}
    }

    pub fn print_tree(&self, expression: Expression) -> String {
        return expression.accept(self);
    }

    fn parenthesize(&self, name: String, expressions: &[&Expression]) -> String {
        let mut ast_string: String = String::new();
        ast_string.push_str("(");
        ast_string.push_str(&name);
        for expression in expressions {
            ast_string.push_str(" ");
            ast_string.push_str(expression.accept(self).as_str());
        }
        ast_string.push_str(")");
        return ast_string;
    }
}
