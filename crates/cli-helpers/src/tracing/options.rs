use super::{LogLevel, init_tracing};
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Configuration for process-wide tracing initialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TracingOptions {
    level: LogLevel,
}

impl TracingOptions {
    /// Creates tracing options for the given level.
    #[must_use]
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }

    /// Returns the configured log level.
    #[must_use]
    pub fn level(&self) -> LogLevel {
        self.level
    }

    /// Installs the tracing subscriber for the current process.
    ///
    /// # Errors
    ///
    /// Returns an error when a global tracing subscriber is already installed.
    pub fn init(self, no_color: bool) -> Result {
        init_tracing(self.level, no_color)
    }
}
