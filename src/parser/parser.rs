use core::panic;

use crate::{
    lox_parser_error, loxerror,
    scanner::{token::Token, token_type::TokenType},
    utils::literal_value::LiteralValue,
};

use super::expression::Expression;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/* GRAMMAR
 *  expression     → equality ;
 *  equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 *  comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 *  term           → factor ( ( "-" | "+" ) factor )* ;
 *  factor         → unary ( ( "/" | "*" ) unary )* ;
 *  unary          → ( "!" | "-" ) unary | primary ;
 *  primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
*/

impl Parser {
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expression: Expression = self.comparison();
        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expression = self.comparison();
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }
        return expression;
    }

    fn comparison(&mut self) -> Expression {
        let mut expression: Expression = self.term();
        while self.match_token_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous();
            let right: Expression = self.term();
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }
        return expression;
    }

    fn term(&mut self) -> Expression {
        let mut expression: Expression = self.factor();
        while self.match_token_type(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expression = self.factor();
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }
        }
        return expression;
    }

    fn factor(&mut self) -> Expression {
        let mut expression: Expression = self.unary();
        while self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expression = self.unary();
            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }
        return expression;
    }

    fn unary(&mut self) -> Expression {
        if self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expression = self.unary();
            return Expression::Unary {
                operator,
                right: Box::new(right),
            };
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expression {
        if self.match_token_type(&[TokenType::False]) {
            return Expression::Literal {
                value: LiteralValue::Boolean(false),
            };
        }
        if self.match_token_type(&[TokenType::True]) {
            return Expression::Literal {
                value: LiteralValue::Boolean(true),
            };
        }
        if self.match_token_type(&[TokenType::Nil]) {
            return Expression::Literal {
                value: LiteralValue::Nil,
            };
        }
        if self.match_token_type(&[TokenType::Number, TokenType::String]) {
            return Expression::Literal {
                value: match self.previous().literal {
                    LiteralValue::String(v) => LiteralValue::String(v),
                    LiteralValue::Float(v) => LiteralValue::Float(v),
                    _ => LiteralValue::None,
                },
            };
        }
        if self.match_token_type(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            );
            return Expression::Grouping {
                expression: Box::new(expr),
            };
        }
        return Expression::Literal {
            value: LiteralValue::Nil,
        };
    }

    /*
     *+-------+
     *| Utils |
     *+-------+
     */

    fn match_token_type(&mut self, types: &[TokenType]) -> bool {
        for _type in types {
            if self.check(_type.to_owned()) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&mut self, _type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == _type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&mut self) -> bool {
        return match self.peek().token_type {
            TokenType::Eof => true,
            _ => false,
        };
    }

    fn peek(&mut self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        if self.check(token_type) {
            self.advance();
        }
        lox_parser_error(self.peek(), message);
        panic!("Error while parsing")
    }
}
