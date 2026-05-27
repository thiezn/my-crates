use super::{OutputArgs, TracingArgs};
use crate::{CommandContext, CommandContextBuilder, Result};

/// Common CLI arguments shared across binaries using cli-helpers.
#[derive(::clap::Args, Debug, Clone, Default)]
pub struct CommonArgs {
    #[command(flatten)]
    output: OutputArgs,
    #[command(flatten)]
    tracing: TracingArgs,
    #[arg(long, global = true, help = "Disable ANSI color output")]
    no_color: bool,
}

impl CommonArgs {
    /// Builds a command context from the parsed CLI arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if any field selector is invalid.
    pub fn command_context(&self) -> Result<CommandContext> {
        Ok(self.command_context_builder()?.build())
    }

    /// Builds a command context builder from the parsed CLI arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if any field selector is invalid.
    pub fn command_context_builder(&self) -> Result<CommandContextBuilder> {
        Ok(CommandContext::builder()
            .no_color(self.no_color)
            .output(self.output.to_output_options()?))
    }

    /// Installs tracing from the parsed CLI arguments.
    ///
    /// # Errors
    ///
    /// Returns an error if tracing cannot be installed.
    pub fn init_tracing(&self) -> Result {
        self.tracing.init_tracing(self.no_color)
    }

    /// Returns whether ANSI colors should be disabled.
    #[must_use]
    pub fn no_color(&self) -> bool {
        self.no_color
    }

    /// Returns the parsed output arguments.
    #[must_use]
    pub fn output(&self) -> &OutputArgs {
        &self.output
    }

    /// Returns the parsed tracing arguments.
    #[must_use]
    pub fn tracing(&self) -> &TracingArgs {
        &self.tracing
    }
}
