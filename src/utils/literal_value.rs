#[derive(Debug, Clone)]
pub enum LiteralValue {
    Float(f64),
    Boolean(bool),
    String(String),
    Nil,
    None,
}
