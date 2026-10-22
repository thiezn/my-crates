use super::LogLevel;
use crate::error::{Error, Result};
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::layer::{Layer, SubscriberExt};

/// Installs a process-global tracing subscriber.
///
/// If `RUST_LOG` is set, that environment filter takes precedence over `level`.
///
/// # Errors
///
/// Returns an error when another global subscriber has already been installed.
pub fn init_tracing(level: LogLevel, no_color: bool) -> Result {
    let filter = env_filter(level);
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_ansi(!no_color)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber).map_err(Error::from)
}

/// Installs a process-global tracing subscriber restricted to a target and its subtargets.
///
/// The `RUST_LOG` environment filter takes precedence over `level`, as it does for
/// [`init_tracing`]. Regardless of that filter, only events from `target` or its
/// subtargets are emitted.
///
/// # Errors
///
/// Returns an error when another global subscriber has already been installed.
pub fn init_tracing_for_target(level: LogLevel, no_color: bool, target: &str) -> Result {
    let subscriber = tracing_subscriber::registry().with(env_filter(level)).with(
        tracing_subscriber::fmt::layer()
            .without_time()
            .with_target(false)
            .with_ansi(!no_color)
            .with_filter(Targets::new().with_target(target, LevelFilter::TRACE)),
    );

    tracing::subscriber::set_global_default(subscriber).map_err(Error::from)
}

fn env_filter(level: LogLevel) -> tracing_subscriber::EnvFilter {
    if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::EnvFilter::from_default_env()
    } else {
        tracing_subscriber::EnvFilter::new(level.as_str())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct SharedWriter(Arc<Mutex<Vec<u8>>>);

    impl Write for SharedWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let mut output = self.0.lock().unwrap();
            output.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for SharedWriter {
        type Writer = Self;

        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    #[test]
    fn target_filter_excludes_other_crates() {
        let output = Arc::new(Mutex::new(Vec::new()));
        let subscriber = tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new("trace"))
            .with(
                tracing_subscriber::fmt::layer()
                    .without_time()
                    .with_target(false)
                    .with_writer(SharedWriter(Arc::clone(&output)))
                    .with_filter(Targets::new().with_target("utl", LevelFilter::TRACE)),
            );

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!(target: "utl::cli", "application event");
            tracing::info!(target: "azure_devops_rust_api::client", "dependency event");
        });

        let output = String::from_utf8(output.lock().unwrap().clone()).unwrap();
        assert!(output.contains("application event"));
        assert!(!output.contains("dependency event"));
    }
}
