use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD047;

impl Rule for MD047 {
    fn name(&self) -> &str {
        "MD047"
    }

    fn description(&self) -> &str {
        "Files should end with a single newline character"
    }

    fn tags(&self) -> &[&str] {
        &["blank_lines"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();
        let content = parser.content();

        if content.is_empty() {
            return violations;
        }

        // Check if file ends with a newline
        if !content.ends_with('\n') {
            let lines = parser.lines();
            violations.push(Violation {
                line: lines.len(),
                column: Some(1),
                rule: self.name().to_string(),
                message: "Files should end with a single newline character".to_string(),
                fix: None,
            });
        } else if content.ends_with("\n\n") {
            // Multiple trailing newlines
            let lines = parser.lines();
            violations.push(Violation {
                line: lines.len(),
                column: Some(1),
                rule: self.name().to_string(),
                message: "Files should end with a single newline character".to_string(),
                fix: None,
            });
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
    fn test_single_newline() {
        let content = "# Heading\n\nContent\n";
        let parser = MarkdownParser::new(content);
        let rule = MD047;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_no_newline() {
        let content = "# Heading\n\nContent";
        let parser = MarkdownParser::new(content);
        let rule = MD047;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_multiple_newlines() {
        let content = "# Heading\n\nContent\n\n";
        let parser = MarkdownParser::new(content);
        let rule = MD047;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_empty_file() {
        let content = "";
        let parser = MarkdownParser::new(content);
        let rule = MD047;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // Empty file is OK
    }
}
