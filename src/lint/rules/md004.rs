use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD004;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListMarker {
    Asterisk, // *
    Plus,     // +
    Dash,     // -
}

impl Rule for MD004 {
    fn name(&self) -> &str {
        "MD004"
    }

    fn description(&self) -> &str {
        "Unordered list style should be consistent"
    }

    fn tags(&self) -> &[&str] {
        &["bullet", "ul"]
    }

    fn check(&self, parser: &MarkdownParser, config: Option<&Value>) -> Vec<Violation> {
        let style_config = config.and_then(|c| c.get("style")).and_then(|v| v.as_str());

        let mut violations = Vec::new();
        let mut first_marker: Option<ListMarker> = None;

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim_start();

            // Detect unordered list marker
            let marker = if trimmed.starts_with("* ") {
                Some(ListMarker::Asterisk)
            } else if trimmed.starts_with("+ ") {
                Some(ListMarker::Plus)
            } else if trimmed.starts_with("- ") {
                Some(ListMarker::Dash)
            } else {
                None
            };

            if let Some(current_marker) = marker {
                // If config specifies a style, check against it
                if let Some(required) = style_config {
                    let required_marker = match required {
                        "asterisk" => ListMarker::Asterisk,
                        "plus" => ListMarker::Plus,
                        "dash" => ListMarker::Dash,
                        _ => continue,
                    };

                    if current_marker != required_marker {
                        violations.push(Violation {
                            line: line_number,
                            column: Some(line.len() - trimmed.len() + 1),
                            rule: self.name().to_string(),
                            message: format!("List marker style should be {:?}", required_marker),
                            fix: None,
                        });
                    }
                } else {
                    // No config: ensure consistency
                    if let Some(first) = first_marker {
                        if current_marker != first {
                            violations.push(Violation {
                                line: line_number,
                                column: Some(line.len() - trimmed.len() + 1),
                                rule: self.name().to_string(),
                                message: format!(
                                    "List marker style should be consistent (expected {:?}, found {:?})",
                                    first, current_marker
                                ),
                                fix: None,
                            });
                        }
                    } else {
                        first_marker = Some(current_marker);
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
    fn test_consistent_asterisk() {
        let content = "* Item 1\n* Item 2\n* Item 3";
        let parser = MarkdownParser::new(content);
        let rule = MD004;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_inconsistent_markers() {
        let content = "* Item 1\n+ Item 2\n- Item 3";
        let parser = MarkdownParser::new(content);
        let rule = MD004;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 2); // Second and third items differ from first
    }

    #[test]
    fn test_enforced_dash_style() {
        let content = "* Item 1\n- Item 2";
        let parser = MarkdownParser::new(content);
        let rule = MD004;
        let config = serde_json::json!({ "style": "dash" });
        let violations = rule.check(&parser, Some(&config));

        assert_eq!(violations.len(), 1); // First item uses asterisk
    }

    #[test]
    fn test_nested_lists() {
        let content = "* Item 1\n  * Nested 1\n  * Nested 2\n* Item 2";
        let parser = MarkdownParser::new(content);
        let rule = MD004;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // All use asterisk
    }
}
