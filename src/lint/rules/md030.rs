use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::{Fix, Violation};
use serde_json::Value;

pub struct MD030;

impl Rule for MD030 {
    fn name(&self) -> &str {
        "MD030"
    }

    fn description(&self) -> &str {
        "Spaces after list markers"
    }

    fn tags(&self) -> &[&str] {
        &["ol", "ul", "whitespace"]
    }

    fn check(&self, parser: &MarkdownParser, config: Option<&Value>) -> Vec<Violation> {
        let ul_single = config
            .and_then(|c| c.get("ul_single"))
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;

        let _ul_multi = config
            .and_then(|c| c.get("ul_multi"))
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;

        let ol_single = config
            .and_then(|c| c.get("ol_single"))
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;

        let _ol_multi = config
            .and_then(|c| c.get("ol_multi"))
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;

        let mut violations = Vec::new();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim_start();

            // Check unordered list markers
            if trimmed.starts_with('*') || trimmed.starts_with('+') || trimmed.starts_with('-') {
                let after_marker = &trimmed[1..];
                let space_count = after_marker.chars().take_while(|&c| c == ' ').count();

                // Only check if there's content after the marker (not just a marker alone)
                if !after_marker.trim().is_empty() {
                    // For now, assume single-line (could be enhanced to detect multi-line)
                    let expected = ul_single;

                    if space_count != expected {
                        // Fix the spacing after list marker
                        let leading_spaces = &line[..line.len() - trimmed.len()];
                        let marker = trimmed.chars().next().unwrap();
                        let content = after_marker[space_count..].trim_start();
                        let spaces = " ".repeat(expected);
                        let replacement =
                            format!("{}{}{}{}", leading_spaces, marker, spaces, content);

                        violations.push(Violation {
                            line: line_number,
                            column: Some(line.len() - trimmed.len() + 2),
                            rule: self.name().to_string(),
                            message: format!(
                                "Expected {} space(s) after list marker, found {}",
                                expected, space_count
                            ),
                            fix: Some(Fix {
                                line_start: line_number,
                                line_end: line_number,
                                column_start: None,
                                column_end: None,
                                replacement,
                                description: format!("Adjust spacing to {} space(s)", expected),
                            }),
                        });
                    }
                }
            }

            // Check ordered list markers
            if let Some(dot_pos) = trimmed.find('.') {
                let prefix = &trimmed[..dot_pos];
                if prefix.chars().all(|c| c.is_ascii_digit()) && !prefix.is_empty() {
                    let after_dot = &trimmed[dot_pos + 1..];

                    // Only check if there's content after the marker
                    if !after_dot.trim().is_empty() {
                        let space_count = after_dot.chars().take_while(|&c| c == ' ').count();

                        // For now, assume single-line
                        let expected = ol_single;

                        if space_count != expected {
                            // Fix the spacing after list marker
                            let leading_spaces = &line[..line.len() - trimmed.len()];
                            let marker = &trimmed[..=dot_pos];
                            let content = after_dot[space_count..].trim_start();
                            let spaces = " ".repeat(expected);
                            let replacement =
                                format!("{}{}{}{}", leading_spaces, marker, spaces, content);

                            violations.push(Violation {
                                line: line_number,
                                column: Some(line.len() - trimmed.len() + dot_pos + 2),
                                rule: self.name().to_string(),
                                message: format!(
                                    "Expected {} space(s) after list marker, found {}",
                                    expected, space_count
                                ),
                                fix: Some(Fix {
                                    line_start: line_number,
                                    line_end: line_number,
                                    column_start: None,
                                    column_end: None,
                                    replacement,
                                    description: format!("Adjust spacing to {} space(s)", expected),
                                }),
                            });
                        }
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
    fn test_correct_spacing() {
        let content = "* Item 1\n+ Item 2\n- Item 3\n1. Ordered";
        let parser = MarkdownParser::new(content);
        let rule = MD030;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_no_space() {
        let content = "*Item without space";
        let parser = MarkdownParser::new(content);
        let rule = MD030;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("found 0"));
    }

    #[test]
    fn test_multiple_spaces() {
        let content = "*  Item with 2 spaces";
        let parser = MarkdownParser::new(content);
        let rule = MD030;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("found 2"));
    }

    #[test]
    fn test_custom_spacing() {
        let content = "*  Item with 2 spaces";
        let parser = MarkdownParser::new(content);
        let rule = MD030;
        let config = serde_json::json!({ "ul_single": 2 });
        let violations = rule.check(&parser, Some(&config));

        assert_eq!(violations.len(), 0); // 2 spaces now expected
    }
}
