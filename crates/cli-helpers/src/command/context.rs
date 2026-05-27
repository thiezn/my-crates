use crate::error::Result;
#[cfg(feature = "output")]
use crate::output::OutputOptions;
#[cfg(feature = "output")]
use serde::Serialize;
use std::io::{self, Write};

/// Builds a [`CommandContext`] with the process-level services a command needs.
pub struct CommandContextBuilder {
    stdout: Box<dyn Write + Send>,
    stderr: Box<dyn Write + Send>,
    no_color: bool,
    #[cfg(feature = "output")]
    output: OutputOptions,
}

impl Default for CommandContextBuilder {
    fn default() -> Self {
        Self {
            stdout: Box::new(io::stdout()),
            stderr: Box::new(io::stderr()),
            no_color: false,
            #[cfg(feature = "output")]
            output: OutputOptions::default(),
        }
    }
}

impl CommandContextBuilder {
    /// Creates a new builder backed by the process stdout and stderr streams.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the stdout writer used by commands.
    #[must_use]
    pub fn stdout(mut self, stdout: impl Write + Send + 'static) -> Self {
        self.stdout = Box::new(stdout);
        self
    }

    /// Replaces the stderr writer used by commands.
    #[must_use]
    pub fn stderr(mut self, stderr: impl Write + Send + 'static) -> Self {
        self.stderr = Box::new(stderr);
        self
    }

    /// Disables ANSI colors for commands that render styled output.
    #[must_use]
    pub fn no_color(mut self, no_color: bool) -> Self {
        self.no_color = no_color;
        self
    }

    /// Configures structured output handling for commands.
    #[cfg(feature = "output")]
    #[must_use]
    pub fn output(mut self, output: OutputOptions) -> Self {
        self.output = output;
        self
    }

    /// Finalizes the builder into a concrete command context.
    #[must_use]
    pub fn build(self) -> CommandContext {
        CommandContext {
            stdout: self.stdout,
            stderr: self.stderr,
            no_color: self.no_color,
            #[cfg(feature = "output")]
            output: self.output,
        }
    }
}

/// Runtime services shared with reusable command crates.
pub struct CommandContext {
    stdout: Box<dyn Write + Send>,
    stderr: Box<dyn Write + Send>,
    no_color: bool,
    #[cfg(feature = "output")]
    output: OutputOptions,
}

impl CommandContext {
    /// Creates a builder for a new command context.
    #[must_use]
    pub fn builder() -> CommandContextBuilder {
        CommandContextBuilder::new()
    }

    /// Returns the configured stdout writer.
    pub fn stdout(&mut self) -> &mut dyn Write {
        &mut *self.stdout
    }

    /// Returns the configured stderr writer.
    pub fn stderr(&mut self) -> &mut dyn Write {
        &mut *self.stderr
    }

    /// Writes a single line to stdout.
    ///
    /// # Errors
    ///
    /// Returns an error when the underlying writer fails.
    pub fn stdout_line(&mut self, line: impl AsRef<str>) -> Result {
        writeln!(self.stdout, "{}", line.as_ref())?;
        Ok(())
    }

    /// Writes a single line to stderr.
    ///
    /// # Errors
    ///
    /// Returns an error when the underlying writer fails.
    pub fn stderr_line(&mut self, line: impl AsRef<str>) -> Result {
        writeln!(self.stderr, "{}", line.as_ref())?;
        Ok(())
    }

    /// Returns whether ANSI colors should be disabled.
    #[must_use]
    pub fn no_color(&self) -> bool {
        self.no_color
    }

    /// Returns the configured structured output options.
    #[cfg(feature = "output")]
    #[must_use]
    pub fn output(&self) -> &OutputOptions {
        &self.output
    }

    /// Writes structured output using the configured output options.
    ///
    /// # Errors
    ///
    /// Returns an error when serialization or I/O fails.
    #[cfg(feature = "output")]
    pub fn render<T: Serialize>(&mut self, data: &T) -> Result {
        self.output
            .write(&mut self.stdout, &mut self.stderr, data, None)
    }

    /// Writes Markdown output using the configured output options.
    ///
    /// # Errors
    ///
    /// Returns an error when output is not configured for Markdown or when I/O fails.
    #[cfg(feature = "output")]
    pub fn render_markdown<T: Serialize>(
        &mut self,
        data: &T,
        renderer: &dyn Fn(&T) -> String,
    ) -> Result {
        self.output
            .write(&mut self.stdout, &mut self.stderr, data, Some(renderer))
    }
}
