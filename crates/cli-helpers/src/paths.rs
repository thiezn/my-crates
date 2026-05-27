//! Path resolution helpers for CLI tools.

use crate::error::{Error, Result};
use std::path::{Path, PathBuf};

fn home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or(Error::HomeDirectoryUnavailable)
}

/// Resolves a path against the current working directory and `~` home expansion.
///
/// # Errors
///
/// Returns an error if the current directory or home directory cannot be determined.
pub fn resolve_path(path: &Path) -> Result<PathBuf> {
    if let Some(path_str) = path.to_str()
        && let Some(rest) = path_str.strip_prefix("~/")
    {
        return Ok(home_dir()?.join(rest));
    }

    if path == Path::new("~") {
        return home_dir();
    }

    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }

    let current_dir = std::env::current_dir().map_err(Error::CurrentDirectory)?;
    Ok(current_dir.join(path))
}

/// Resolves a string path against the current working directory and `~` home expansion.
///
/// # Errors
///
/// Returns an error if the path cannot be resolved.
pub fn resolve_path_str(path: &str) -> Result<PathBuf> {
    resolve_path(Path::new(path))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

    use super::*;

    #[test]
    fn resolve_absolute_path() {
        let path = resolve_path(Path::new("/tmp/test.toml")).unwrap();
        assert_eq!(path, PathBuf::from("/tmp/test.toml"));
    }

    #[test]
    fn resolve_tilde_path() {
        let path = resolve_path(Path::new("~/test.toml")).unwrap();
        let home = dirs::home_dir().unwrap();
        assert_eq!(path, home.join("test.toml"));
    }

    #[test]
    fn resolve_relative_path() {
        let path = resolve_path(Path::new("test.toml")).unwrap();
        let current_dir = std::env::current_dir().unwrap();
        assert_eq!(path, current_dir.join("test.toml"));
    }
}
