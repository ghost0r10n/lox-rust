#[derive(Debug, Clone)]
pub enum LiteralValue {
    Float(f64),
    Boolean(bool),
    String(String),
    Nil,
    None,
}
impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Float(value) => write!(f, "{}", value),
            LiteralValue::Boolean(value) => write!(f, "{}", value),
            LiteralValue::String(value) => write!(f, "{}", value),
            LiteralValue::Nil => write!(f, "Nil"),
            LiteralValue::None => write!(f, "None"),
            // Add display logic for other variants if necessary
        }
    }
}
impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralValue::Float(lv), LiteralValue::Float(rv)) => lv == rv,
            (LiteralValue::Boolean(lv), LiteralValue::Boolean(rv)) => lv == rv,
            (LiteralValue::String(lv), LiteralValue::String(rv)) => lv == rv,
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            (LiteralValue::None, LiteralValue::None) => true,
            _ => false,
        }
    }
}
