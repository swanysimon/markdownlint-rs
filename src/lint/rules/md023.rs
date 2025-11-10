use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD023;

impl Rule for MD023 {
    fn name(&self) -> &str {
        "MD023"
    }

    fn description(&self) -> &str {
        "Headings must start at the beginning of the line"
    }

    fn tags(&self) -> &[&str] {
        &["headings", "headers", "spaces"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            // Check if line starts with whitespace followed by hash
            if line.starts_with(' ') || line.starts_with('\t') {
                let trimmed = line.trim_start();
                if trimmed.starts_with('#') {
                    // Count leading hashes to verify it's a heading
                    let hash_count = trimmed.chars().take_while(|&c| c == '#').count();

                    // Valid heading should have 1-6 hashes
                    if hash_count > 0 && hash_count <= 6 {
                        let indent = line.len() - trimmed.len();
                        violations.push(Violation {
                            line: line_number,
                            column: Some(1),
                            rule: self.name().to_string(),
                            message: format!(
                                "Heading must start at the beginning of the line ({} space(s) before)",
                                indent
                            ),
                            fix: None,
                        });
                    }
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
    fn test_correct_headings() {
        let content = "# Heading 1\n## Heading 2\n### Heading 3";
        let parser = MarkdownParser::new(content);
        let rule = MD023;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_indented_heading() {
        let content = " # Heading with space\n## Correct heading";
        let parser = MarkdownParser::new(content);
        let rule = MD023;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
    }

    #[test]
    fn test_tab_indented() {
        let content = "\t# Heading with tab";
        let parser = MarkdownParser::new(content);
        let rule = MD023;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_multiple_spaces() {
        let content = "    # Heading with 4 spaces";
        let parser = MarkdownParser::new(content);
        let rule = MD023;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("4 space"));
    }
}
