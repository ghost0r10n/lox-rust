use crate::scanner::{
    token::Token,
    token_type::{self, TokenType},
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
    fn expression(&self) -> Expression {
        self.equality()
    }

    fn equality(&self) -> Expression {
        let mut expression: Expression = self.comparison();
        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expression = self.comparison();
            expression = Expression::Binary {
                left: expression,
                operator,
                right,
            }
        }
        return expression;
    }
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
        return self.tokens[self.current];
    }
    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1];
    }
}
