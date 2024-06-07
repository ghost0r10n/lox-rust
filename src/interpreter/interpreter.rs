use crate::{
    lox_runtime_error,
    parser::{expression::Expression, visitor::Visitor},
    scanner::token_type::TokenType,
    utils::literal_value::LiteralValue,
};

pub struct Interpreter;

impl Visitor<LiteralValue> for Interpreter {
    fn visit(&self, expression: &Expression) -> LiteralValue {
        match expression {
            Expression::Unary { operator, right } => {
                println!("UNARY");
                let right_evaluated: LiteralValue = self.evaluate(right.clone());

                match operator.token_type {
                    TokenType::Bang => LiteralValue::Boolean(!self.is_truthy(right_evaluated)),
                    TokenType::Minus => match right_evaluated {
                        LiteralValue::Float(value) => LiteralValue::Float(-value),
                        LiteralValue::Boolean(_) => lox_runtime_error(
                            operator.clone(),
                            "Tried to negate boolean".to_string(),
                        ),
                        LiteralValue::String(_) => lox_runtime_error(
                            operator.clone(),
                            "Tried to negate string".to_string(),
                        ),
                        LiteralValue::Nil => {
                            lox_runtime_error(operator.clone(), "Tried to negate nil".to_string())
                        }
                        LiteralValue::None => lox_runtime_error(
                            operator.clone(),
                            "None value cannot be used, that is really strange".to_string(),
                        ),
                    },
                    _ => LiteralValue::Nil,
                }
            }
            Expression::Literal { value } => value.clone(),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_evaluated = self.evaluate(left.clone());
                let right_evaluated = self.evaluate(right.clone());
                match operator.token_type {
                    //ARITMETICHS
                    TokenType::Minus => {
                        self.evaluate_arithmetic_op(left_evaluated, right_evaluated, |lv, rv| {
                            lv - rv
                        })
                    }
                    TokenType::Star => {
                        self.evaluate_arithmetic_op(left_evaluated, right_evaluated, |lv, rv| {
                            lv * rv
                        })
                    }
                    TokenType::Slash => {
                        self.evaluate_arithmetic_op(left_evaluated, right_evaluated, |lv, rv| {
                            lv / rv
                        })
                    }
                    TokenType::Plus => match (left_evaluated, right_evaluated) {
                        (LiteralValue::String(lv), LiteralValue::String(rv)) => {
                            LiteralValue::String(format!("{}{}", lv, rv))
                        }
                        (LiteralValue::Float(lv), LiteralValue::Float(rv)) => {
                            LiteralValue::Float(lv + rv)
                        }
                        _ => lox_runtime_error(
                            operator.clone(),
                            "Addition can be done between string and numbers".to_string(),
                        ),
                    },

                    //BOOLEAN OPERATORS
                    TokenType::LessEqual => {
                        self.evaluate_boolean_op(left_evaluated, right_evaluated, |lv, rv| lv <= rv)
                    }
                    TokenType::Less => {
                        self.evaluate_boolean_op(left_evaluated, right_evaluated, |lv, rv| lv < rv)
                    }
                    TokenType::Greater => {
                        self.evaluate_boolean_op(left_evaluated, right_evaluated, |lv, rv| lv > rv)
                    }
                    TokenType::GreaterEqual => {
                        self.evaluate_boolean_op(left_evaluated, right_evaluated, |lv, rv| lv >= rv)
                    }
                    TokenType::BangEqual => {
                        LiteralValue::Boolean(!self.is_equal(left_evaluated, right_evaluated))
                    }
                    TokenType::EqualEqual => {
                        LiteralValue::Boolean(self.is_equal(left_evaluated, right_evaluated))
                    }
                    _ => todo!(),
                }
            }
            Expression::Grouping { expression } => self.evaluate(Box::new(Expression::Grouping {
                expression: expression.clone(),
            })),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpet(&self, expression: Expression) {
        let result: LiteralValue = self.evaluate(Box::new(expression));
        println!("{}", result);
    }

    fn evaluate(&self, expression: Box<Expression>) -> LiteralValue {
        return expression.accept(self);
    }

    //TODO Refactor this code using a trait maybe
    fn evaluate_arithmetic_op(
        &self,
        left: LiteralValue,
        right: LiteralValue,
        op: fn(f64, f64) -> f64,
    ) -> LiteralValue {
        match (left, right) {
            (LiteralValue::Float(lv), LiteralValue::Float(rv)) => LiteralValue::Float(op(lv, rv)),
            _ => todo!(),
        }
    }
    fn evaluate_boolean_op(
        &self,
        left: LiteralValue,
        right: LiteralValue,
        op: fn(f64, f64) -> bool,
    ) -> LiteralValue {
        match (left, right) {
            (LiteralValue::Float(lv), LiteralValue::Float(rv)) => LiteralValue::Boolean(op(lv, rv)),
            _ => todo!(),
        }
    }
    fn is_truthy(&self, value: LiteralValue) -> bool {
        match value {
            LiteralValue::Boolean(value) => value,
            LiteralValue::None => false,
            _ => true,
        }
    }

    fn is_equal(&self, lv: LiteralValue, rv: LiteralValue) -> bool {
        if lv == LiteralValue::None && rv == LiteralValue::None {
            return true;
        }
        if lv == LiteralValue::None {
            return false;
        }
        return lv == rv;
    }
}
