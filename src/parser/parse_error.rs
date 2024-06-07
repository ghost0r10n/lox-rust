use crate::scanner::token::Token;

#[derive(Debug, Clone)]
pub struct ParsingError {
    pub token: Token,
    pub message: String,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing error")
    }
}

impl ParsingError {
    pub fn new(message: String, token: Token) -> Self {
        ParsingError { message, token }
    }
}
