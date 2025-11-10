use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD049;

impl Rule for MD049 {
    fn name(&self) -> &str {
        "MD049"
    }

    fn description(&self) -> &str {
        "Emphasis style should be consistent"
    }

    fn tags(&self) -> &[&str] {
        &["emphasis"]
    }

    fn check(&self, parser: &MarkdownParser, config: Option<&Value>) -> Vec<Violation> {
        let style = config
            .and_then(|c| c.get("style"))
            .and_then(|v| v.as_str())
            .unwrap_or("consistent");

        let mut violations = Vec::new();
        let mut first_style: Option<char> = None;

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            // Look for emphasis patterns: *text* or _text_ (not ** or __)
            let mut chars: Vec<char> = line.chars().collect();
            let mut i = 0;

            while i < chars.len() {
                let ch = chars[i];

                // Check for single * or _ (emphasis, not strong)
                if (ch == '*' || ch == '_') && i + 1 < chars.len() {
                    // Make sure it's not strong (**  or __)
                    let is_strong = (i + 1 < chars.len() && chars[i + 1] == ch)
                        || (i > 0 && chars[i - 1] == ch);

                    if !is_strong {
                        // Find closing marker
                        let mut found_close = false;
                        for j in (i + 1)..chars.len() {
                            if chars[j] == ch {
                                // Make sure closing is also not strong
                                let close_is_strong = (j + 1 < chars.len() && chars[j + 1] == ch)
                                    || (j > 0 && chars[j - 1] == ch);

                                if !close_is_strong {
                                    found_close = true;

                                    // Track style
                                    if style == "consistent" {
                                        if let Some(first) = first_style {
                                            if ch != first {
                                                violations.push(Violation {
                                                    line: line_number,
                                                    column: Some(i + 1),
                                                    rule: self.name().to_string(),
                                                    message: format!(
                                                        "Emphasis style should be consistent: expected '{}', found '{}'",
                                                        first, ch
                                                    ),
                                                    fix: None,
                                                });
                                            }
                                        } else {
                                            first_style = Some(ch);
                                        }
                                    } else {
                                        let expected = if style == "asterisk" { '*' } else { '_' };
                                        if ch != expected {
                                            violations.push(Violation {
                                                line: line_number,
                                                column: Some(i + 1),
                                                rule: self.name().to_string(),
                                                message: format!(
                                                    "Emphasis style should be '{}', found '{}'",
                                                    expected, ch
                                                ),
                                                fix: None,
                                            });
                                        }
                                    }

                                    i = j; // Skip to after closing
                                    break;
                                }
                            }
                        }
                    }
                }

                i += 1;
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
        let content = "This is *italic* and *more italic*.";
        let parser = MarkdownParser::new(content);
        let rule = MD049;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_consistent_underscore() {
        let content = "This is _italic_ and _more italic_.";
        let parser = MarkdownParser::new(content);
        let rule = MD049;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_inconsistent() {
        let content = "This is *italic* and _also italic_.";
        let parser = MarkdownParser::new(content);
        let rule = MD049;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_enforced_style() {
        let content = "This is _italic_ text.";
        let parser = MarkdownParser::new(content);
        let rule = MD049;
        let config = serde_json::json!({ "style": "asterisk" });
        let violations = rule.check(&parser, Some(&config));

        assert_eq!(violations.len(), 1);
    }
}
