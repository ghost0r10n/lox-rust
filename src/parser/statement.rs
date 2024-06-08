use std::fmt;

use crate::scanner::token::Token;

use super::{expression::Expression, visitor::VisitorStatement};

#[derive(Clone)]
pub enum Statement {
    Expr {
        expression: Expression,
    },
    Print {
        expression: Expression,
    },
    Var {
        name: Token,
        initializer: Expression,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expr { expression } => write!(f, "{}", expression),
            Statement::Print { expression } => write!(f, "{}", expression),
            Statement::Var { name, initializer } => write!(f, "{} {}", name, initializer),
        }
    }
}

impl Statement {
    pub fn accept<R>(&self, visitor: &mut dyn VisitorStatement<R>) -> R {
        match self {
            Statement::Expr { expression } => visitor.visit(&Statement::Expr {
                expression: expression.clone(),
            }),
            Statement::Print { expression } => visitor.visit(&Statement::Print {
                expression: expression.clone(),
            }),
            Statement::Var { name, initializer } => visitor.visit(&Statement::Var {
                name: name.clone(),
                initializer: initializer.clone(),
            }),
        }
    }
}
