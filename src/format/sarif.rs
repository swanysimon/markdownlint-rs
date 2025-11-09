use crate::format::Formatter;
use crate::lint::LintResult;

pub struct SarifFormatter;

impl SarifFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for SarifFormatter {
    fn format(&self, _result: &LintResult) -> String {
        todo!("Implement SARIF formatter")
    }
}
