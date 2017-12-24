use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    MissingField,
    InvalidNumber
}

#[derive(Debug)]
pub struct ParseComponentError {
    kind: ErrorKind
}

impl ParseComponentError {
    pub fn missing_field() -> ParseComponentError {
        ParseComponentError {
            kind: ErrorKind::MissingField
        }
    }

    pub fn invalid_number() -> ParseComponentError {
        ParseComponentError {
            kind: ErrorKind::InvalidNumber
        }
    }
}

impl fmt::Display for ParseComponentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self.kind {
           ErrorKind::InvalidNumber => write!(f, "Invalid number"),
           ErrorKind::MissingField => write!(f, "Component does not have two connectors.")
       }
    }
}

impl Error for ParseComponentError {
    fn description(&self) -> &str {
        "Invalid component definition"
    }
}

impl From<ParseIntError> for ParseComponentError {
    fn from(_: ParseIntError) -> Self {
        ParseComponentError::invalid_number()
    }
}
