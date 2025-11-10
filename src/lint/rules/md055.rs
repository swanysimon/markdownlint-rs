use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD055;

impl Rule for MD055 {
    fn name(&self) -> &str {
        "MD055"
    }

    fn description(&self) -> &str {
        "Table pipe style"
    }

    fn tags(&self) -> &[&str] {
        &["table"]
    }

    fn check(&self, parser: &MarkdownParser, config: Option<&Value>) -> Vec<Violation> {
        let style = config
            .and_then(|c| c.get("style"))
            .and_then(|v| v.as_str())
            .unwrap_or("consistent");

        let mut violations = Vec::new();
        let mut first_style: Option<&str> = None;

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            // Check if line is a table row (contains pipes)
            if !line.contains('|') {
                continue;
            }

            let trimmed = line.trim();

            // Skip separator lines (e.g., |-----|-----|)
            if trimmed.contains("---") || trimmed.contains(":--") || trimmed.contains("--:") {
                continue;
            }

            // Determine the style of this line
            let has_leading = trimmed.starts_with('|');
            let has_trailing = trimmed.ends_with('|');

            let current_style = match (has_leading, has_trailing) {
                (true, true) => "leading_and_trailing",
                (true, false) => "leading_only",
                (false, true) => "trailing_only",
                (false, false) => "no_leading_or_trailing",
            };

            if style == "consistent" {
                if let Some(first) = first_style {
                    if current_style != first {
                        violations.push(Violation {
                            line: line_number,
                            column: Some(1),
                            rule: self.name().to_string(),
                            message: format!(
                                "Table pipe style should be consistent: expected '{}', found '{}'",
                                first, current_style
                            ),
                            fix: None,
                        });
                    }
                } else {
                    first_style = Some(current_style);
                }
            } else if style == "leading_and_trailing" && current_style != "leading_and_trailing" {
                violations.push(Violation {
                    line: line_number,
                    column: Some(1),
                    rule: self.name().to_string(),
                    message: "Table should have leading and trailing pipes".to_string(),
                    fix: None,
                });
            } else if style == "no_leading_or_trailing" && (has_leading || has_trailing) {
                violations.push(Violation {
                    line: line_number,
                    column: Some(1),
                    rule: self.name().to_string(),
                    message: "Table should not have leading or trailing pipes".to_string(),
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
    fn test_consistent_with_pipes() {
        let content = "| Col1 | Col2 |\n|------|------|\n| A    | B    |";
        let parser = MarkdownParser::new(content);
        let rule = MD055;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_consistent_without_pipes() {
        let content = "Col1 | Col2\n-----|-----\nA    | B";
        let parser = MarkdownParser::new(content);
        let rule = MD055;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_inconsistent_pipes() {
        let content = "| Col1 | Col2 |\n|------|------|\nA    | B";
        let parser = MarkdownParser::new(content);
        let rule = MD055;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_enforced_leading_and_trailing() {
        let content = "Col1 | Col2\n-----|-----\nA | B";
        let parser = MarkdownParser::new(content);
        let rule = MD055;
        let config = serde_json::json!({ "style": "leading_and_trailing" });
        let violations = rule.check(&parser, Some(&config));

        assert_eq!(violations.len(), 2); // Header and data row
    }

    #[test]
    fn test_simple_table() {
        let content = "| Header |\n| ------ |\n| Cell   |";
        let parser = MarkdownParser::new(content);
        let rule = MD055;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }
}
