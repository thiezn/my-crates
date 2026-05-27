use super::render;
use super::{FieldSelector, OutputFormat};
use crate::error::Result;
use serde::Serialize;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Output configuration shared across CLI commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputOptions {
    format: OutputFormat,
    output_path: Option<PathBuf>,
    field_selectors: Vec<FieldSelector>,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::Json,
            output_path: None,
            field_selectors: Vec::new(),
        }
    }
}

impl OutputOptions {
    /// Creates output options for the given format.
    #[must_use]
    pub fn new(format: OutputFormat) -> Self {
        Self {
            format,
            ..Self::default()
        }
    }

    /// Returns the configured output format.
    #[must_use]
    pub fn format(&self) -> OutputFormat {
        self.format
    }

    /// Returns the configured output path, when one has been supplied.
    #[must_use]
    pub fn output_path(&self) -> Option<&Path> {
        self.output_path.as_deref()
    }

    /// Returns the validated field selectors.
    #[must_use]
    pub fn field_selectors(&self) -> &[FieldSelector] {
        &self.field_selectors
    }

    /// Sets the output path for rendered content.
    #[must_use]
    pub fn with_output_path(mut self, output_path: impl Into<PathBuf>) -> Self {
        self.output_path = Some(output_path.into());
        self
    }

    /// Adds a validated field selector.
    #[must_use]
    pub fn with_field_selector(mut self, field_selector: FieldSelector) -> Self {
        self.field_selectors.push(field_selector);
        self
    }

    /// Replaces the field selector list with parsed selectors.
    ///
    /// # Errors
    ///
    /// Returns an error if any selector is malformed.
    pub fn try_with_field_selectors<I, S>(mut self, field_selectors: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.field_selectors = field_selectors
            .into_iter()
            .map(FieldSelector::parse)
            .collect::<Result<Vec<_>>>()?;
        Ok(self)
    }

    /// Writes output using the configured format.
    ///
    /// # Errors
    ///
    /// Returns an error when serialization, validation, or writing fails.
    pub fn write<T: Serialize>(
        &self,
        stdout: &mut dyn Write,
        stderr: &mut dyn Write,
        data: &T,
        markdown_renderer: Option<&dyn Fn(&T) -> String>,
    ) -> Result {
        render::write_output(self, stdout, stderr, data, markdown_renderer)
    }
}
