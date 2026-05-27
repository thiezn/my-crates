use super::LogLevel;
use crate::error::{Error, Result};

/// Installs a process-global tracing subscriber.
///
/// If `RUST_LOG` is set, that environment filter takes precedence over `level`.
///
/// # Errors
///
/// Returns an error when another global subscriber has already been installed.
pub fn init_tracing(level: LogLevel, no_color: bool) -> Result {
    let filter = if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::EnvFilter::from_default_env()
    } else {
        tracing_subscriber::EnvFilter::new(level.as_str())
    };

    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_ansi(!no_color)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).map_err(Error::from)
}
