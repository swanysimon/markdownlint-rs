use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use pulldown_cmark::{Event, Tag};
use serde_json::Value;

pub struct MD042;

impl Rule for MD042 {
    fn name(&self) -> &str {
        "MD042"
    }

    fn description(&self) -> &str {
        "No empty links"
    }

    fn tags(&self) -> &[&str] {
        &["links"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();
        let mut in_link = false;
        let mut link_start_line = 0;
        let mut link_text = String::new();

        for (event, range) in parser.parse_with_offsets() {
            match event {
                Event::Start(Tag::Link { .. }) => {
                    in_link = true;
                    link_start_line = parser.offset_to_line(range.start);
                    link_text.clear();
                }
                Event::Text(text) if in_link => {
                    link_text.push_str(&text);
                }
                Event::End(Tag::Link { .. }) if in_link => {
                    if link_text.trim().is_empty() {
                        violations.push(Violation {
                            line: link_start_line,
                            column: Some(1),
                            rule: self.name().to_string(),
                            message: "No empty links".to_string(),
                            fix: None,
                        });
                    }
                    in_link = false;
                }
                _ => {}
            }
        }

        violations
    }

    fn fixable(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_with_text() {
        let content = "[Link text](https://example.com)";
        let parser = MarkdownParser::new(content);
        let rule = MD042;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_empty_link() {
        let content = "[](https://example.com)";
        let parser = MarkdownParser::new(content);
        let rule = MD042;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_whitespace_only_link() {
        let content = "[  ](https://example.com)";
        let parser = MarkdownParser::new(content);
        let rule = MD042;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_multiple_links() {
        let content = "[Good](url1) and [](url2) and [Also good](url3)";
        let parser = MarkdownParser::new(content);
        let rule = MD042;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1); // Only second link is empty
    }
}
