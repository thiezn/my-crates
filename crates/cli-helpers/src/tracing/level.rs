use serde::{Deserialize, Deserializer, Serialize};

/// Log levels supported by cli-helpers tracing setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Verbose trace-level logging.
    Trace,
    /// Debug logging.
    Debug,
    /// Informational logging.
    #[default]
    Info,
    /// Warning logging.
    Warn,
    /// Error logging only.
    Error,
}

impl LogLevel {
    /// Returns the canonical string representation of the log level.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }

    /// Parses a log level string.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        let value = value.trim();
        if value.eq_ignore_ascii_case("trace") {
            Some(Self::Trace)
        } else if value.eq_ignore_ascii_case("debug") {
            Some(Self::Debug)
        } else if value.eq_ignore_ascii_case("info") {
            Some(Self::Info)
        } else if value.eq_ignore_ascii_case("warn") {
            Some(Self::Warn)
        } else if value.eq_ignore_ascii_case("error") {
            Some(Self::Error)
        } else {
            None
        }
    }
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).ok_or_else(|| {
            serde::de::Error::custom(
                "unknown log level; expected one of trace, debug, info, warn, error",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::LogLevel;

    #[test]
    fn serializes_as_lowercase_string() {
        let serialized = serde_json::to_string(&LogLevel::Debug).unwrap();
        assert_eq!(serialized, "\"debug\"");
    }

    #[test]
    fn deserializes_case_insensitive_strings() {
        let deserialized = serde_json::from_str::<LogLevel>("\"DeBuG\"").unwrap();
        assert_eq!(deserialized, LogLevel::Debug);
    }
}
