use std::fmt::{Debug, Display, Formatter, Result};

/// Describes all possible Locket errors.
#[derive(Debug)]
pub enum Error {
    /// A wrapped `std::io::Error` error.
    IO(std::io::Error),

    /// An error occurred during FFI.
    FFI(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::IO(error) => write!(f, "io error: {}", error),
            Error::FFI(reason) => write!(f, "ffi error: {}", reason),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}
