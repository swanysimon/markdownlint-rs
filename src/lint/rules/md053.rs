use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use regex::Regex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

pub struct MD053;

impl Rule for MD053 {
    fn name(&self) -> &str {
        "MD053"
    }

    fn description(&self) -> &str {
        "Link and image reference definitions should be needed"
    }

    fn tags(&self) -> &[&str] {
        &["links"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        // First pass: collect all defined reference labels with their line numbers
        let mut defined_labels: HashMap<String, usize> = HashMap::new();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            if trimmed.starts_with('[')
                && let Some(end_bracket) = trimmed.find("]:")
            {
                let label = &trimmed[1..end_bracket];
                defined_labels.insert(label.to_lowercase(), line_number);
            }
        }

        // Second pass: find reference-style links and images in raw text
        // Pattern: [text][label] or ![alt][label]
        let mut used_labels: HashSet<String> = HashSet::new();
        let regex_link = Regex::new(r"!?\[([^\]]+)\]\[([^\]]+)\]").unwrap();

        for line in parser.lines() {
            for cap in regex_link.captures_iter(line) {
                let label = cap.get(2).unwrap().as_str().to_lowercase();
                used_labels.insert(label);
            }
        }

        // Find unused definitions
        for (label, line_number) in defined_labels {
            if !used_labels.contains(&label) {
                violations.push(Violation {
                    line: line_number,
                    column: Some(1),
                    rule: self.name().to_string(),
                    message: format!(
                        "Link reference definition '{}' is defined but not used",
                        label
                    ),
                    fix: None,
                });
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
    fn test_used_definition() {
        let content = "[example]: https://example.com\n\n[Link][example]";
        let parser = MarkdownParser::new(content);
        let rule = MD053;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_unused_definition() {
        let content = "[unused]: https://example.com\n\nSome text without links.";
        let parser = MarkdownParser::new(content);
        let rule = MD053;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("unused"));
    }

    #[test]
    fn test_multiple_definitions() {
        let content = "[used]: https://example.com\n[unused]: https://other.com\n\n[Link][used]";
        let parser = MarkdownParser::new(content);
        let rule = MD053;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("unused"));
    }

    #[test]
    fn test_image_reference() {
        let content = "[img]: image.png\n\n![Alt][img]";
        let parser = MarkdownParser::new(content);
        let rule = MD053;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_all_used() {
        let content = "[link1]: url1\n[link2]: url2\n\n[A][link1] [B][link2]";
        let parser = MarkdownParser::new(content);
        let rule = MD053;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }
}
