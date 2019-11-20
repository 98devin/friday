use std::error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error + 'static>>;

#[derive(Debug)]
pub enum FridayError {
    InvalidFilename(String),
}

impl error::Error for FridayError {}

impl fmt::Display for FridayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FridayError::InvalidFilename(s) => write!(f, "Filename was invalid: {}", s),
        }
    }
}
