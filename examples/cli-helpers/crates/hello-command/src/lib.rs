//! Reusable hello subcommand used by the example binaries.

mod command;
mod error;

pub use command::HelloCommand;
pub use error::{Error, Result};
