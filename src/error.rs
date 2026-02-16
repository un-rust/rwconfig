//! Unified error type for the crate.

use std::fmt;
use std::io;

/// Errors that can occur when reading, writing, or modifying config.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse(String),
    Path(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io: {}", e),
            Error::Parse(s) => write!(f, "parse: {}", s),
            Error::Path(s) => write!(f, "path: {}", s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<Error> for io::Error {
    fn from(e: Error) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, e.to_string())
    }
}
