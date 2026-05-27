use serde::{Deserialize, Deserializer, Serialize};

/// Output formats supported by cli-helpers.
#[derive(::clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Render structured JSON output.
    #[default]
    Json,
    /// Render Markdown output through a caller-provided renderer.
    Markdown,
}

impl OutputFormat {
    /// Returns the canonical string representation of the format.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Markdown => "markdown",
        }
    }

    /// Parses an output format name.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        let value = value.trim();
        if value.eq_ignore_ascii_case("json") {
            Some(Self::Json)
        } else if value.eq_ignore_ascii_case("markdown") {
            Some(Self::Markdown)
        } else {
            None
        }
    }
}

impl<'de> Deserialize<'de> for OutputFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).ok_or_else(|| {
            serde::de::Error::custom("unknown output format; expected one of json or markdown")
        })
    }
}
