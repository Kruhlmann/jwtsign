use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidJWTError {
    pub message: String,
}

impl InvalidJWTError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for InvalidJWTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid JWT Error: {}", self.message)
    }
}

impl Error for InvalidJWTError {}
