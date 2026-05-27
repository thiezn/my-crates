use std::error::Error as StdError;
use std::fmt;

/// Result alias for the reusable hello command crate.
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Errors produced by the reusable hello command.
#[derive(Debug)]
pub enum Error {
    /// The shared CLI runtime returned an error.
    Cli(cli_helpers::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Cli(source) => write!(f, "hello command failed: {source}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Cli(source) => Some(source),
        }
    }
}

impl From<cli_helpers::Error> for Error {
    fn from(error: cli_helpers::Error) -> Self {
        Self::Cli(error)
    }
}
