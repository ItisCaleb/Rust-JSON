#[derive(Debug, Clone, PartialEq)]
pub enum JsonType {
    Object,
    Array,
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Null,
}
