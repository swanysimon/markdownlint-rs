use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::{Fix, Violation};
use serde_json::Value;

pub struct MD019;

impl Rule for MD019 {
    fn name(&self) -> &str {
        "MD019"
    }

    fn description(&self) -> &str {
        "Multiple spaces after hash on atx style heading"
    }

    fn tags(&self) -> &[&str] {
        &["headings", "headers", "atx", "spaces"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            // Check for ATX heading with multiple spaces after hash
            if trimmed.starts_with('#') {
                // Count leading hashes
                let hash_count = trimmed.chars().take_while(|&c| c == '#').count();

                // Valid heading should have 1-6 hashes
                if hash_count > 0 && hash_count <= 6 && trimmed.len() > hash_count {
                    let after_hashes = &trimmed[hash_count..];

                    // Count spaces after hashes
                    let space_count = after_hashes.chars().take_while(|&c| c == ' ').count();

                    if space_count > 1 {
                        // Replace multiple spaces with single space
                        let hashes = "#".repeat(hash_count);
                        let rest = after_hashes[space_count..].trim_start();
                        let replacement = format!("{} {}", hashes, rest);

                        violations.push(Violation {
                            line: line_number,
                            column: Some(hash_count + 2),
                            rule: self.name().to_string(),
                            message: format!(
                                "Multiple spaces after hash on atx style heading ({} spaces)",
                                space_count
                            ),
                            fix: Some(Fix {
                                line_start: line_number,
                                line_end: line_number,
                                column_start: None,
                                column_end: None,
                                replacement,
                                description: "Replace multiple spaces with single space"
                                    .to_string(),
                            }),
                        });
                    }
                }
            }
        }

        violations
    }

    fn fixable(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_single_space() {
        let content = "# Heading 1\n## Heading 2\n### Heading 3";
        let parser = MarkdownParser::new(content);
        let rule = MD019;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_multiple_spaces() {
        let content = "#  Heading with 2 spaces\n## Correct heading";
        let parser = MarkdownParser::new(content);
        let rule = MD019;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
    }

    #[test]
    fn test_many_spaces() {
        let content = "###     Heading with 5 spaces";
        let parser = MarkdownParser::new(content);
        let rule = MD019;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("5 spaces"));
    }
}
