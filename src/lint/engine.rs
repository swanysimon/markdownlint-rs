use crate::config::{Config, RuleConfig};
use crate::error::Result;
use crate::lint::{Rule, RuleRegistry};
use crate::markdown::MarkdownParser;
use crate::types::Violation;
use serde_json::Value;
use std::path::Path;

pub struct LintEngine {
    config: Config,
    registry: RuleRegistry,
}

impl LintEngine {
    pub fn new(config: Config) -> Self {
        let registry = crate::lint::rules::create_default_registry();
        Self { config, registry }
    }

    pub fn lint_content(&self, content: &str) -> Result<Vec<Violation>> {
        let parser = MarkdownParser::new(content);
        Ok(self
            .registry
            .all_rules()
            .map(|rule| self.violations(&parser, rule))
            .flatten()
            .collect())
    }

    fn violations(&self, parser: &MarkdownParser, rule: &dyn Rule) -> Vec<Violation> {
        let rule_config = self.config.config.get(rule.name());
        let config_value = match rule_config {
            Some(RuleConfig::Enabled(false)) => return Vec::new(),
            Some(RuleConfig::Enabled(true)) => None,
            Some(RuleConfig::Config(cfg)) => {
                if let Some(Value::Bool(false)) = cfg.get("enabled") {
                    return Vec::new();
                }
                Some(serde_json::to_value(cfg).unwrap())
            }
            None => None,
        };

        rule.check(&parser, config_value.as_ref())
    }

    pub fn lint_file(&self, path: &Path) -> Result<Vec<Violation>> {
        let content = std::fs::read_to_string(path)?;
        self.lint_content(&content)
    }
}
