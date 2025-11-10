use crate::config::{Config, RuleConfig};
use crate::error::Result;
use crate::lint::RuleRegistry;
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
        let registry = RuleRegistry::new();
        Self { config, registry }
    }

    pub fn lint_content(&self, content: &str) -> Result<Vec<Violation>> {
        let parser = MarkdownParser::new(content);
        let mut all_violations = Vec::new();

        // Run all rules
        for rule in self.registry.all_rules() {
            // Get rule-specific config if available
            let rule_config = self.config.config.get(rule.name());

            // Check if rule is disabled and extract config value
            let config_value = match rule_config {
                Some(RuleConfig::Enabled(false)) => continue, // Rule explicitly disabled
                Some(RuleConfig::Enabled(true)) => None,      // Rule enabled with no config
                Some(RuleConfig::Config(cfg)) => {
                    // Check if config has "enabled: false"
                    if let Some(Value::Bool(false)) = cfg.get("enabled") {
                        continue; // Rule disabled via config
                    }
                    // Convert HashMap to Value for the rule
                    Some(serde_json::to_value(cfg).unwrap())
                }
                None => None, // No config for this rule, use defaults
            };

            // Run the rule
            let violations = rule.check(&parser, config_value.as_ref());
            all_violations.extend(violations);
        }

        // Sort violations by line number
        all_violations.sort_by(|a, b| a.line.cmp(&b.line));

        Ok(all_violations)
    }

    pub fn lint_file(&self, path: &Path) -> Result<Vec<Violation>> {
        let content = std::fs::read_to_string(path)?;
        self.lint_content(&content)
    }
}

