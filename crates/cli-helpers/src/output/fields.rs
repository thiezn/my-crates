use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PathSegment {
    Key(String),
    Index(usize),
    Wildcard,
}

/// A validated selector for extracting fields from JSON output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldSelector {
    raw: String,
    segments: Vec<PathSegment>,
}

impl FieldSelector {
    /// Parses and validates a field selector.
    ///
    /// Supported selectors use dot-separated keys with optional array selectors,
    /// such as `user.name`, `items[0]`, or `items[*].id`.
    ///
    /// # Errors
    ///
    /// Returns an error when the selector is malformed.
    pub fn parse(input: impl Into<String>) -> Result<Self> {
        let raw = input.into();
        let segments = parse_segments(&raw)?;
        Ok(Self { raw, segments })
    }

    /// Returns the original selector string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    pub(crate) fn segments(&self) -> &[PathSegment] {
        &self.segments
    }
}

fn parse_segments(input: &str) -> Result<Vec<PathSegment>> {
    if input.trim().is_empty() {
        return Err(Error::InvalidFieldSelector {
            input: input.to_string(),
            reason: "selector cannot be empty",
        });
    }

    let mut segments = Vec::new();
    for part in input.split('.') {
        if part.is_empty() {
            return Err(Error::InvalidFieldSelector {
                input: input.to_string(),
                reason: "selector contains an empty path segment",
            });
        }

        let mut remainder = part;
        loop {
            if let Some(bracket_pos) = remainder.find('[') {
                let key = &remainder[..bracket_pos];
                if !key.is_empty() {
                    segments.push(PathSegment::Key(key.to_string()));
                }

                remainder = &remainder[bracket_pos + 1..];
                let closing = remainder
                    .find(']')
                    .ok_or_else(|| Error::InvalidFieldSelector {
                        input: input.to_string(),
                        reason: "missing closing bracket",
                    })?;

                let bracket_content = &remainder[..closing];
                if bracket_content == "*" {
                    segments.push(PathSegment::Wildcard);
                } else if let Ok(index) = bracket_content.parse::<usize>() {
                    segments.push(PathSegment::Index(index));
                } else {
                    return Err(Error::InvalidFieldSelector {
                        input: input.to_string(),
                        reason: "array selectors must use an index or '*'",
                    });
                }

                remainder = &remainder[closing + 1..];
                if remainder.is_empty() {
                    break;
                }

                if !remainder.starts_with('[') {
                    return Err(Error::InvalidFieldSelector {
                        input: input.to_string(),
                        reason: "unexpected characters after array selector",
                    });
                }
            } else {
                segments.push(PathSegment::Key(remainder.to_string()));
                break;
            }
        }
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

    use super::*;

    #[test]
    fn parses_simple_key() {
        let selector = FieldSelector::parse("name").unwrap();
        assert_eq!(selector.segments, vec![PathSegment::Key("name".into())]);
    }

    #[test]
    fn parses_nested_array_selectors() {
        let selector = FieldSelector::parse("items[0][1].id").unwrap();
        assert_eq!(
            selector.segments,
            vec![
                PathSegment::Key("items".into()),
                PathSegment::Index(0),
                PathSegment::Index(1),
                PathSegment::Key("id".into()),
            ]
        );
    }

    #[test]
    fn rejects_missing_bracket() {
        let error = FieldSelector::parse("items[0").unwrap_err();
        assert!(matches!(
            error,
            Error::InvalidFieldSelector {
                reason: "missing closing bracket",
                ..
            }
        ));
    }

    #[test]
    fn rejects_trailing_characters() {
        let error = FieldSelector::parse("items[0]tail").unwrap_err();
        assert!(matches!(
            error,
            Error::InvalidFieldSelector {
                reason: "unexpected characters after array selector",
                ..
            }
        ));
    }
}
