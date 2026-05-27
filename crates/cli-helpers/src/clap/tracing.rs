use crate::Result;
use crate::tracing::{LogLevel, TracingOptions};

/// Shared CLI arguments for tracing configuration.
#[derive(::clap::Args, Debug, Clone, Default)]
pub struct TracingArgs {
    #[arg(long, global = true, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
}

impl TracingArgs {
    /// Converts the parsed CLI arguments into tracing options.
    #[must_use]
    pub fn tracing_options(&self) -> TracingOptions {
        TracingOptions::new(self.level())
    }

    /// Installs tracing from the parsed CLI arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if tracing cannot be installed.
    pub fn init_tracing(&self, no_color: bool) -> Result {
        self.tracing_options().init(no_color)
    }

    /// Returns the configured log level.
    #[must_use]
    pub fn level(&self) -> LogLevel {
        self.log_level
    }
}
