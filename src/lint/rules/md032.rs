use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD032;

impl Rule for MD032 {
    fn name(&self) -> &str {
        "MD032"
    }

    fn description(&self) -> &str {
        "Lists should be surrounded by blank lines"
    }

    fn tags(&self) -> &[&str] {
        &["bullet", "ul", "ol", "blank_lines"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();
        let lines = parser.lines();
        let mut in_list = false;
        let mut _list_end_line = 0;

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            let is_list_item = is_list_line(trimmed);

            if is_list_item && !in_list {
                // Starting a list
                in_list = true;

                // Check if previous line is blank (unless it's the first line)
                if line_num > 0 {
                    let prev_line = &lines[line_num - 1];
                    if !prev_line.trim().is_empty() {
                        violations.push(Violation {
                            line: line_num + 1,
                            column: Some(1),
                            rule: self.name().to_string(),
                            message: "List should be surrounded by blank lines".to_string(),
                            fix: None,
                        });
                    }
                }
            } else if !is_list_item && in_list && !line.trim().is_empty() {
                // Ending a list (non-blank, non-list line)
                in_list = false;
                _list_end_line = line_num - 1;

                // Check if next line should have been blank
                violations.push(Violation {
                    line: line_num + 1, // The line after the list
                    column: Some(1),
                    rule: self.name().to_string(),
                    message: "List should be surrounded by blank lines".to_string(),
                    fix: None,
                });
            } else if !is_list_item && in_list && line.trim().is_empty() {
                // Blank line during list - might be end
                // Look ahead to see if list continues
                let mut continues = false;
                for future_line in lines.iter().skip(line_num + 1) {
                    if is_list_line(future_line.trim_start()) {
                        continues = true;
                        break;
                    } else if !future_line.trim().is_empty() {
                        break;
                    }
                }
                if !continues {
                    in_list = false;
                }
            }
        }

        violations
    }

    fn fixable(&self) -> bool {
        false
    }
}

fn is_list_line(trimmed: &str) -> bool {
    // Check for unordered list markers
    if trimmed.starts_with("* ") || trimmed.starts_with("+ ") || trimmed.starts_with("- ") {
        return true;
    }

    // Check for ordered list markers
    if let Some(dot_pos) = trimmed.find(". ") {
        let prefix = &trimmed[..dot_pos];
        if !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properly_surrounded() {
        let content = "Text before\n\n* Item 1\n* Item 2\n\nText after";
        let parser = MarkdownParser::new(content);
        let rule = MD032;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_missing_blank_before() {
        let content = "Text before\n* Item 1\n* Item 2\n\nText after";
        let parser = MarkdownParser::new(content);
        let rule = MD032;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2); // List starts on line 2
    }

    #[test]
    fn test_missing_blank_after() {
        let content = "Text before\n\n* Item 1\n* Item 2\nText after";
        let parser = MarkdownParser::new(content);
        let rule = MD032;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 5); // Text after list
    }

    #[test]
    fn test_first_line() {
        let content = "* Item 1\n* Item 2\n\nText after";
        let parser = MarkdownParser::new(content);
        let rule = MD032;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // First line is OK
    }
}
