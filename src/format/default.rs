use crate::format::Formatter;
use crate::lint::LintResult;

pub struct DefaultFormatter {
    use_color: bool,
}

impl DefaultFormatter {
    pub fn new(use_color: bool) -> Self {
        Self { use_color }
    }
}

impl Formatter for DefaultFormatter {
    fn format(&self, _result: &LintResult) -> String {
        todo!("Implement default formatter")
    }

    fn supports_color(&self) -> bool {
        self.use_color
    }
}
