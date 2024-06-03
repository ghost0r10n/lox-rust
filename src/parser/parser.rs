use super::{expression::Expression, literal_value::LiteralValue, visitor::Visitor};

pub struct ASTprinter {}

impl Visitor<String> for ASTprinter {
    fn visit_binary_expr(
        &self,
        left: &super::expression::Expression,
        operator: &crate::scanner::token::Token,
        right: &super::expression::Expression,
    ) -> String {
        self.parenthesize(operator.lexame.to_owned(), &[left, right])
    }

    fn visit_grouping_expr(&self, expression: &super::expression::Expression) -> String {
        self.parenthesize("group".to_string(), &[expression])
    }

    fn visit_unary_expr(
        &self,
        operator: &crate::scanner::token::Token,
        right: &super::expression::Expression,
    ) -> String {
        self.parenthesize(operator.lexame.to_owned(), &[right])
    }

    fn visit_literal_expr(&self, value: &super::literal_value::LiteralValue) -> String {
        match value {
            LiteralValue::String(v) => return v.to_string(),
            LiteralValue::Float(v) => return v.to_string(),
            LiteralValue::None => return "nil".to_string(),
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

    pub fn parenthesize(&self, name: String, expressions: &[&Expression]) -> String {
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
