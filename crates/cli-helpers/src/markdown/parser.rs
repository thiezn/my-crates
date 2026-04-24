use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use super::types::{
    MarkdownDocument, MarkdownLine, MarkdownLineKind, MarkdownSpan, MarkdownSpanStyle,
};

#[derive(Debug, Clone)]
struct ListState {
    ordered: bool,
    next_index: usize,
}

#[derive(Debug, Clone)]
struct ItemState {
    first_prefix: String,
    continuation_prefix: String,
    needs_first_prefix: bool,
}

#[derive(Debug, Clone)]
struct CodeBlockState {
    language: Option<String>,
    content: String,
}

#[derive(Debug, Clone)]
struct LinkState {
    destination: String,
    text: String,
}

#[derive(Debug, Default)]
struct ParserState {
    lines: Vec<MarkdownLine>,
    current_line: Option<MarkdownLine>,
    current_style: MarkdownSpanStyle,
    heading_level: Option<u8>,
    quote_depth: usize,
    list_stack: Vec<ListState>,
    item_state: Option<ItemState>,
    code_block: Option<CodeBlockState>,
    link_stack: Vec<LinkState>,
}

pub fn parse_markdown(input: &str) -> MarkdownDocument {
    let mut state = ParserState::default();
    let options = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION;

    for event in Parser::new_ext(input, options) {
        state.handle_event(event);
    }

    state.finish_line();
    state.trim_blank_lines();
    if state.lines.is_empty() {
        state.lines.push(MarkdownLine {
            kind: MarkdownLineKind::Paragraph,
            prefix: String::new(),
            spans: vec![MarkdownSpan {
                text: input.to_string(),
                style: MarkdownSpanStyle::default(),
            }],
        });
        state.trim_blank_lines();
    }

    MarkdownDocument { lines: state.lines }
}

