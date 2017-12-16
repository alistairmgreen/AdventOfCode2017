use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParseDanceMoveError {
    error_description: String,
}

impl ParseDanceMoveError {
    pub fn new(description: &str) -> ParseDanceMoveError {
        ParseDanceMoveError {
            error_description: description.to_string()
        }
    }
}

impl fmt::Display for ParseDanceMoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_description)
    }
}

impl Error for ParseDanceMoveError {
    fn description(&self) -> &str {
        "Invalid dance move"
    }
}

impl From<ParseIntError> for ParseDanceMoveError
{
    fn from(error: ParseIntError) -> ParseDanceMoveError {
        ParseDanceMoveError::new(error.description())
    }
}
