#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TokenType {
    LCurlyBracket,
    RCurlyBracket,
    LBracket,
    RBracket,
    String,
    Colon,
    Comma,
    Int,
    Float,
    Error,
    Eof,
    Bool,
    Null,
}
