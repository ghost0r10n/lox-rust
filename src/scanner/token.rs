use core::fmt;

use crate::utils::literal_value::LiteralValue;

use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexame: String,
    pub line: usize,
    pub literal: LiteralValue,
}

impl Token {
    pub fn new(token_type: TokenType, lexame: String, line: usize, literal: LiteralValue) -> Self {
        Self {
            token_type,
            lexame,
            line,
            literal,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.token_type, &self.lexame)
    }
}
