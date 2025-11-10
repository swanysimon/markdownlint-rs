use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD006;

impl Rule for MD006 {
    fn name(&self) -> &str {
        "MD006"
    }

    fn description(&self) -> &str {
        "Consider starting bulleted lists at the beginning of the line"
    }

    fn tags(&self) -> &[&str] {
        &["bullet", "ul", "indentation"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            // Check if line starts with spaces followed by bullet marker
            if line.starts_with(' ') {
                let trimmed = line.trim_start();
                // Check for unordered list markers
                if trimmed.starts_with("* ")
                    || trimmed.starts_with("+ ")
                    || trimmed.starts_with("- ")
                {
                    violations.push(Violation {
                        line: line_number,
                        column: Some(1),
                        rule: self.name().to_string(),
                        message: "Consider starting bulleted lists at the beginning of the line"
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
    fn test_list_at_start() {
        let content = "* Item 1\n* Item 2";
        let parser = MarkdownParser::new(content);
        let rule = MD006;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_indented_list() {
        let content = "  * Item 1\n  * Item 2";
        let parser = MarkdownParser::new(content);
        let rule = MD006;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_nested_list() {
        let content = "* Item 1\n  * Nested item";
        let parser = MarkdownParser::new(content);
        let rule = MD006;
        let violations = rule.check(&parser, None);

        // Nested items are expected to be indented
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_mixed() {
        let content = "* Good\n  * Nested (violation)\n+ Also good";
        let parser = MarkdownParser::new(content);
        let rule = MD006;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }
}
