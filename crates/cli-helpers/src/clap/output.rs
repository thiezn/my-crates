use crate::Result;
use crate::output::{OutputFormat, OutputOptions};
use std::path::{Path, PathBuf};

/// Shared CLI arguments for structured output behavior.
#[derive(::clap::Args, Debug, Clone, Default)]
pub struct OutputArgs {
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,
    #[arg(long, global = true, value_name = "PATH")]
    output: Option<PathBuf>,
    #[arg(long = "field", global = true, value_name = "SELECTOR")]
    fields: Vec<String>,
}

impl OutputArgs {
    /// Converts the parsed CLI arguments into validated output options.
    ///
    /// # Errors
    ///
    /// Returns an error if any field selector is invalid.
    pub fn to_output_options(&self) -> Result<OutputOptions> {
        let mut options = OutputOptions::new(self.format());
        if let Some(path) = &self.output {
            options = options.with_output_path(path.clone());
        }

        options.try_with_field_selectors(self.fields.clone())
    }

    /// Returns the configured output format.
    #[must_use]
    pub fn format(&self) -> OutputFormat {
        self.format
    }

    /// Returns the configured output path.
    #[must_use]
    pub fn output_path(&self) -> Option<&Path> {
        self.output.as_deref()
    }

    /// Returns the raw field selector strings.
    #[must_use]
    pub fn fields(&self) -> &[String] {
        &self.fields
    }
}
