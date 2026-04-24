#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MarkdownDocument {
    pub lines: Vec<MarkdownLine>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownLine {
    pub kind: MarkdownLineKind,
    pub prefix: String,
    pub spans: Vec<MarkdownSpan>,
}

impl MarkdownLine {
    pub fn blank() -> Self {
        Self {
            kind: MarkdownLineKind::Blank,
            prefix: String::new(),
            spans: Vec::new(),
        }
    }

    pub fn is_blank(&self) -> bool {
        matches!(self.kind, MarkdownLineKind::Blank)
            || (self.prefix.is_empty() && self.spans.iter().all(|span| span.text.is_empty()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownLineKind {
    Paragraph,
    Heading { level: u8 },
    Quote,
    ListItem,
    CodeBlock { language: Option<String> },
    Blank,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownSpan {
    pub text: String,
    pub style: MarkdownSpanStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MarkdownSpanStyle {
    pub strong: bool,
    pub emphasis: bool,
    pub code: bool,
    pub strikethrough: bool,
    pub link: bool,
}
