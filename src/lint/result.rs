use crate::types::{FileResult, Violation};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct LintResult {
    pub file_results: Vec<FileResult>,
    pub total_errors: usize,
}

impl LintResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file_result(&mut self, path: PathBuf, violations: Vec<Violation>) {
        self.total_errors += violations.len();
        self.file_results.push(FileResult { path, violations });
    }

    pub fn has_errors(&self) -> bool {
        self.total_errors > 0
    }
}
