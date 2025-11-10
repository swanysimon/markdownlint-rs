use crate::lint::rule::Rule;
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use pulldown_cmark::{CodeBlockKind, Event, Tag};
use serde_json::Value;

pub struct MD014;

impl Rule for MD014 {
    fn name(&self) -> &str {
        "MD014"
    }

    fn description(&self) -> &str {
        "Dollar signs used before commands without showing output"
    }

    fn tags(&self) -> &[&str] {
        &["code"]
    }

    fn check(&self, parser: &MarkdownParser, _config: Option<&Value>) -> Vec<Violation> {
        let mut violations = Vec::new();
        let mut in_shell_code_block = false;
        let mut code_block_start_line = 0;
        let mut code_block_lines: Vec<String> = Vec::new();

        for (event, range) in parser.parse_with_offsets() {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    let lang_str = lang.to_string().to_lowercase();
                    in_shell_code_block = lang_str == "bash"
                        || lang_str == "sh"
                        || lang_str == "shell"
                        || lang_str == "console";
                    if in_shell_code_block {
                        code_block_start_line = parser.offset_to_line(range.start);
                        code_block_lines.clear();
                    }
                }
                Event::Text(text) if in_shell_code_block => {
                    code_block_lines.push(text.to_string());
                }
                Event::End(Tag::CodeBlock(_)) if in_shell_code_block => {
                    // Check if all non-empty lines start with $
                    let code_text = code_block_lines.join("");
                    let lines: Vec<&str> = code_text.lines().collect();
                    let non_empty_lines: Vec<&str> =
                        lines.iter().filter(|l| !l.trim().is_empty()).copied().collect();

                    if !non_empty_lines.is_empty() {
                        let all_start_with_dollar = non_empty_lines
                            .iter()
                            .all(|line| line.trim_start().starts_with('$'));

                        if all_start_with_dollar {
                            violations.push(Violation {
                                line: code_block_start_line + 1, // Line after opening fence
                                column: Some(1),
                                rule: self.name().to_string(),
                                message:
                                    "Dollar signs should not be used before commands without showing output"
                                        .to_string(),
                                fix: None,
                            });
                        }
                    }

                    in_shell_code_block = false;
                    code_block_lines.clear();
                }
                _ => {}
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
    fn test_no_dollar_signs() {
        let content = "```bash\nls -la\necho hello\n```";
        let parser = MarkdownParser::new(content);
        let rule = MD014;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_all_dollar_signs() {
        let content = "```bash\n$ ls -la\n$ echo hello\n```";
        let parser = MarkdownParser::new(content);
        let rule = MD014;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 2); // First line of code content
    }

    #[test]
    fn test_dollar_with_output() {
        let content = "```bash\n$ ls -la\ntotal 64\n$ echo hello\nhello\n```";
        let parser = MarkdownParser::new(content);
        let rule = MD014;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // Mixed lines, showing output
    }

    #[test]
    fn test_non_shell_language() {
        let content = "```python\n$ this is not a shell\n```";
        let parser = MarkdownParser::new(content);
        let rule = MD014;
        let violations = rule.check(&parser, None);

        assert_eq!(violations.len(), 0); // Not a shell language
    }
}
