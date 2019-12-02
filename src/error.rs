use std::error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error + 'static>>;

#[derive(Debug)]
pub enum FridayError {
    InvalidFilename(String),
    UnresolvableModulePath(String),
    UnexpectedModuleAlias,
    UnexpectedModuleRecord,
}

pub use FridayError::*;

impl error::Error for FridayError {}

impl fmt::Display for FridayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidFilename(s) => write!(f, "Filename was invalid: {}", s),
            UnresolvableModulePath(s) => write!(f, "Unresolvable path: {}", s),
            UnexpectedModuleAlias => write!(f, "Expected module record, got alias."),
            UnexpectedModuleRecord => write!(f, "Expected module alias, got record."),
        }
    }
}
