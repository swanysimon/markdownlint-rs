use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use pulldown_cmark::{Event, Tag};
use serde_json::Value;

pub struct MD022;

impl Rule for MD022 {
    fn name(&self) -> &str {
        "MD022"
    }

    fn description(&self) -> &str {
        "Headings should be surrounded by blank lines"
    }

    fn tags(&self) -> &[&str] {
        &["headings", "headers", "blank_lines"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();
        let lines = parser.lines();

        // Find all heading lines using the AST
        let mut heading_lines = Vec::new();
        for (event, range) in parser.parse_with_offsets() {
            if let Event::Start(Tag::Heading(_, _, _)) = event {
                let line = parser.offset_to_line(range.start);
                heading_lines.push(line);
            }
        }

        for &heading_line in &heading_lines {
            let line_idx = heading_line - 1;

            // Check if there's a blank line before (skip if first line or after blank)
            if line_idx > 0 {
                let prev_line = lines[line_idx - 1].trim();
                if !prev_line.is_empty() {
                    violations.push(Violation {
                        line: heading_line,
                        column: Some(1),
                        rule: self.name().to_string(),
                        message: "Heading should be surrounded by blank lines (missing before)"
                            .to_string(),
                        fix: None,
                    });
                }
            }

            // Check if there's a blank line after (skip if last line)
            if line_idx + 1 < lines.len() {
                let next_line = lines[line_idx + 1].trim();
                // Allow another heading right after (for closed headings or setext underlines)
                if !next_line.is_empty()
                    && !next_line.starts_with('#')
                    && !next_line.chars().all(|c| c == '=' || c == '-' || c.is_whitespace())
                {
                    violations.push(Violation {
                        line: heading_line,
                        column: Some(1),
                        rule: self.name().to_string(),
                        message: "Heading should be surrounded by blank lines (missing after)"
                            .to_string(),
                        fix: None,
                    });
                }
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
    fn test_properly_surrounded() {
        let content = "Paragraph\n\n# Heading\n\nAnother paragraph";
        let parser = MarkdownParser::new(content);
        let rule = MD022;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_missing_blank_before() {
        let content = "Paragraph\n# Heading\n\nContent";
        let parser = MarkdownParser::new(content);
        let rule = MD022;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("before"));
    }

    #[test]
    fn test_missing_blank_after() {
        let content = "\n# Heading\nContent";
        let parser = MarkdownParser::new(content);
        let rule = MD022;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("after"));
    }

    #[test]
    fn test_first_line() {
        let content = "# Heading\n\nContent";
        let parser = MarkdownParser::new(content);
        let rule = MD022;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // First line is exempt from "before" check
    }
}
