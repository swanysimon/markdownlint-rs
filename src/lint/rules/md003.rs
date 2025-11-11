use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD003;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeadingStyle {
    Atx,       // # Heading
    AtxClosed, // # Heading #
    Setext,    // Heading\n======
}

impl Rule for MD003 {
    fn name(&self) -> &str {
        "MD003"
    }

    fn description(&self) -> &str {
        "Heading style should be consistent throughout the document"
    }

    fn tags(&self) -> &[&str] {
        &["headings", "headers"]
    }

    fn check(&self, parser: &MarkdownParser, config: Option<&Value>) -> Vec<Violation> {
        let style_config = config.and_then(|c| c.get("style")).and_then(|v| v.as_str());

        let mut violations = Vec::new();
        let mut first_style: Option<HeadingStyle> = None;

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            // Detect heading style
            let current_style = if trimmed.starts_with('#') {
                // ATX or ATX_CLOSED
                // ATX_CLOSED: should have text, then space(s), then hash(es)
                // e.g., "## Heading 2 ##"
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 && parts.last().unwrap().chars().all(|c| c == '#') {
                    Some(HeadingStyle::AtxClosed)
                } else {
                    Some(HeadingStyle::Atx)
                }
            } else if line_num + 1 < parser.lines().len() {
                // Check for setext (next line is === or ---)
                let next_line = parser.lines()[line_num + 1];
                let is_setext_underline =
                    (next_line.chars().all(|c| c == '=' || c.is_whitespace())
                        && next_line.contains('='))
                        || (next_line.chars().all(|c| c == '-' || c.is_whitespace())
                            && next_line.contains('-')
                            && next_line.trim().len() >= 3);

                if is_setext_underline {
                    Some(HeadingStyle::Setext)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(style) = current_style {
                // If config specifies a style, check against it
                if let Some(required) = style_config {
                    let required_style = match required {
                        "atx" => HeadingStyle::Atx,
                        "atx_closed" => HeadingStyle::AtxClosed,
                        "setext" => HeadingStyle::Setext,
                        _ => continue,
                    };

                    if style != required_style {
                        violations.push(Violation {
                            line: line_number,
                            column: Some(1),
                            rule: self.name().to_string(),
                            message: format!(
                                "Heading style should be {:?} but found {:?}",
                                required_style, style
                            ),
                            fix: None,
                        });
                    }
                } else {
                    // No config: ensure consistency
                    if let Some(first) = first_style {
                        if style != first {
                            violations.push(Violation {
                                line: line_number,
                                column: Some(1),
                                rule: self.name().to_string(),
                                message: format!(
                                    "Heading style should be consistent (expected {:?}, found {:?})",
                                    first, style
                                ),
                                fix: None,
                            });
                        }
                    } else {
                        first_style = Some(style);
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
    fn test_consistent_atx() {
        let content = "# Heading 1\n## Heading 2\n### Heading 3";
        let parser = MarkdownParser::new(content);
        let rule = MD003;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_inconsistent_styles() {
        let content = "# Heading 1\n## Heading 2 ##\n### Heading 3";
        let parser = MarkdownParser::new(content);
        let rule = MD003;
        let violations = rule.check(&parser, None);

        assert!(!violations.is_empty());
    }

    #[test]
    fn test_enforced_atx_style() {
        let content = "# Heading 1\n## Heading 2 ##";
        let parser = MarkdownParser::new(content);
        let rule = MD003;
        let config = serde_json::json!({ "style": "atx" });
        let violations = rule.check(&parser, Some(&config));

        assert_eq!(violations.len(), 1); // Second heading has closing #
    }

    #[test]
    fn test_setext_detection() {
        let content = "Heading 1\n=========\n\nHeading 2\n---------";
        let parser = MarkdownParser::new(content);
        let rule = MD003;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // Both setext style
    }
}
