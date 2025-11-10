use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use regex::Regex;
use serde_json::Value;

pub struct MD038;

impl Rule for MD038 {
    fn name(&self) -> &str {
        "MD038"
    }

    fn description(&self) -> &str {
        "Spaces inside code span elements"
    }

    fn tags(&self) -> &[&str] {
        &["whitespace", "code"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Regex to detect spaces inside backticks
        // ` text` or `text ` or ` text `
        let pattern = Regex::new(r"`( .+?|.+? | .+? )`").unwrap();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;

            for mat in pattern.find_iter(line) {
                let matched_text = mat.as_str();
                // Extract the content between backticks
                let content = &matched_text[1..matched_text.len() - 1];

                // Check if there are leading or trailing spaces
                if content.starts_with(' ') || content.ends_with(' ') {
                    // Exception: allow ` ` (single space) or multiple spaces intentionally
                    if content.trim().is_empty() {
                        continue; // Intentional space code
                    }

                    violations.push(Violation {
                        line: line_number,
                        column: Some(mat.start() + 1),
                        rule: self.name().to_string(),
                        message: "Spaces inside code span elements".to_string(),
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
    fn test_correct_code_span() {
        let content = "Use the `function()` to call it.";
        let parser = MarkdownParser::new(content);
        let rule = MD038;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_leading_space() {
        let content = "Use the ` function()` to call it.";
        let parser = MarkdownParser::new(content);
        let rule = MD038;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_trailing_space() {
        let content = "Use the `function() ` to call it.";
        let parser = MarkdownParser::new(content);
        let rule = MD038;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_both_spaces() {
        let content = "Use the ` function() ` to call it.";
        let parser = MarkdownParser::new(content);
        let rule = MD038;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn test_intentional_space() {
        let content = "A single space ` ` character.";
        let parser = MarkdownParser::new(content);
        let rule = MD038;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // Intentional space, should be allowed
    }
}
