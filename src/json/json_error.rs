use std::fmt;
pub type Result<T> = std::result::Result<T, JsonError>;

#[derive(Debug, Clone)]
pub struct JsonError {
    message: String,
}

impl JsonError {
    pub(crate) fn new(messeage: String) -> JsonError {
        JsonError { message: messeage }
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
