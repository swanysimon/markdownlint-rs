use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD029;

impl Rule for MD029 {
    fn name(&self) -> &str {
        "MD029"
    }

    fn description(&self) -> &str {
        "Ordered list item prefix"
    }

    fn tags(&self) -> &[&str] {
        &["ol"]
    }

    fn check(&self, parser: &MarkdownParser, config: Option<&Value>) -> Vec<Violation> {
        let style = config
            .and_then(|c| c.get("style"))
            .and_then(|v| v.as_str())
            .unwrap_or("one_or_ordered");

        let mut violations = Vec::new();
        let mut expected_num = 1;
        let mut in_ordered_list = false;

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim_start();

            // Check if this is an ordered list item
            if let Some(dot_pos) = trimmed.find('.') {
                let prefix = &trimmed[..dot_pos];
                if !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_digit()) {
                    if let Ok(num) = prefix.parse::<usize>() {
                        if !in_ordered_list {
                            in_ordered_list = true;
                            expected_num = 1;
                        }

                        let is_valid = match style {
                            "one" => num == 1,
                            "ordered" => num == expected_num,
                            _ => {
                                // "one_or_ordered" - accept either all 1s OR sequential
                                num == 1 || num == expected_num
                            }
                        };

                        if !is_valid {
                            let should_be = match style {
                                "one" => 1,
                                _ => expected_num,
                            };
                            violations.push(Violation {
                                line: line_number,
                                column: Some(line.len() - trimmed.len() + 1),
                                rule: self.name().to_string(),
                                message: format!(
                                    "Ordered list item prefix: expected {}, found {}",
                                    should_be, num
                                ),
                                fix: None,
                            });
                        }

                        // Always set expected to be the next sequential number after what we saw
                        expected_num = num + 1;
                    }
                } else {
                    in_ordered_list = false;
                }
            } else if !line.trim().is_empty()
                && !trimmed.starts_with("*")
                && !trimmed.starts_with("+")
                && !trimmed.starts_with("-")
            {
                // Non-list line that's not blank
                in_ordered_list = false;
                expected_num = 1;
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
    fn test_ordered_sequence() {
        let content = "1. First\n2. Second\n3. Third";
        let parser = MarkdownParser::new(content);
        let rule = MD029;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_all_ones() {
        let content = "1. First\n1. Second\n1. Third";
        let parser = MarkdownParser::new(content);
        let rule = MD029;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // Default allows "one" style
    }

    #[test]
    fn test_wrong_sequence() {
        let content = "1. First\n3. Third - wrong\n4. Fourth";
        let parser = MarkdownParser::new(content);
        let rule = MD029;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2);
    }

    #[test]
    fn test_enforced_ordered() {
        let content = "1. First\n1. Second - should be 2";
        let parser = MarkdownParser::new(content);
        let rule = MD029;
        let config = serde_json::json!({ "style": "ordered" });
        let violations = rule.check(&parser, Some(&config));

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2);
    }

    #[test]
    fn test_enforced_one() {
        let content = "1. First\n2. Second - should be 1";
        let parser = MarkdownParser::new(content);
        let rule = MD029;
        let config = serde_json::json!({ "style": "one" });
        let violations = rule.check(&parser, Some(&config));

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2);
    }
}
