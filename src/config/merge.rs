use crate::config::{Config, RuleConfig};
use std::collections::HashMap;

pub fn merge_configs(mut base: Config, override_cfg: Config) -> Config {
    if !override_cfg.custom_rules.is_empty() {
        base.custom_rules.extend(override_cfg.custom_rules);
    }

    if override_cfg.fix {
        base.fix = true;
    }

    if override_cfg.front_matter.is_some() {
        base.front_matter = override_cfg.front_matter;
    }

    if !override_cfg.gitignore {
        base.gitignore = false;
    }

    if !override_cfg.globs.is_empty() {
        base.globs.extend(override_cfg.globs);
    }

    if !override_cfg.ignores.is_empty() {
        base.ignores.extend(override_cfg.ignores);
    }

    if !override_cfg.markdown_it_plugins.is_empty() {
        base.markdown_it_plugins
            .extend(override_cfg.markdown_it_plugins);
    }

    if override_cfg.no_banner {
        base.no_banner = true;
    }

    if override_cfg.no_progress {
        base.no_progress = true;
    }

    if override_cfg.no_inline_config {
        base.no_inline_config = true;
    }

    if !override_cfg.output_formatters.is_empty() {
        base.output_formatters
            .extend(override_cfg.output_formatters);
    }

    for (rule_name, rule_config) in override_cfg.config {
        base.config.insert(rule_name, rule_config);
    }

    base
}

pub fn merge_rule_configs(
    base: &HashMap<String, RuleConfig>,
    override_cfg: &HashMap<String, RuleConfig>,
) -> HashMap<String, RuleConfig> {
    let mut merged = base.clone();

    for (rule_name, rule_config) in override_cfg {
        merged.insert(rule_name.clone(), rule_config.clone());
    }

    merged
}

pub fn merge_many_configs(configs: Vec<Config>) -> Config {
    configs.into_iter().fold(Config::default(), merge_configs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_configs_fix_flag() {
        let base = Config::default();
        let mut override_cfg = Config::default();
        override_cfg.fix = true;

        let merged = merge_configs(base, override_cfg);
        assert!(merged.fix);
    }

    #[test]
    fn test_merge_configs_globs() {
        let mut base = Config::default();
        base.globs = vec!["*.md".to_string()];

        let mut override_cfg = Config::default();
        override_cfg.globs = vec!["**/*.markdown".to_string()];

        let merged = merge_configs(base, override_cfg);
        assert_eq!(merged.globs.len(), 2);
        assert!(merged.globs.contains(&"*.md".to_string()));
        assert!(merged.globs.contains(&"**/*.markdown".to_string()));
    }

    #[test]
    fn test_merge_configs_rules() {
        let mut base = Config::default();
        base.config
            .insert("MD001".to_string(), RuleConfig::Enabled(true));

        let mut override_cfg = Config::default();
        override_cfg
            .config
            .insert("MD002".to_string(), RuleConfig::Enabled(false));

        let merged = merge_configs(base, override_cfg);
        assert_eq!(merged.config.len(), 2);
    }

    #[test]
    fn test_merge_many_configs() {
        let mut config1 = Config::default();
        config1.globs = vec!["*.md".to_string()];

        let mut config2 = Config::default();
        config2.globs = vec!["*.markdown".to_string()];
        config2.fix = true;

        let mut config3 = Config::default();
        config3.no_banner = true;

        let merged = merge_many_configs(vec![config1, config2, config3]);
        assert_eq!(merged.globs.len(), 2);
        assert!(merged.fix);
        assert!(merged.no_banner);
    }
}
