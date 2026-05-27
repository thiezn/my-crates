//! Interactive prompts built on top of dialoguer.

use crate::error::{Error, Result};
use std::fmt::Display;
use std::str::FromStr;

/// Prompts for free-form text input.
///
/// # Errors
///
/// Returns an error if the terminal interaction fails.
pub fn text_input(prompt: &str, default: Option<&str>) -> Result<String> {
    let mut input = dialoguer::Input::<String>::new().with_prompt(prompt);
    if let Some(default_value) = default {
        input = input.default(default_value.to_string());
    }

    input.interact_text().map_err(Error::from)
}

/// Prompts for text input that must not be empty.
///
/// # Errors
///
/// Returns an error if validation or terminal interaction fails.
pub fn text_input_required(prompt: &str) -> Result<String> {
    dialoguer::Input::<String>::new()
        .with_prompt(prompt)
        .validate_with(|input: &String| -> std::result::Result<(), &str> {
            if input.trim().is_empty() {
                Err("Value cannot be empty")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .map_err(Error::from)
}

/// Prompts the user to select one item from a list.
///
/// # Errors
///
/// Returns an error if terminal interaction fails.
pub fn select(prompt: &str, items: &[&str], default: Option<usize>) -> Result<usize> {
    let mut selection = dialoguer::Select::new().with_prompt(prompt).items(items);
    if let Some(default_index) = default {
        selection = selection.default(default_index);
    }

    selection.interact().map_err(Error::from)
}

/// Prompts for a yes or no confirmation.
///
/// # Errors
///
/// Returns an error if terminal interaction fails.
pub fn confirm(prompt: &str, default: Option<bool>) -> Result<bool> {
    let mut confirmation = dialoguer::Confirm::new().with_prompt(prompt);
    if let Some(default_value) = default {
        confirmation = confirmation.default(default_value);
    }

    confirmation.interact().map_err(Error::from)
}

/// Prompts for a numeric value with an optional default.
///
/// # Errors
///
/// Returns an error if terminal interaction fails or parsing does not succeed.
pub fn number_input<T>(prompt: &str, default: Option<T>) -> Result<T>
where
    T: FromStr + Display + Clone,
    T::Err: Display,
{
    let mut input = dialoguer::Input::<String>::new().with_prompt(prompt);
    if let Some(ref default_value) = default {
        input = input.default(default_value.to_string());
    }

    let raw = input
        .validate_with(|value: &String| -> std::result::Result<(), String> {
            value
                .parse::<T>()
                .map(|_| ())
                .map_err(|error| format!("Invalid number: {error}"))
        })
        .interact_text()
        .map_err(Error::from)?;

    raw.parse::<T>()
        .map_err(|error| Error::Other(format!("Parse failed: {error}")))
}

/// Prompts for a hidden password or secret value.
///
/// # Errors
///
/// Returns an error if terminal interaction fails.
pub fn password_input(prompt: &str) -> Result<String> {
    dialoguer::Password::new()
        .with_prompt(prompt)
        .interact()
        .map_err(Error::from)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    #[test]
    fn number_parse_validation_rejects_non_numeric() {
        let result = "abc".parse::<u16>();
        assert!(result.is_err());
    }

    #[test]
    fn number_parse_validation_accepts_valid() {
        let result = "6969".parse::<u16>();
        assert_eq!(result.unwrap(), 6969);
    }

    #[test]
    fn number_parse_u64_accepts_large_value() {
        let result = "15000".parse::<u64>();
        assert_eq!(result.unwrap(), 15000);
    }
}
