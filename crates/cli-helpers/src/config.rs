//! Helpers for loading and saving TOML-backed configuration files.

use crate::error::{Error, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

/// Loads a configuration value from a TOML file.
///
/// Missing files return `C::default()` instead of an error.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read or parsed.
pub fn load<C: DeserializeOwned + Default>(path: &Path) -> Result<C> {
    if !path.exists() {
        return Ok(C::default());
    }

    load_from_path(path)
}

/// Loads a configuration value, creating a default file when needed.
///
/// # Errors
///
/// Returns an error if the default cannot be serialized or if the file cannot be read.
pub fn load_or_create<C: Serialize + DeserializeOwned + Default>(path: &Path) -> Result<C> {
    if !path.exists() {
        let config = C::default();
        save(&config, path)?;
        return Ok(config);
    }

    load_from_path(path)
}

/// Saves a configuration value to a TOML file.
///
/// Parent directories are created automatically.
///
/// # Errors
///
/// Returns an error if directory creation, serialization, or file writing fails.
pub fn save<C: Serialize>(config: &C, path: &Path) -> Result {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| Error::CreateDirectory {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let serialized = toml::to_string_pretty(config).map_err(Error::from)?;

    std::fs::write(path, serialized).map_err(|source| Error::WriteFile {
        path: path.to_path_buf(),
        source,
    })
}

fn load_from_path<C: DeserializeOwned>(path: &Path) -> Result<C> {
    let contents = std::fs::read_to_string(path).map_err(|source| Error::ReadFile {
        path: path.to_path_buf(),
        source,
    })?;

    toml::from_str(&contents).map_err(|source| Error::ParseToml {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

    use super::*;
    use serde::Deserialize;
    use std::io::Write as _;
    use std::path::PathBuf;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
    #[serde(default)]
    struct TestConfig {
        log_level: String,
        custom_field: Option<String>,
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}_{}_{}", std::process::id(), nanos))
    }

    #[test]
    fn load_returns_defaults_when_no_file() {
        let dir = unique_temp_dir("cli_helpers_config_defaults");
        let path = dir.join("nope.toml");
        let config: TestConfig = load(&path).unwrap();
        assert_eq!(config, TestConfig::default());
    }

    #[test]
    fn load_or_create_bootstraps_missing_file() {
        let dir = unique_temp_dir("cli_helpers_config_bootstrap");
        let path = dir.join("config.toml");
        let config: TestConfig = load_or_create(&path).unwrap();

        assert!(path.exists());
        assert_eq!(config, TestConfig::default());

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn save_and_reload_roundtrip() {
        let dir = unique_temp_dir("cli_helpers_config_roundtrip");
        let path = dir.join("config.toml");

        let config = TestConfig {
            log_level: "debug".to_string(),
            custom_field: Some("hello".to_string()),
        };

        save(&config, &path).unwrap();
        let loaded: TestConfig = load(&path).unwrap();
        assert_eq!(config, loaded);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn ignores_unknown_sections() {
        let dir = unique_temp_dir("cli_helpers_config_unknown");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("config.toml");

        let mut file = std::fs::File::create(&path).unwrap();
        writeln!(
            file,
            r#"log_level = "info"

[some_future_section]
foo = "bar"
"#
        )
        .unwrap();

        let config: TestConfig = load(&path).unwrap();
        assert_eq!(config.log_level, "info");

        let _ = std::fs::remove_dir_all(dir);
    }
}