impl ParserState {
    fn handle_event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag) => self.end_tag(tag),
            Event::Text(text) => self.push_text(text.as_ref(), self.current_style),
            Event::Code(text) => {
                let mut style = self.current_style;
                style.code = true;
                self.push_text(text.as_ref(), style);
            }
            Event::SoftBreak | Event::HardBreak => self.push_line_break(),
            Event::Rule => {
                self.finish_line();
                self.lines.push(MarkdownLine {
                    kind: MarkdownLineKind::Paragraph,
                    prefix: String::new(),
                    spans: vec![MarkdownSpan {
                        text: "────────────────".to_string(),
                        style: MarkdownSpanStyle {
                            strong: true,
                            ..MarkdownSpanStyle::default()
                        },
                    }],
                });
                self.push_blank_line();
            }
            Event::TaskListMarker(checked) => {
                let marker = if checked { "[x] " } else { "[ ] " };
                self.push_text(marker, self.current_style);
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                self.push_text(html.as_ref(), self.current_style);
            }
            Event::FootnoteReference(reference) => {
                self.push_text(&format!("[^{reference}]"), self.current_style);
            }
            Event::InlineMath(text) | Event::DisplayMath(text) => {
                let mut style = self.current_style;
                style.code = true;
                self.push_text(text.as_ref(), style);
            }
        }
    }

    fn start_tag(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Paragraph => {}
            Tag::Heading { level, .. } => {
                self.finish_line();
                self.heading_level = Some(heading_level(level));
            }
            Tag::BlockQuote(_) => {
                self.finish_line();
                self.quote_depth = self.quote_depth.saturating_add(1);
            }
            Tag::List(start) => {
                self.finish_line();
                self.list_stack.push(ListState {
                    ordered: start.is_some(),
                    next_index: start.unwrap_or(1) as usize,
                });
            }
            Tag::Item => {
                self.finish_line();
                self.item_state = Some(self.next_item_state());
            }
            Tag::Emphasis => self.current_style.emphasis = true,
            Tag::Strong => self.current_style.strong = true,
            Tag::Strikethrough => self.current_style.strikethrough = true,
            Tag::Link { dest_url, .. } => {
                self.current_style.link = true;
                self.link_stack.push(LinkState {
                    destination: dest_url.to_string(),
                    text: String::new(),
                });
            }
            Tag::CodeBlock(kind) => {
                self.finish_line();
                self.code_block = Some(CodeBlockState {
                    language: code_block_language(kind),
                    content: String::new(),
                });
            }
            _ => {}
        }
    }

    fn end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => {
                self.finish_line();
                if self.item_state.is_none() && self.quote_depth == 0 {
                    self.push_blank_line();
                }
            }
            TagEnd::Heading(_) => {
                self.finish_line();
                self.heading_level = None;
                self.push_blank_line();
            }
            TagEnd::BlockQuote(_) => {
                self.finish_line();
                self.quote_depth = self.quote_depth.saturating_sub(1);
                if self.quote_depth == 0 {
                    self.push_blank_line();
                }
            }
            TagEnd::List(_) => {
                self.finish_line();
                self.list_stack.pop();
                if self.list_stack.is_empty() {
                    self.push_blank_line();
                }
            }
            TagEnd::Item => {
                self.finish_line();
                self.item_state = None;
            }
            TagEnd::Emphasis => self.current_style.emphasis = false,
            TagEnd::Strong => self.current_style.strong = false,
            TagEnd::Strikethrough => self.current_style.strikethrough = false,
            TagEnd::Link => {
                self.current_style.link = false;
                if let Some(link) = self.link_stack.pop()
                    && !link.destination.is_empty()
                    && link.destination.trim() != link.text.trim()
                {
                    let style = MarkdownSpanStyle {
                        link: true,
                        ..self.current_style
                    };
                    self.push_text(&format!(" ({})", link.destination), style);
                }
            }
            TagEnd::CodeBlock => {
                self.finish_line();
                if let Some(code_block) = self.code_block.take() {
                    self.push_code_block(code_block);
                    self.push_blank_line();
                }
            }
            _ => {}
        }
    }

    fn push_text(&mut self, text: &str, style: MarkdownSpanStyle) {
        if text.is_empty() {
            return;
        }

        if let Some(code_block) = &mut self.code_block {
            code_block.content.push_str(text);
            return;
        }

        if let Some(link) = self.link_stack.last_mut() {
            link.text.push_str(text);
        }

        let line = self.ensure_line();
        if let Some(last) = line.spans.last_mut()
            && last.style == style
        {
            last.text.push_str(text);
            return;
        }

        line.spans.push(MarkdownSpan {
            text: text.to_string(),
            style,
        });
    }

    fn push_line_break(&mut self) {
        if let Some(code_block) = &mut self.code_block {
            code_block.content.push('\n');
        } else {
            self.finish_line();
        }
    }

    fn ensure_line(&mut self) -> &mut MarkdownLine {
        if self.current_line.is_none() {
            let kind = if let Some(level) = self.heading_level {
                MarkdownLineKind::Heading { level }
            } else if self.item_state.is_some() {
                MarkdownLineKind::ListItem
            } else if self.quote_depth > 0 {
                MarkdownLineKind::Quote
            } else {
                MarkdownLineKind::Paragraph
            };

            let mut prefix = String::new();
            if self.quote_depth > 0 {
                prefix.push_str(&"▎ ".repeat(self.quote_depth));
            }
            if let Some(item_state) = &mut self.item_state {
                if item_state.needs_first_prefix {
                    prefix.push_str(&item_state.first_prefix);
                    item_state.needs_first_prefix = false;
                } else {
                    prefix.push_str(&item_state.continuation_prefix);
                }
            }

            self.current_line = Some(MarkdownLine {
                kind,
                prefix,
                spans: Vec::new(),
            });
        }

        self.current_line.get_or_insert_with(MarkdownLine::blank)
    }

    fn finish_line(&mut self) {
        if let Some(line) = self.current_line.take()
            && (!line.spans.is_empty() || !line.prefix.is_empty())
        {
            self.lines.push(line);
        }
    }

    fn push_blank_line(&mut self) {
        if !matches!(self.lines.last(), Some(line) if line.is_blank()) {
            self.lines.push(MarkdownLine::blank());
        }
    }

    fn push_code_block(&mut self, code_block: CodeBlockState) {
        let language = code_block.language;
        let code = code_block.content.trim_end_matches('\n');
        if code.is_empty() {
            self.lines.push(MarkdownLine {
                kind: MarkdownLineKind::CodeBlock {
                    language: language.clone(),
                },
                prefix: "│ ".to_string(),
                spans: vec![MarkdownSpan {
                    text: String::new(),
                    style: MarkdownSpanStyle {
                        code: true,
                        ..MarkdownSpanStyle::default()
                    },
                }],
            });
            return;
        }

        for line in code.lines() {
            self.lines.push(MarkdownLine {
                kind: MarkdownLineKind::CodeBlock {
                    language: language.clone(),
                },
                prefix: "│ ".to_string(),
                spans: vec![MarkdownSpan {
                    text: line.to_string(),
                    style: MarkdownSpanStyle {
                        code: true,
                        ..MarkdownSpanStyle::default()
                    },
                }],
            });
        }
    }

    fn next_item_state(&mut self) -> ItemState {
        let depth = self.list_stack.len().saturating_sub(1) * 2;
        let indent = " ".repeat(depth);
        let Some(state) = self.list_stack.last_mut() else {
            return ItemState {
                first_prefix: "• ".to_string(),
                continuation_prefix: "  ".to_string(),
                needs_first_prefix: true,
            };
        };

        let marker = if state.ordered {
            let index = state.next_index;
            state.next_index = state.next_index.saturating_add(1);
            format!("{indent}{index}. ")
        } else {
            format!("{indent}• ")
        };
        let continuation_prefix = " ".repeat(marker.chars().count());

        ItemState {
            first_prefix: marker,
            continuation_prefix,
            needs_first_prefix: true,
        }
    }

    fn trim_blank_lines(&mut self) {
        while matches!(self.lines.last(), Some(line) if line.is_blank()) {
            self.lines.pop();
        }
        while matches!(self.lines.first(), Some(line) if line.is_blank()) {
            self.lines.remove(0);
        }
    }
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn code_block_language(kind: CodeBlockKind<'_>) -> Option<String> {
    match kind {
        CodeBlockKind::Indented => None,
        CodeBlockKind::Fenced(language) => {
            let language = language
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .trim();
            (!language.is_empty()).then(|| language.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn parses_inline_emphasis_and_code() {
        let document = parse_markdown("hello **bold** and `code`");
        assert_eq!(document.lines.len(), 1);
        assert_eq!(document.lines[0].spans.len(), 4);
        assert!(document.lines[0].spans[1].style.strong);
        assert!(document.lines[0].spans[3].style.code);
    }

    #[test]
    fn parses_lists_and_blockquotes() {
        let document = parse_markdown("> quoted\n\n- first\n- second");
        assert_eq!(document.lines[0].prefix, "▎ ");
        assert!(matches!(document.lines[0].kind, MarkdownLineKind::Quote));
        assert!(document.lines.iter().any(|line| line.prefix == "• "));
    }

    #[test]
    fn parses_fenced_code_blocks() {
        let document = parse_markdown("```rust\nfn main() {}\n```\n");
        assert!(matches!(
            document.lines[0].kind,
            MarkdownLineKind::CodeBlock {
                language: Some(ref language)
            } if language == "rust"
        ));
        assert_eq!(document.lines[0].prefix, "│ ");
        assert!(document.lines[0].spans[0].style.code);
    }

    #[test]
    fn preserves_link_destination() {
        let document = parse_markdown("[Rust](https://www.rust-lang.org/)");
        let text = document.lines[0]
            .spans
            .iter()
            .map(|span| span.text.as_str())
            .collect::<String>();
        assert!(text.contains("Rust"));
        assert!(text.contains("https://www.rust-lang.org/"));
    }
}
