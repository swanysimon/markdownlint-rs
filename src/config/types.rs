use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub config: HashMap<String, RuleConfig>,

    #[serde(default)]
    pub custom_rules: Vec<String>,

    #[serde(default)]
    pub fix: bool,

    #[serde(default)]
    pub front_matter: Option<String>,

    #[serde(default = "default_gitignore")]
    pub gitignore: bool,

    #[serde(default)]
    pub globs: Vec<String>,

    #[serde(default)]
    pub ignores: Vec<String>,

    #[serde(default)]
    pub markdown_it_plugins: Vec<String>,

    #[serde(default)]
    pub no_banner: bool,

    #[serde(default)]
    pub no_progress: bool,

    #[serde(default)]
    pub no_inline_config: bool,

    #[serde(default)]
    pub output_formatters: Vec<FormatterConfig>,
}

fn default_gitignore() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RuleConfig {
    Enabled(bool),
    Config(HashMap<String, serde_json::Value>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormatterConfig {
    pub name: String,
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Default,
    Json,
    Junit,
    Sarif,
    GitHub,
}
