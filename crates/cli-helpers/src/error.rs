//! Shared error type used by the cli-helpers crate.

use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;

/// Convenient result alias for cli-helpers APIs.
pub type Result<T = ()> = std::result::Result<T, Error>;

/// Errors raised by cli-helpers primitives and adapters.
#[derive(Debug)]
pub enum Error {
    /// Failed to create a directory needed by the command.
    CreateDirectory {
        /// The directory that could not be created.
        path: PathBuf,
        /// The underlying I/O failure.
        source: std::io::Error,
    },
    /// Failed to determine the current working directory.
    CurrentDirectory(std::io::Error),
    #[cfg(feature = "interactive")]
    /// An interactive terminal prompt failed.
    Dialoguer(dialoguer::Error),
    /// Home directory expansion was requested but is unavailable on this machine.
    HomeDirectoryUnavailable,
    /// A field selector string was invalid.
    InvalidFieldSelector {
        /// The original selector text.
        input: String,
        /// A short explanation of what was invalid.
        reason: &'static str,
    },
    /// An I/O operation on a generic stream failed.
    Io(std::io::Error),
    /// Markdown output does not support JSON field filtering.
    MarkdownFieldsUnsupported,
    /// Markdown output was requested without a renderer callback.
    MissingMarkdownRenderer,
    /// A string-based error for cases that do not warrant a dedicated variant.
    Other(String),
    /// A synchronization primitive became poisoned.
    Poisoned(String),
    #[cfg(feature = "config")]
    /// Failed to parse TOML configuration.
    ParseToml {
        /// The file that was being parsed.
        path: PathBuf,
        /// The underlying TOML parse error.
        source: toml::de::Error,
    },
    #[cfg(feature = "output")]
    /// Failed to serialize JSON output.
    Json(serde_json::Error),
    /// Failed to read a file from disk.
    ReadFile {
        /// The file that was being read.
        path: PathBuf,
        /// The underlying I/O failure.
        source: std::io::Error,
    },
    #[cfg(feature = "config")]
    /// Failed to serialize TOML configuration.
    SerializeToml(toml::ser::Error),
    #[cfg(feature = "tracing")]
    /// Failed to install the process-global tracing subscriber.
    TracingSubscriber(tracing::subscriber::SetGlobalDefaultError),
    /// Failed to write a file to disk.
    WriteFile {
        /// The file that was being written.
        path: PathBuf,
        /// The underlying I/O failure.
        source: std::io::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CreateDirectory { path, source } => {
                write!(f, "failed to create '{}': {source}", path.display())
            }
            Error::CurrentDirectory(source) => {
                write!(f, "failed to determine the current directory: {source}")
            }
            #[cfg(feature = "interactive")]
            Error::Dialoguer(source) => write!(f, "interactive prompt failed: {source}"),
            Error::HomeDirectoryUnavailable => write!(f, "unable to determine the home directory"),
            Error::InvalidFieldSelector { input, reason } => {
                write!(f, "invalid field selector '{input}': {reason}")
            }
            Error::Io(source) => write!(f, "I/O error: {source}"),
            Error::MarkdownFieldsUnsupported => {
                write!(f, "field selectors are only supported for JSON output")
            }
            Error::MissingMarkdownRenderer => {
                write!(f, "markdown output requires a renderer callback")
            }
            Error::Other(msg) => write!(f, "{msg}"),
            Error::Poisoned(msg) => write!(f, "poisoned lock: {msg}"),
            #[cfg(feature = "config")]
            Error::ParseToml { path, source } => {
                write!(f, "failed to parse TOML '{}': {source}", path.display())
            }
            #[cfg(feature = "output")]
            Error::Json(source) => write!(f, "JSON error: {source}"),
            Error::ReadFile { path, source } => {
                write!(f, "failed to read '{}': {source}", path.display())
            }
            #[cfg(feature = "config")]
            Error::SerializeToml(source) => write!(f, "failed to serialize TOML: {source}"),
            #[cfg(feature = "tracing")]
            Error::TracingSubscriber(source) => {
                write!(f, "failed to initialize tracing: {source}")
            }
            Error::WriteFile { path, source } => {
                write!(f, "failed to write '{}': {source}", path.display())
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::CreateDirectory { source, .. } => Some(source),
            Error::CurrentDirectory(source) => Some(source),
            #[cfg(feature = "interactive")]
            Error::Dialoguer(source) => Some(source),
            Error::HomeDirectoryUnavailable => None,
            Error::InvalidFieldSelector { .. } => None,
            Error::Io(source) => Some(source),
            Error::MarkdownFieldsUnsupported => None,
            Error::MissingMarkdownRenderer => None,
            Error::Other(_) => None,
            Error::Poisoned(_) => None,
            #[cfg(feature = "config")]
            Error::ParseToml { source, .. } => Some(source),
            #[cfg(feature = "output")]
            Error::Json(source) => Some(source),
            Error::ReadFile { source, .. } => Some(source),
            #[cfg(feature = "config")]
            Error::SerializeToml(source) => Some(source),
            #[cfg(feature = "tracing")]
            Error::TracingSubscriber(source) => Some(source),
            Error::WriteFile { source, .. } => Some(source),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        Self::Poisoned(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[cfg(feature = "interactive")]
impl From<dialoguer::Error> for Error {
    fn from(error: dialoguer::Error) -> Self {
        Self::Dialoguer(error)
    }
}

#[cfg(feature = "output")]
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[cfg(feature = "config")]
impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        Self::SerializeToml(error)
    }
}

#[cfg(feature = "tracing")]
impl From<tracing::subscriber::SetGlobalDefaultError> for Error {
    fn from(error: tracing::subscriber::SetGlobalDefaultError) -> Self {
        Self::TracingSubscriber(error)
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
