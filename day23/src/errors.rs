use std::error;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum ErrorKind {
    UnrecognizedInstruction,
    MissingArgument,
    WrongArgumentType,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn missing_argument() -> Error {
        Error {
            kind: ErrorKind::MissingArgument,
        }
    }

    pub fn unrecognized_instruction() -> Error {
        Error {
            kind: ErrorKind::UnrecognizedInstruction,
        }
    }

    pub fn wrong_type() -> Error {
        Error {
            kind: ErrorKind::WrongArgumentType,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::UnrecognizedInstruction => write!(f, "Unrecognized instruction"),
            ErrorKind::MissingArgument => write!(f, "Missing argument"),
            ErrorKind::WrongArgumentType => {
                write!(f, "Literal value supplied where register required")
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Invalid instruction"
    }
}
