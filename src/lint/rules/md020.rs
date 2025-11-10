use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD020;

impl Rule for MD020 {
    fn name(&self) -> &str {
        "MD020"
    }

    fn description(&self) -> &str {
        "No space inside hashes on closed atx style heading"
    }

    fn tags(&self) -> &[&str] {
        &["headings", "atx_closed", "spaces"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            // Check if this is a closed ATX heading (starts and ends with #)
            if trimmed.starts_with('#') && trimmed.ends_with('#') {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    // Check if last part is all hashes
                    if parts.last().unwrap().chars().all(|c| c == '#') {
                        let closing_hashes = parts.last().unwrap();

                        // Find position of closing hashes in original line
                        if let Some(pos) = trimmed.rfind(closing_hashes) {
                            // Check if there's a space before the closing hashes
                            if pos > 0 && trimmed.chars().nth(pos - 1) == Some(' ') {
                                violations.push(Violation {
                                    line: line_number,
                                    column: Some(1),
                                    rule: self.name().to_string(),
                                    message: "No space inside hashes on closed atx style heading".to_string(),
                                    fix: None,
                                });
                            }
                        }
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
    fn test_correct_closed_heading() {
        let content = "# Heading#";
        let parser = MarkdownParser::new(content);
        let rule = MD020;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_space_before_closing() {
        let content = "# Heading #";
        let parser = MarkdownParser::new(content);
        let rule = MD020;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_regular_heading() {
        let content = "# Heading";
        let parser = MarkdownParser::new(content);
        let rule = MD020;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // Not a closed heading
    }

    #[test]
    fn test_multiple_levels() {
        let content = "## Heading ##\n### Another ###";
        let parser = MarkdownParser::new(content);
        let rule = MD020;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 2);
    }
}
