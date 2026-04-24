mod parser;
mod types;

pub use parser::parse_markdown;
pub use types::{
    MarkdownDocument, MarkdownLine, MarkdownLineKind, MarkdownSpan, MarkdownSpanStyle,
};
