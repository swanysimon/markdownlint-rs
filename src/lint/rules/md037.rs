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

        // Regex patterns to detect spaces inside emphasis markers
        let strong_asterisk = Regex::new(r"\*\* .+? \*\*").unwrap(); // ** text **
        let strong_underscore = Regex::new(r"__ .+? __").unwrap();   // __ text __
        let em_asterisk = Regex::new(r"\* .+? \*").unwrap();         // * text *
        let em_underscore = Regex::new(r"_ .+? _").unwrap();         // _ text _

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            // Check for ** text **
            for mat in strong_asterisk.find_iter(line) {
                violations.push(Violation {
                    line: line_number,
                    column: Some(mat.start() + 1),
                    rule: self.name().to_string(),
                    message: "Spaces inside emphasis markers".to_string(),
                    fix: None,
                });
            }

            // Check for __ text __
            for mat in strong_underscore.find_iter(line) {
                violations.push(Violation {
                    line: line_number,
                    column: Some(mat.start() + 1),
                    rule: self.name().to_string(),
                    message: "Spaces inside emphasis markers".to_string(),
                    fix: None,
                });
            }

            // Check for * text * (but avoid ** text **)
            for mat in em_asterisk.find_iter(line) {
                // Make sure it's not part of **
                let before_pos = mat.start();
                let after_pos = mat.end();
                let is_strong = (before_pos > 0 && line.chars().nth(before_pos - 1) == Some('*'))
                    || (after_pos < line.len() && line.chars().nth(after_pos) == Some('*'));

                if !is_strong {
                    violations.push(Violation {
                        line: line_number,
                        column: Some(mat.start() + 1),
                        rule: self.name().to_string(),
                        message: "Spaces inside emphasis markers".to_string(),
                        fix: None,
                    });
                }
            }

            // Check for _ text _ (but avoid __ text __)
            for mat in em_underscore.find_iter(line) {
                let before_pos = mat.start();
                let after_pos = mat.end();
                let is_strong = (before_pos > 0 && line.chars().nth(before_pos - 1) == Some('_'))
                    || (after_pos < line.len() && line.chars().nth(after_pos) == Some('_'));

                if !is_strong {
                    violations.push(Violation {
                        line: line_number,
                        column: Some(mat.start() + 1),
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

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_spaces_in_emphasis() {
        let content = "This is * italic * text.";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_underscores() {
        let content = "This is __ bold __ text.";
        let parser = MarkdownParser::new(content);
        let rule = MD037;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }
}
