/// A parsed Markdown document represented as rendered lines.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MarkdownDocument {
    /// The rendered lines in display order.
    pub lines: Vec<MarkdownLine>,
}

/// A rendered line in a Markdown document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownLine {
    /// The semantic kind of line.
    pub kind: MarkdownLineKind,
    /// The display prefix for the line, such as bullets or quote markers.
    pub prefix: String,
    /// The styled spans that make up the line body.
    pub spans: Vec<MarkdownSpan>,
}

impl MarkdownLine {
    /// Creates a blank line.
    #[must_use]
    pub fn blank() -> Self {
        Self {
            kind: MarkdownLineKind::Blank,
            prefix: String::new(),
            spans: Vec::new(),
        }
    }

    /// Returns whether the line renders as blank.
    #[must_use]
    pub fn is_blank(&self) -> bool {
        matches!(self.kind, MarkdownLineKind::Blank)
            || (self.prefix.is_empty() && self.spans.iter().all(|span| span.text.is_empty()))
    }
}

/// The semantic kind of a rendered Markdown line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownLineKind {
    /// A normal paragraph line.
    Paragraph,
    /// A heading line.
    Heading {
        /// The heading depth, where `1` is an H1.
        level: u8,
    },
    /// A quoted line.
    Quote,
    /// A list item line.
    ListItem,
    /// A fenced or indented code block line.
    CodeBlock {
        /// The optional code block language tag.
        language: Option<String>,
    },
    /// A blank line.
    Blank,
}

/// A styled span of rendered Markdown text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownSpan {
    /// The span text.
    pub text: String,
    /// The text styling flags for the span.
    pub style: MarkdownSpanStyle,
}

/// Styling flags applied to a Markdown span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MarkdownSpanStyle {
    /// Whether the span is bold.
    pub strong: bool,
    /// Whether the span is emphasized.
    pub emphasis: bool,
    /// Whether the span is inline code.
    pub code: bool,
    /// Whether the span is strikethrough text.
    pub strikethrough: bool,
    /// Whether the span represents a link.
    pub link: bool,
}
