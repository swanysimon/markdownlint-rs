use crate::config::Config;
use crate::error::Result;
use crate::lint::{LintResult, RuleRegistry};
use std::path::Path;

pub struct LintEngine {
    _config: Config,
    _registry: RuleRegistry,
}

impl LintEngine {
    pub fn new(config: Config) -> Self {
        let registry = RuleRegistry::new();
        Self {
            _config: config,
            _registry: registry,
        }
    }

    pub fn lint_file(&self, _path: &Path) -> Result<LintResult> {
        todo!("Implement file linting")
    }

    pub fn lint_files(&self, _paths: &[&Path]) -> Result<LintResult> {
        todo!("Implement multi-file linting")
    }
}
