//! Structured output helpers shared by CLI commands.

mod fields;
mod format;
mod options;
mod render;

pub use fields::FieldSelector;
pub use format::OutputFormat;
pub use options::OutputOptions;
