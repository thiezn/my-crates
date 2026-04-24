use std::error::Error as StdError;
use std::fmt;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Config(String),
    Io(String),
    Network(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Config(msg) => write!(f, "Config error: {msg}"),
            Error::Io(msg) => write!(f, "IO error: {msg}"),
            Error::Network(msg) => write!(f, "Network error: {msg}"),
            Error::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl StdError for Error {}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        Self::Other(format!("Arc lock poisoned: {error}"))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

#[cfg(feature = "output")]
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_owned())
    }
}
