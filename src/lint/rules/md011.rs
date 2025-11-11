use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use regex::Regex;
use serde_json::Value;

pub struct MD011;

impl Rule for MD011 {
    fn name(&self) -> &str {
        "MD011"
    }

    fn description(&self) -> &str {
        "Reversed link syntax"
    }

    fn tags(&self) -> &[&str] {
        &["links"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Pattern for reversed link syntax: (text)[url]
        // Match opening paren, non-empty content, closing paren, opening bracket, content, closing bracket
        let re = Regex::new(r"\([^)]+\)\[[^\]]+\]").unwrap();

        for (line_num, line) in parser.lines().iter().enumerate() {
            for m in re.find_iter(line) {
                violations.push(Violation {
                    line: line_num + 1,
                    column: Some(m.start() + 1),
                    rule: self.name().to_string(),
                    message: "Reversed link syntax (found '(text)[url]', should be '[text](url)')"
                        .to_string(),
                    fix: None,
                });
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
    fn test_correct_link_syntax() {
        let content = "This is [a link](http://example.com) and [another](url).";
        let parser = MarkdownParser::new(content);
        let rule = MD011;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_reversed_link_syntax() {
        let content = "This is (a link)[http://example.com] which is wrong.";
        let parser = MarkdownParser::new(content);
        let rule = MD011;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
    }

    #[test]
    fn test_multiple_reversed_links() {
        let content = "First (link)[url1] and second (link)[url2].";
        let parser = MarkdownParser::new(content);
        let rule = MD011;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_mixed_correct_and_reversed() {
        let content = "Correct [link](url) and (reversed)[url].";
        let parser = MarkdownParser::new(content);
        let rule = MD011;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
    }

    #[test]
    fn test_no_false_positives() {
        let content = "Some (parentheses) and [brackets] but not links.";
        let parser = MarkdownParser::new(content);
        let rule = MD011;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }
}
