//! Unified error type for the crate.
//!
//! All config operations (load, save, get, set) return this error type.

use std::fmt;
use std::io;

/// Errors that can occur when reading, writing, or modifying config.
#[derive(Debug)]
pub enum Error {
    /// I/O error (e.g. file not found, permission denied).
    Io(io::Error),
    /// Parse error (invalid JSON/YAML/TOML).
    Parse(String),
    /// Path error (empty path, path segment not an object).
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn error_display() {
        assert!(Error::Parse("bad".into()).to_string().contains("parse"));
        assert!(Error::Path("empty".into()).to_string().contains("path"));
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file");
        assert!(Error::Io(io_err).to_string().contains("io"));
    }

    #[test]
    fn from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "x");
        let e: Error = io_err.into();
        match &e {
            Error::Io(_) => {}
            _ => panic!("expected Io"),
        }
    }

    #[test]
    fn error_source() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file");
        let e: Error = Error::Io(io_err);
        assert!(StdError::source(&e).is_some());

        let parse_err = Error::Parse("bad".into());
        assert!(StdError::source(&parse_err).is_none());

        let path_err = Error::Path("empty".into());
        assert!(StdError::source(&path_err).is_none());
    }

    #[test]
    fn from_error_to_io_error() {
        let e = Error::Parse("parse failed".into());
        let io_err: io::Error = e.into();
        assert_eq!(io_err.kind(), io::ErrorKind::InvalidData);
    }
}
