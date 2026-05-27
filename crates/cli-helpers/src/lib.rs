#![doc = include_str!("../README.md")]

#[cfg(feature = "clap")]
pub mod clap;
pub mod command;
#[cfg(feature = "config")]
pub mod config;
pub mod error;
#[cfg(feature = "interactive")]
pub mod interactive;
#[cfg(feature = "markdown")]
pub mod markdown;
#[cfg(feature = "output")]
pub mod output;
#[cfg(feature = "paths")]
pub mod paths;
#[cfg(feature = "progress")]
pub mod progress;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "clap")]
pub use clap::{CommonArgs, OutputArgs, TracingArgs};
pub use command::{CommandContext, CommandContextBuilder, Runnable};
pub use error::{Error, Result};

#[cfg(feature = "output")]
pub use output::{FieldSelector, OutputFormat, OutputOptions};

#[cfg(feature = "paths")]
pub use paths::{resolve_path, resolve_path_str};

#[cfg(feature = "tracing")]
pub use tracing::{LogLevel, TracingOptions, init_tracing};
