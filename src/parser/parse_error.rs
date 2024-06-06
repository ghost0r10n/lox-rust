type Result<T> = std::result::Result<T, ParsingError>;

#[derive(Debug, Clone)]
pub struct ParsingError {
    message: String,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing error")
    }
}

impl ParsingError {
    pub fn new(message: String) -> Self {
        ParsingError { message }
    }
}
