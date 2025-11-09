use crate::format::Formatter;
use crate::lint::LintResult;

pub struct JsonFormatter {
    pretty: bool,
}

impl JsonFormatter {
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, _result: &LintResult) -> String {
        todo!("Implement JSON formatter")
    }
}
