use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;

pub struct MD052;

impl Rule for MD052 {
    fn name(&self) -> &str {
        "MD052"
    }

    fn description(&self) -> &str {
        "Reference links and images should use a label that is defined"
    }

    fn tags(&self) -> &[&str] {
        &["links"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        // First pass: collect all defined reference labels
        let mut defined_labels: HashSet<String> = HashSet::new();

        for line in parser.lines() {
            // Match reference definitions: [label]: url
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                if let Some(end_bracket) = trimmed.find("]:") {
                    let label = &trimmed[1..end_bracket];
                    defined_labels.insert(label.to_lowercase());
                }
            }
        }

        // Second pass: find reference-style links and images in raw text
        // Pattern: [text][label] or ![alt][label]
        let regex_link = Regex::new(r"!?\[([^\]]+)\]\[([^\]]+)\]").unwrap();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            for cap in regex_link.captures_iter(line) {
                let label = cap.get(2).unwrap().as_str().to_lowercase();

                if !defined_labels.contains(&label) {
                    let is_image = cap.get(0).unwrap().as_str().starts_with('!');
                    let item_type = if is_image { "image" } else { "link" };

                    violations.push(Violation {
                        line: line_number,
                        column: Some(1),
                        rule: self.name().to_string(),
                        message: format!("Reference {} label '{}' is not defined", item_type, cap.get(2).unwrap().as_str()),
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
    fn test_defined_reference() {
        let content = "[example]: https://example.com\n\n[Link][example]";
        let parser = MarkdownParser::new(content);
        let rule = MD052;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_undefined_reference() {
        let content = "[Link][undefined]";
        let parser = MarkdownParser::new(content);
        let rule = MD052;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("undefined"));
    }

    #[test]
    fn test_image_reference() {
        let content = "[img]: image.png\n\n![Alt][img]";
        let parser = MarkdownParser::new(content);
        let rule = MD052;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_inline_links_ignored() {
        let content = "[Link](https://example.com)";
        let parser = MarkdownParser::new(content);
        let rule = MD052;
        let violations = rule.check(&parser, None);

        // Inline links should not trigger violations
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_case_insensitive() {
        let content = "[EXAMPLE]: https://example.com\n\n[Link][example]";
        let parser = MarkdownParser::new(content);
        let rule = MD052;
        let violations = rule.check(&parser, None);

        // Labels should be case-insensitive
        assert_eq!(violations.len(), 0);
    }
}
