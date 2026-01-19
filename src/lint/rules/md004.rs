use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use pulldown_cmark::{Event, Tag};
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

        // Track code blocks to exclude them from checking
        let mut code_block_lines = std::collections::HashSet::new();
        let mut in_code_block = false;

        for (event, range) in parser.parse_with_offsets() {
            match event {
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                }
                Event::End(Tag::CodeBlock(_)) => {
                    in_code_block = false;
                }
                Event::Text(_) if in_code_block => {
                    // Mark all lines that this text event spans
                    let start_line = parser.offset_to_line(range.start);
                    let end_line = parser.offset_to_line(range.end.saturating_sub(1));
                    for line in start_line..=end_line {
                        code_block_lines.insert(line);
                    }
                }
                _ => {}
            }
        }

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            // Skip code blocks
            if code_block_lines.contains(&line_number) {
                continue;
            }

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

    #[test]
    fn test_code_block_line_detection() {
        // Verify that all lines in a code block are detected
        let content = "```\ncode line 1\ncode line 2\ncode line 3\n```\n";
        let parser = MarkdownParser::new(content);

        let mut code_block_lines = std::collections::HashSet::new();
        let mut in_code_block = false;

        for (event, range) in parser.parse_with_offsets() {
            match event {
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                }
                Event::End(Tag::CodeBlock(_)) => {
                    in_code_block = false;
                }
                Event::Text(_) if in_code_block => {
                    // Mark all lines that this text event spans
                    let start_line = parser.offset_to_line(range.start);
                    let end_line = parser.offset_to_line(range.end.saturating_sub(1));
                    for line in start_line..=end_line {
                        code_block_lines.insert(line);
                    }
                }
                _ => {}
            }
        }

        // All code lines should be marked (lines 2, 3, 4)
        assert!(
            code_block_lines.contains(&2),
            "Line 2 (code line 1) should be in code block"
        );
        assert!(
            code_block_lines.contains(&3),
            "Line 3 (code line 2) should be in code block"
        );
        assert!(
            code_block_lines.contains(&4),
            "Line 4 (code line 3) should be in code block"
        );
    }

    #[test]
    fn test_markdown_syntax_in_code_block() {
        let content = r#"# My Document

Here's a code block with markdown syntax:

```
- This looks like a list item
* This also looks like a list item
+ And this one too
```

* Real list item
"#;
        let parser = MarkdownParser::new(content);
        let rule = MD004;
        let violations = rule.check(&parser, None);

        // Should not flag list markers inside code blocks
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_indented_code_block() {
        let content = r#"Regular text

    - This is an indented code block
    * Not a real list
    + Just code

* Real list item
"#;
        let parser = MarkdownParser::new(content);
        let rule = MD004;
        let violations = rule.check(&parser, None);

        // Should not flag list markers in indented code blocks
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_dash_in_code_block_with_real_list() {
        let content = r#"* List item 1

```python
# Comment with -- dashes
value = 10 - 5  # subtraction
```

+ List item 2
"#;
        let parser = MarkdownParser::new(content);
        let rule = MD004;
        let violations = rule.check(&parser, None);

        // Should only flag the inconsistent list marker, not code content
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 8); // Line with "+ List item 2"
    }
}
