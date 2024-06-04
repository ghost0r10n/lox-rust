use super::{token::Token, token_type::TokenType};
use crate::loxerror;
use crate::utils::reserved_words::KEYWORDS;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let tokens = Vec::new();
        let start = 0;
        let current = 0;
        let line = 1;
        Scanner {
            source,
            tokens,
            start,
            current,
            line,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexame: String::new(),
            line: self.line,
            literal: None,
        });
        self.tokens.to_vec()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() - 1
    }

    fn advance(&mut self) -> char {
        let char_vec: Vec<char> = self.source.chars().collect();
        let current_char = char_vec[self.current];
        if !self.is_at_end() {
            self.current += 1;
        }

        return current_char;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, self.line, literal))
    }

    fn match_token(&mut self, expected: String) -> bool {
        if self.is_at_end() {
            return false;
        }
        let char_vec: Vec<char> = self.source.chars().collect();
        if char_vec[self.current].to_string() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn scan_token(&mut self) {
        let character: char = self.advance();
        match character {
            //Small lexemes
            '(' => self.add_token(TokenType::LeftParen, Option::None),
            ')' => self.add_token(TokenType::RightParen, Option::None),
            '{' => self.add_token(TokenType::RightBrace, Option::None),
            '}' => self.add_token(TokenType::LeftBrace, Option::None),
            ',' => self.add_token(TokenType::Comma, Option::None),
            '.' => self.add_token(TokenType::Dot, Option::None),
            '-' => self.add_token(TokenType::Minus, Option::None),
            '+' => self.add_token(TokenType::Plus, Option::None),
            ';' => self.add_token(TokenType::Semicolon, Option::None),
            '*' => self.add_token(TokenType::Star, Option::None),
            '=' => self.scan_equal_equal_token(),
            '!' => self.scan_bang_equal_token(),
            '<' => self.scan_less_equal_token(),
            '>' => self.scan_greater_equal_token(),
            '/' => self.scan_slash_token(),
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => loxerror(self.line, "Unexpected Character".to_string()),
        }
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || ('0'..='9').contains(&c);
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let token_type = &KEYWORDS
            .get(&text.as_str())
            .copied()
            .unwrap_or(TokenType::Identifier);

        self.add_token(token_type.to_owned(), None)
    }

    fn scan_slash_token(&mut self) {
        if self.match_token('/'.to_string()) {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
        } else {
            self.add_token(TokenType::Slash, None);
        }
    }

    fn number(&mut self) {
        while ('0'..='9').contains(&self.peek()) {
            self.advance();
        }

        while self.peek() == '.' && ('0'..='9').contains(&self.peek_next()) {
            self.advance();
            while ('0'..='9').contains(&self.peek()) {
                self.advance();
            }
        }
        self.add_token(
            TokenType::Number,
            Some(self.source[self.start..self.current].to_string()),
        )
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            loxerror(self.line, "rlox:: Undeterminated String".to_string());
            return;
        }
        self.advance();
        let value: String = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token(TokenType::String, Some(value))
    }

    fn scan_equal_equal_token(&mut self) {
        match self.match_token("=".to_string()) {
            true => self.add_token(TokenType::EqualEqual, None),
            false => self.add_token(TokenType::Equal, None),
        }
    }
    fn scan_less_equal_token(&mut self) {
        match self.match_token("=".to_string()) {
            true => self.add_token(TokenType::LessEqual, None),
            false => self.add_token(TokenType::Less, None),
        }
    }
    fn scan_bang_equal_token(&mut self) {
        match self.match_token("=".to_string()) {
            true => self.add_token(TokenType::BangEqual, None),
            false => self.add_token(TokenType::Bang, None),
        }
    }
    fn scan_greater_equal_token(&mut self) {
        match self.match_token("=".to_string()) {
            true => self.add_token(TokenType::GreaterEqual, None),
            false => self.add_token(TokenType::Greater, None),
        }
    }
    fn peek(&self) -> char {
        let char_vec: Vec<char> = self.source.chars().collect();
        if self.is_at_end() {
            return '\0';
        }
        return char_vec[self.current];
    }
    fn peek_next(&self) -> char {
        let char_vec: Vec<char> = self.source.chars().collect();
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return char_vec[self.current + 1];
    }
}
