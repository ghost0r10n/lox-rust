use crate::{
    lox_parser_error,
    scanner::{token::Token, token_type::TokenType},
    utils::literal_value::LiteralValue,
};

use super::{
    expression::{self, Expression},
    parse_error::ParsingError,
    statement::Statement,
};

pub struct Parser {
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
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self, is_ast: bool) -> Result<Vec<Statement>, ParsingError> {
        let mut statements: Vec<Statement> = Vec::new();
        while !self.is_at_end() {
            match self.statement() {
                Ok(statement) => {
                    if is_ast {
                        println!("{}", statement);
                    }
                    match self.declaration() {
                        Ok(declaration) => statements.push(declaration),
                        Err(_) => self.sync(),
                    }
                }
                Err(_) => self.sync(),
            }
        }
        return Ok(statements);
    }

    pub fn sync(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                ()
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::If
                | TokenType::Var
                | TokenType::For
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {
                    let _advance = self.advance();
                }
            }
        }
    }

    fn declaration(&mut self) -> Result<Statement, ParsingError> {
        match self.match_token_type(&[TokenType::Var]) {
            true => self.var_declaration(),
            false => match self.statement() {
                Ok(r) => Ok(r),
                Err(error) => Err(error),
            },
        }
    }

    fn var_declaration(&mut self) -> Result<Statement, ParsingError> {
        return match self.consume(TokenType::Identifier, "Expected identifier".to_string()) {
            Ok(name) => match self.match_token_type(&[TokenType::Equal]) {
                true => {
                    {
                        return match self.consume(
                            TokenType::Semicolon,
                            "Expected ';' after variable declaration".to_string(),
                        ) {
                            Ok(_) => Ok(Statement::Var {
                                name,
                                initializer: Expression::Literal {
                                    value: LiteralValue::Nil,
                                },
                            }),
                            Err(e) => Err(e),
                        };
                    };
                }
                false => {
                    return match self.consume(
                        TokenType::Semicolon,
                        "Expected ';' after variable declaration".to_string(),
                    ) {
                        Ok(_) => Ok(Statement::Var {
                            name,
                            initializer: Expression::Literal {
                                value: LiteralValue::Nil,
                            },
                        }),
                        Err(e) => Err(e),
                    };
                }
            },
            Err(error) => Err(error),
        };
    }
    fn statement(&mut self) -> Result<Statement, ParsingError> {
        match self.match_token_type(&[TokenType::Print]) {
            true => self.print_statement(),
            false => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Statement, ParsingError> {
        return match self.expression() {
            Ok(value) => {
                match self.consume(TokenType::Semicolon, "Expect ; after value".to_string()) {
                    Ok(_) => Ok(Statement::Print { expression: value }),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        };
    }

    fn expression_statement(&mut self) -> Result<Statement, ParsingError> {
        return match self.expression() {
            Ok(value) => {
                match self.consume(TokenType::Semicolon, "Expect ; after value".to_string()) {
                    Ok(_) => Ok(Statement::Expr { expression: value }),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        };
    }

    fn expression(&mut self) -> Result<Expression, ParsingError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ParsingError> {
        match self.comparison() {
            Ok(expression) => {
                let mut mutable_expression = expression.clone();
                while self.match_token_type(&[TokenType::EqualEqual, TokenType::BangEqual]) {
                    let operator: Token = self.previous();
                    match self.comparison() {
                        Ok(right) => {
                            mutable_expression = Expression::Binary {
                                left: Box::new(mutable_expression),
                                operator,
                                right: Box::new(right),
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(mutable_expression);
            }
            Err(error) => Err(error),
        }
    }

    fn comparison(&mut self) -> Result<Expression, ParsingError> {
        match self.term() {
            Ok(expression) => {
                let mut mutable_expression = expression.clone();
                while self.match_token_type(&[
                    TokenType::Greater,
                    TokenType::GreaterEqual,
                    TokenType::Less,
                    TokenType::LessEqual,
                ]) {
                    let operator: Token = self.previous();
                    match self.term() {
                        Ok(right) => {
                            mutable_expression = Expression::Binary {
                                left: Box::new(mutable_expression),
                                operator,
                                right: Box::new(right),
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(mutable_expression);
            }
            Err(error) => Err(error),
        }
    }

    fn term(&mut self) -> Result<Expression, ParsingError> {
        match self.factor() {
            Ok(expression) => {
                let mut mutable_expression = expression.clone();
                while self.match_token_type(&[TokenType::Minus, TokenType::Plus]) {
                    let operator: Token = self.previous();
                    match self.factor() {
                        Ok(right) => {
                            mutable_expression = Expression::Binary {
                                left: Box::new(mutable_expression),
                                operator,
                                right: Box::new(right),
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(mutable_expression);
            }
            Err(error) => Err(error),
        }
    }

    fn factor(&mut self) -> Result<Expression, ParsingError> {
        match self.unary() {
            Ok(expression) => {
                let mut mutable_expression = expression.clone();
                while self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
                    let operator: Token = self.previous();
                    match self.unary() {
                        Ok(right) => {
                            mutable_expression = Expression::Binary {
                                left: Box::new(mutable_expression),
                                operator,
                                right: Box::new(right),
                            }
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                }
                return Ok(mutable_expression);
            }
            Err(error) => Err(error),
        }
    }

    fn unary(&mut self) -> Result<Expression, ParsingError> {
        match self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
            true => {
                let operator: Token = self.previous();
                let right: Result<Expression, ParsingError> = self.unary();
                match right {
                    Ok(right_expression) => Ok(Expression::Unary {
                        operator,
                        right: Box::new(right_expression),
                    }),
                    Err(error) => Err(error),
                }
            }
            _ => return self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expression, ParsingError> {
        if self.match_token_type(&[TokenType::False]) {
            return Ok(Expression::Literal {
                value: LiteralValue::Boolean(false),
            });
        }
        if self.match_token_type(&[TokenType::True]) {
            return Ok(Expression::Literal {
                value: LiteralValue::Boolean(true),
            });
        }
        if self.match_token_type(&[TokenType::Identifier]) {
            return Ok(Expression::Variable {
                name: self.previous(),
            });
        }
        if self.match_token_type(&[TokenType::Nil]) {
            return Ok(Expression::Literal {
                value: LiteralValue::Nil,
            });
        }
        if self.match_token_type(&[TokenType::Number, TokenType::String]) {
            return Ok(Expression::Literal {
                value: match self.previous().literal {
                    LiteralValue::String(v) => LiteralValue::String(v),
                    LiteralValue::Float(v) => LiteralValue::Float(v),
                    _ => LiteralValue::None,
                },
            });
        }
        if self.match_token_type(&[TokenType::LeftParen]) {
            match self.expression() {
                Ok(expr) => match self.consume(
                    TokenType::RightParen,
                    "Expect ')' after expression.".to_string(),
                ) {
                    Ok(_) => {
                        return Ok(Expression::Grouping {
                            expression: Box::new(expr),
                        })
                    }
                    Err(error) => return Err(error),
                },
                Err(error) => return Err(error),
            }
        }

        let token = self.peek();
        lox_parser_error(token.clone(), "Expected expression".to_string());
        return Err(ParsingError::new(
            "expected expression".to_string(),
            token.clone(),
        ));
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
        match self.is_at_end() {
            true => false,
            _ => self.peek().token_type == _type,
        }
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

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, ParsingError> {
        match self.check(token_type) {
            true => Ok(self.advance()),
            _ => {
                let token = self.peek();
                lox_parser_error(token.clone(), message.clone());
                Err(ParsingError::new(message, token.clone()))
            }
        }
    }
}
