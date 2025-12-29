use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD032;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ListMarker {
    Asterisk,
    Plus,
    Dash,
    Ordered,
}

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
        let mut current_marker: Option<ListMarker> = None;
        let mut last_list_line: usize = 0;

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            let list_marker = get_list_marker(trimmed);
            let is_indented = !line.is_empty() && line.chars().next().unwrap().is_whitespace();

            if let Some(marker) = list_marker {
                if !in_list {
                    // Starting a new list
                    in_list = true;
                    current_marker = Some(marker);
                    last_list_line = line_num;

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
                } else if Some(marker) != current_marker {
                    // Different list marker - this is a new list!
                    // The previous list needs a blank line after it (report at previous list line)
                    violations.push(Violation {
                        line: last_list_line + 1,
                        column: Some(1),
                        rule: self.name().to_string(),
                        message: "List should be surrounded by blank lines".to_string(),
                        fix: None,
                    });
                    // Also this new list needs a blank line before it (report at new list line)
                    violations.push(Violation {
                        line: line_num + 1,
                        column: Some(1),
                        rule: self.name().to_string(),
                        message: "List should be surrounded by blank lines".to_string(),
                        fix: None,
                    });
                    current_marker = Some(marker);
                    last_list_line = line_num;
                } else {
                    // Same marker, continue in list
                    last_list_line = line_num;
                }
            } else if in_list && is_indented && !line.trim().is_empty() {
                // Indented non-list line - this is a continuation of the list item
                // Do nothing, stay in list
            } else if in_list && !line.trim().is_empty() {
                // Ending a list (non-blank, non-indented, non-list line)
                in_list = false;
                current_marker = None;

                // Check if next line should have been blank
                violations.push(Violation {
                    line: line_num + 1, // The line after the list
                    column: Some(1),
                    rule: self.name().to_string(),
                    message: "List should be surrounded by blank lines".to_string(),
                    fix: None,
                });
            } else if in_list && line.trim().is_empty() {
                // Blank line during list - might be end
                // Look ahead to see if list continues with same marker
                let mut continues = false;
                for future_line in lines.iter().skip(line_num + 1) {
                    if let Some(future_marker) = get_list_marker(future_line.trim_start()) {
                        if Some(future_marker) == current_marker {
                            continues = true;
                        }
                        break;
                    } else if !future_line.trim().is_empty() {
                        break;
                    }
                }
                if !continues {
                    in_list = false;
                    current_marker = None;
                }
            }
        }

        violations
    }

    fn fixable(&self) -> bool {
        false
    }
}

fn get_list_marker(trimmed: &str) -> Option<ListMarker> {
    // Check for unordered list markers
    if trimmed.starts_with("* ") {
        return Some(ListMarker::Asterisk);
    }
    if trimmed.starts_with("+ ") {
        return Some(ListMarker::Plus);
    }
    if trimmed.starts_with("- ") {
        return Some(ListMarker::Dash);
    }

    // Check for ordered list markers
    if let Some(dot_pos) = trimmed.find(". ") {
        let prefix = &trimmed[..dot_pos];
        if !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_digit()) {
            return Some(ListMarker::Ordered);
        }
    }

    None
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

    #[test]
    fn test_wrapped_list_item() {
        // List items that wrap to multiple lines should not be treated as list ending
        let content = "Text before\n\n* This is a long list item\n  that wraps to the next line\n* Item 2\n\nText after";
        let parser = MarkdownParser::new(content);
        let rule = MD032;
        let violations = rule.check(&parser, None);

        // Should have 0 violations - the wrapped line is a continuation, not a new paragraph
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_multiple_wrapped_lines() {
        // Multiple continuation lines in a single list item
        let content = "Text\n\n* Item with multiple\n  lines of text\n  spanning across\n  multiple lines\n* Item 2\n\nText after";
        let parser = MarkdownParser::new(content);
        let rule = MD032;
        let violations = rule.check(&parser, None);

        // Should have 0 violations
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_wrapped_with_nested_list() {
        // Wrapped items with nested list
        let content =
            "Text\n\n* Item 1 that\n  wraps across lines\n  * Nested item\n* Item 2\n\nText after";
        let parser = MarkdownParser::new(content);
        let rule = MD032;
        let violations = rule.check(&parser, None);

        // Should have 0 violations
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_mixed_markers_are_separate_lists() {
        // Different list markers are treated as separate lists
        let content = "Text\n\n* Item asterisk\n+ Item plus\n- Item dash\n\nText after";
        let parser = MarkdownParser::new(content);
        let rule = MD032;
        let violations = rule.check(&parser, None);

        // Each marker change is a new list needing blank lines
        // + needs blank before/after (2 violations)
        // - needs blank before/after (2 violations)
        assert_eq!(violations.len(), 4);
    }
}
