use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::{Fix, Violation};
use pulldown_cmark::{Event, Tag};
use serde_json::Value;
use std::collections::HashSet;

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

        // Use AST to identify lines that start with emphasis (to exclude them)
        let mut emphasis_start_lines = HashSet::new();

        // Calculate line start offsets
        let mut line_offsets = vec![0];
        let mut current_offset = 0;
        for line in parser.lines() {
            current_offset += line.len() + 1; // +1 for newline
            line_offsets.push(current_offset);
        }

        for (event, range) in parser.parse_with_offsets() {
            if let Event::Start(Tag::Emphasis | Tag::Strong) = event {
                let line_num = parser.offset_to_line(range.start);
                // Check if this emphasis starts at the beginning of the line (after whitespace)
                if let Some(line) = parser.lines().get(line_num - 1) {
                    let trimmed_start = line.len() - line.trim_start().len();
                    // If the emphasis starts right at the trimmed position, exclude this line
                    if let Some(&line_start_offset) = line_offsets.get(line_num - 1)
                        && range.start == line_start_offset + trimmed_start
                    {
                        emphasis_start_lines.insert(line_num);
                    }
                }
            }
        }

        // Now check spacing using string matching, but skip emphasis lines
        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            // Skip if line starts with emphasis (bold or italic)
            if emphasis_start_lines.contains(&line_number) {
                continue;
            }

            let trimmed = line.trim_start();

            // Skip horizontal rules (3+ of same char: -, *, _)
            if is_horizontal_rule(trimmed) {
                continue;
            }

            // Check unordered list markers
            if trimmed.starts_with('*') || trimmed.starts_with('+') || trimmed.starts_with('-') {
                let marker_char = trimmed.chars().next().unwrap();
                let after_marker = &trimmed[1..];
                let space_count = after_marker.chars().take_while(|&c| c == ' ').count();

                // Only check if there's content after the marker (not just a marker alone)
                if !after_marker.trim().is_empty() {
                    // For now, assume single-line (could be enhanced to detect multi-line)
                    let expected = ul_single;

                    if space_count != expected {
                        // Fix the spacing after list marker
                        let leading_spaces = &line[..line.len() - trimmed.len()];
                        let content = after_marker[space_count..].trim_start();
                        let spaces = " ".repeat(expected);
                        let replacement =
                            format!("{}{}{}{}", leading_spaces, marker_char, spaces, content);

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

/// Check if a line is a horizontal rule (3+ of same char: -, *, _)
fn is_horizontal_rule(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.len() < 3 {
        return false;
    }

    let chars: Vec<char> = trimmed.chars().filter(|&c| c != ' ').collect();
    if chars.len() < 3 {
        return false;
    }

    let first_char = chars[0];
    if first_char != '-' && first_char != '*' && first_char != '_' {
        return false;
    }

    chars.iter().all(|&c| c == first_char)
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

    #[test]
    fn test_bold_not_list_marker() {
        // Bold/emphasis at start of line should not be treated as list marker
        let content = "**Slice-specific schemas** → some text\n\
                       **Bold text** at start\n\
                       *Italic text* here\n\
                       __Also bold__ text";
        let parser = MarkdownParser::new(content);
        let rule = MD030;
        let violations = rule.check(&parser, None);

        assert_eq!(
            violations.len(),
            0,
            "Bold/emphasis should not trigger MD030"
        );
    }

    #[test]
    fn test_actual_list_with_bold() {
        // Actual list items can contain bold text
        let content = "* **Bold** item\n\
                       + *Italic* item\n\
                       - Normal item";
        let parser = MarkdownParser::new(content);
        let rule = MD030;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_horizontal_rules_not_list_markers() {
        // Horizontal rules should not trigger MD030 violations
        let content = "# Heading\n\
                       \n\
                       ---\n\
                       \n\
                       More content\n\
                       \n\
                       ***\n\
                       \n\
                       ___\n\
                       \n\
                       * * *\n\
                       \n\
                       - - -";
        let parser = MarkdownParser::new(content);
        let rule = MD030;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0, "Horizontal rules should not be treated as list markers");
    }
}
