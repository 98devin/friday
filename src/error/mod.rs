
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum FridayError {
    InvalidFilename(String),
}

impl error::Error for FridayError { }

impl fmt::Display for FridayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FridayError::InvalidFilename(s) => write!(f, "Filename was invalid: {}", s),
        }
    }
}
