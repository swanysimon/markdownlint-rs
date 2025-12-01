use crate::format::Formatter;
use crate::lint::LintResult;

pub struct DefaultFormatter {
    use_color: bool,
}

impl DefaultFormatter {
    pub fn new(use_color: bool) -> Self {
        Self { use_color }
    }

    fn colorize(&self, text: &str, color_code: &str) -> String {
        if self.use_color {
            format!("\x1b[{}m{}\x1b[0m", color_code, text)
        } else {
            text.to_string()
        }
    }

    fn red(&self, text: &str) -> String {
        self.colorize(text, "31")
    }

    fn yellow(&self, text: &str) -> String {
        self.colorize(text, "33")
    }

    fn gray(&self, text: &str) -> String {
        self.colorize(text, "90")
    }
}

impl Formatter for DefaultFormatter {
    fn format(&self, result: &LintResult) -> String {
        let mut output = String::new();

        // Output violations by file
        for file_result in &result.file_results {
            if file_result.violations.is_empty() {
                continue;
            }

            // File path header
            let path_display = file_result.path.display();
            output.push_str(&format!("{}\n", self.yellow(&path_display.to_string())));

            // Each violation
            for violation in &file_result.violations {
                let location = if let Some(col) = violation.column {
                    format!("{}:{}", violation.line, col)
                } else {
                    format!("{}", violation.line)
                };

                output.push_str(&format!(
                    "  {}: {} {}\n",
                    self.gray(&location),
                    self.red(&violation.rule),
                    violation.message
                ));
            }

            output.push('\n');
        }

        // Summary
        if result.total_errors == 0 {
            output.push_str("No errors found.\n");
        } else {
            let summary = format!(
                "Found {} error(s) across {} file(s)",
                result.total_errors,
                result.file_results.len()
            );
            output.push_str(&format!("{}\n", self.red(&summary)));
        }

        output
    }

    fn supports_color(&self) -> bool {
        self.use_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Violation;
    use std::path::PathBuf;

    #[test]
    fn test_no_errors() {
        let formatter = DefaultFormatter::new(false);
        let result = LintResult::new();
        let output = formatter.format(&result);

        assert!(output.contains("No errors found"));
    }

    #[test]
    fn test_single_violation() {
        let formatter = DefaultFormatter::new(false);
        let mut result = LintResult::new();

        result.add_file_result(
            PathBuf::from("test.md"),
            vec![Violation {
                line: 5,
                column: Some(10),
                rule: "MD001".to_string(),
                message: "Heading levels should increment by one".to_string(),
                fix: None,
            }],
        );

        let output = formatter.format(&result);

        assert!(output.contains("test.md"));
        assert!(output.contains("5:10"));
        assert!(output.contains("MD001"));
        assert!(output.contains("Heading levels"));
        assert!(output.contains("Found 1 error(s)"));
    }

    #[test]
    fn test_multiple_violations() {
        let formatter = DefaultFormatter::new(false);
        let mut result = LintResult::new();

        result.add_file_result(
            PathBuf::from("file1.md"),
            vec![
                Violation {
                    line: 1,
                    column: Some(1),
                    rule: "MD001".to_string(),
                    message: "First error".to_string(),
                    fix: None,
                },
                Violation {
                    line: 10,
                    column: None,
                    rule: "MD002".to_string(),
                    message: "Second error".to_string(),
                    fix: None,
                },
            ],
        );

        result.add_file_result(
            PathBuf::from("file2.md"),
            vec![Violation {
                line: 3,
                column: Some(5),
                rule: "MD003".to_string(),
                message: "Third error".to_string(),
                fix: None,
            }],
        );

        let output = formatter.format(&result);

        assert!(output.contains("file1.md"));
        assert!(output.contains("file2.md"));
        assert!(output.contains("Found 3 error(s) across 2 file(s)"));
    }

    #[test]
    fn test_with_color() {
        let formatter = DefaultFormatter::new(true);
        let mut result = LintResult::new();

        result.add_file_result(
            PathBuf::from("test.md"),
            vec![Violation {
                line: 5,
                column: Some(10),
                rule: "MD001".to_string(),
                message: "Test error".to_string(),
                fix: None,
            }],
        );

        let output = formatter.format(&result);

        // Should contain ANSI color codes
        assert!(output.contains("\x1b["));
    }
}
