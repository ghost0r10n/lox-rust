use core::fmt;

use super::token_type::TokenType;

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub token_type: TokenType,
    pub lexame: String,
    pub line: usize,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexame: String,
        line: usize,
        literal: Option<String>,
    ) -> Self {
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
