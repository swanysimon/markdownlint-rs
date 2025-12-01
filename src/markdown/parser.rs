use pulldown_cmark::{Event, Options, Parser, Tag};
use std::ops::Range;

pub struct MarkdownParser<'a> {
    content: &'a str,
    lines: Vec<&'a str>,
}

impl<'a> MarkdownParser<'a> {
    pub fn new(content: &'a str) -> Self {
        let lines = content.lines().collect();
        Self { content, lines }
    }

    pub fn content(&self) -> &'a str {
        self.content
    }

    pub fn lines(&self) -> &[&'a str] {
        &self.lines
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line(&self, line_num: usize) -> Option<&'a str> {
        if line_num > 0 && line_num <= self.lines.len() {
            Some(self.lines[line_num - 1])
        } else {
            None
        }
    }

    pub fn parse(&self) -> impl Iterator<Item = Event<'a>> + 'a {
        Parser::new_ext(self.content, Self::options())
    }

    pub fn parse_with_offsets(&self) -> impl Iterator<Item = (Event<'a>, Range<usize>)> {
        Parser::new_ext(self.content, Self::options()).into_offset_iter()
    }

    fn options() -> Options {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        options
    }

    pub fn offset_to_line(&self, offset: usize) -> usize {
        self.offset_to_position(offset).0
    }

    pub fn offset_to_position(&self, offset: usize) -> (usize, usize) {
        let mut current_offset = 0;
        for (line_num, line) in self.lines.iter().enumerate() {
            let line_len = line.len() + 1;
            if offset < current_offset + line_len {
                let column = offset - current_offset + 1;
                return (line_num + 1, column);
            }
            current_offset += line_len;
        }
        (self.lines.len(), 1)
    }

    pub fn is_heading(&self, event: &Event) -> bool {
        matches!(event, Event::Start(Tag::Heading { .. }))
    }

    pub fn is_code_block(&self, event: &Event) -> bool {
        matches!(event, Event::Start(Tag::CodeBlock(_)))
    }

    pub fn is_list(&self, event: &Event) -> bool {
        matches!(event, Event::Start(Tag::List(_)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let content = "# Heading\n\nSome **bold** text.";
        let parser = MarkdownParser::new(content);

        assert_eq!(parser.content(), content);
        assert_eq!(parser.line_count(), 3);
    }

    #[test]
    fn test_get_line() {
        let content = "Line 1\nLine 2\nLine 3";
        let parser = MarkdownParser::new(content);

        assert_eq!(parser.get_line(1), Some("Line 1"));
        assert_eq!(parser.get_line(2), Some("Line 2"));
        assert_eq!(parser.get_line(3), Some("Line 3"));
        assert_eq!(parser.get_line(0), None);
        assert_eq!(parser.get_line(4), None);
    }

    #[test]
    fn test_offset_to_line() {
        let content = "Line 1\nLine 2\nLine 3";
        let parser = MarkdownParser::new(content);

        assert_eq!(parser.offset_to_line(0), 1);
        assert_eq!(parser.offset_to_line(3), 1);
        assert_eq!(parser.offset_to_line(7), 2);
        assert_eq!(parser.offset_to_line(14), 3);
    }

    #[test]
    fn test_offset_to_position() {
        let content = "Line 1\nLine 2\nLine 3";
        let parser = MarkdownParser::new(content);

        assert_eq!(parser.offset_to_position(0), (1, 1));
        assert_eq!(parser.offset_to_position(3), (1, 4));
        assert_eq!(parser.offset_to_position(7), (2, 1));
    }

    #[test]
    fn test_parse_events() {
        let content = "# Heading";
        let parser = MarkdownParser::new(content);

        let events: Vec<_> = parser.parse().collect();
        assert!(!events.is_empty());
        assert!(parser.is_heading(&events[0]));
    }

    #[test]
    fn test_parse_with_offsets() {
        let content = "# Heading\n\nParagraph";
        let parser = MarkdownParser::new(content);

        let events: Vec<_> = parser.parse_with_offsets().collect();
        assert!(!events.is_empty());
    }

    #[test]
    fn test_event_type_checks() {
        let content = "# Heading\n\n```rust\ncode\n```\n\n- item";
        let parser = MarkdownParser::new(content);

        let events: Vec<_> = parser.parse().collect();

        let has_heading = events.iter().any(|e| parser.is_heading(e));
        let has_code = events.iter().any(|e| parser.is_code_block(e));
        let has_list = events.iter().any(|e| parser.is_list(e));

        assert!(has_heading);
        assert!(has_code);
        assert!(has_list);
    }
}
