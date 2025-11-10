use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;

pub struct MD027;

impl Rule for MD027 {
    fn name(&self) -> &str {
        "MD027"
    }

    fn description(&self) -> &str {
        "Multiple spaces after blockquote symbol"
    }

    fn tags(&self) -> &[&str] {
        &["blockquote", "whitespace", "indentation"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        for (line_num, line) in parser.lines().iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim_start();

            // Check if line starts with blockquote marker
            if trimmed.starts_with('>') {
                let after_gt = &trimmed[1..];

                // Count spaces after the >
                let space_count = after_gt.chars().take_while(|&c| c == ' ').count();

                if space_count > 1 {
                    violations.push(Violation {
                        line: line_number,
                        column: Some(line.len() - trimmed.len() + 2),
                        rule: self.name().to_string(),
                        message: format!(
                            "Multiple spaces after blockquote symbol ({} spaces)",
                            space_count
                        ),
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
    fn test_correct_blockquote() {
        let content = "> Quote line 1\n> Quote line 2";
        let parser = MarkdownParser::new(content);
        let rule = MD027;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_multiple_spaces() {
        let content = ">  Quote with 2 spaces\n> Correct quote";
        let parser = MarkdownParser::new(content);
        let rule = MD027;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
    }

    #[test]
    fn test_many_spaces() {
        let content = ">     Quote with 5 spaces";
        let parser = MarkdownParser::new(content);
        let rule = MD027;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("5 spaces"));
    }

    #[test]
    fn test_no_space() {
        let content = ">Quote without space";
        let parser = MarkdownParser::new(content);
        let rule = MD027;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // No space is allowed
    }
}
