use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use regex::Regex;
use serde_json::Value;

pub struct MD037;

impl Rule for MD037 {
    fn name(&self) -> &str {
        "MD037"
    }

    fn description(&self) -> &str {
        "Spaces inside emphasis markers"
    }

    fn tags(&self) -> &[&str] {
        &["whitespace", "emphasis"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Get byte ranges that are in code (more precise than line numbers)
        let code_ranges = parser.get_code_ranges();

        // Helper function to check if a position is within code
        let is_in_code = |line_num: usize, byte_offset: usize| -> bool {
            let absolute_offset = parser.line_offset_to_absolute(line_num, byte_offset);
            code_ranges
                .iter()
                .any(|range| range.contains(&absolute_offset))
        };

        // Regex patterns to detect spaces inside emphasis markers
        let strong_asterisk = Regex::new(r"\*\* .+? \*\*").unwrap(); // ** text **
        let strong_underscore = Regex::new(r"__ .+? __").unwrap(); // __ text __
        let em_asterisk = Regex::new(r"\* .+? \*").unwrap(); // * text *
        let em_underscore = Regex::new(r"_ .+? _").unwrap(); // _ text _

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            // Check for ** text **
            for mat in strong_asterisk.find_iter(line) {
                // Skip if this match is inside code
                if is_in_code(line_number, mat.start()) {
                    continue;
                }
                // Report violation for opening marker (space after **)
                violations.push(Violation {
                    line: line_number,
                    column: Some(mat.start() + 1),
                    rule: self.name().to_string(),
                    message: "Spaces inside emphasis markers".to_string(),
                    fix: None,
                });
                // Report violation for closing marker (space before **)
                violations.push(Violation {
                    line: line_number,
                    column: Some(mat.end() - 2), // Position of closing **
                    rule: self.name().to_string(),
                    message: "Spaces inside emphasis markers".to_string(),
                    fix: None,
                });
            }

            // Check for __ text __
            for mat in strong_underscore.find_iter(line) {
                // Skip if this match is inside code
                if is_in_code(line_number, mat.start()) {
                    continue;
                }
                // Report violation for opening marker
                violations.push(Violation {
                    line: line_number,
                    column: Some(mat.start() + 1),
                    rule: self.name().to_string(),
                    message: "Spaces inside emphasis markers".to_string(),
                    fix: None,
                });
                // Report violation for closing marker
                violations.push(Violation {
                    line: line_number,
                    column: Some(mat.end() - 2),
                    rule: self.name().to_string(),
                    message: "Spaces inside emphasis markers".to_string(),
                    fix: None,
                });
            }

            // Check for * text * (but avoid ** text **)
            for mat in em_asterisk.find_iter(line) {
                // Skip if this match is inside code
                if is_in_code(line_number, mat.start()) {
                    continue;
                }
                // Make sure it's not part of **
                let before_pos = mat.start();
                let after_pos = mat.end();
                let is_strong = (before_pos > 0 && line.chars().nth(before_pos - 1) == Some('*'))
                    || (after_pos < line.len() && line.chars().nth(after_pos) == Some('*'));

                if !is_strong {
                    // Report violations for both opening and closing markers
                    violations.push(Violation {
                        line: line_number,
                        column: Some(mat.start() + 1),
                        rule: self.name().to_string(),
                        message: "Spaces inside emphasis markers".to_string(),
                        fix: None,
                    });
                    violations.push(Violation {
                        line: line_number,
                        column: Some(mat.end()),
                        rule: self.name().to_string(),
                        message: "Spaces inside emphasis markers".to_string(),
                        fix: None,
                    });
                }
            }

            // Check for _ text _ (but avoid __ text __)
            for mat in em_underscore.find_iter(line) {
                // Skip if this match is inside code
                if is_in_code(line_number, mat.start()) {
                    continue;
                }
                let before_pos = mat.start();
                let after_pos = mat.end();
                let is_strong = (before_pos > 0 && line.chars().nth(before_pos - 1) == Some('_'))
                    || (after_pos < line.len() && line.chars().nth(after_pos) == Some('_'));

                if !is_strong {
                    // Report violations for both opening and closing markers
                    violations.push(Violation {
                        line: line_number,
                        column: Some(mat.start() + 1),
                        rule: self.name().to_string(),
                        message: "Spaces inside emphasis markers".to_string(),
                        fix: None,
                    });
                    violations.push(Violation {
                        line: line_number,
                        column: Some(mat.end()),
                        rule: self.name().to_string(),
                        message: "Spaces inside emphasis markers".to_string(),
                        fix: None,
                    });
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
    fn test_correct_emphasis() {
        let content = "This is **bold** and *italic* text.";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_spaces_in_strong() {
        let content = "This is ** bold ** text.";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 2); // One for opening, one for closing
    }

    #[test]
    fn test_spaces_in_emphasis() {
        let content = "This is * italic * text.";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 2); // One for opening, one for closing
    }

    #[test]
    fn test_underscores() {
        let content = "This is __ bold __ text.";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 2); // One for opening, one for closing
    }

    #[test]
    fn test_code_block_with_underscores() {
        let content = "Normal text\n\n```sql\nCREATE POLICY territory_contact_access ON contacts\n  FOR SELECT\n  USING (\n    territory_id IN (\n      SELECT territory_id\n      FROM user_territory_assignments\n      WHERE user_id = current_setting('app.current_user_id')::uuid\n        AND (valid_to IS NULL OR valid_to > NOW())\n    )\n  );\n```\n\nMore text";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        // Should not flag underscores in SQL identifiers as emphasis
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_inline_code_with_underscores() {
        let content = "Use the `user_id` variable for * 2 * 3 multiplication.";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        // The * 2 * 3 part should be flagged (not in code), but user_id should not
        assert_eq!(violations.len(), 2); // Only the * 2 * emphasis
    }

    #[test]
    fn test_typescript_multiplication() {
        let content = "```typescript\nconst result = value_a * value_b * value_c;\n```";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        // Should not flag asterisks in code as emphasis markers
        assert_eq!(violations.len(), 0);
    }
}
