use crate::config::Config;
use crate::error::{MarkdownlintError, Result};
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILES: &[&str] = &[
    ".markdownlint-cli2.jsonc",
    ".markdownlint-cli2.yaml",
    ".markdownlint-cli2.yml",
    ".markdownlint.jsonc",
    ".markdownlint.json",
    ".markdownlint.yaml",
    ".markdownlint.yml",
    "package.json",
];

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_file(path: &Path) -> Result<Config> {
        let content = fs::read_to_string(path).map_err(|e| {
            MarkdownlintError::Config(format!("Failed to read config file {:?}: {}", path, e))
        })?;

        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        Self::parse_config(&content, file_name, path)
    }

    pub fn discover_config(start_dir: &Path) -> Result<Option<Config>> {
        let mut current = start_dir.to_path_buf();

        loop {
            for config_file in CONFIG_FILES {
                let config_path = current.join(config_file);
                if config_path.exists() {
                    let config = Self::load_from_file(&config_path)?;
                    return Ok(Some(config));
                }
            }

            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    pub fn find_all_configs(start_dir: &Path) -> Result<Vec<(PathBuf, Config)>> {
        let mut configs = Vec::new();
        let mut current = start_dir.to_path_buf();

        loop {
            for config_file in CONFIG_FILES {
                let config_path = current.join(config_file);
                if config_path.exists() {
                    let config = Self::load_from_file(&config_path)?;
                    configs.push((config_path, config));
                    break;
                }
            }

            if !current.pop() {
                break;
            }
        }

        configs.reverse();
        Ok(configs)
    }

    fn parse_config(content: &str, file_name: &str, path: &Path) -> Result<Config> {
        if file_name == "package.json" {
            Self::parse_package_json(content)
        } else if file_name.ends_with(".jsonc") || file_name.ends_with(".json") {
            Self::parse_jsonc(content)
        } else if file_name.ends_with(".yaml") || file_name.ends_with(".yml") {
            Self::parse_yaml(content)
        } else {
            Err(MarkdownlintError::Config(format!(
                "Unknown config file format: {:?}",
                path
            )))
        }
    }

    fn parse_jsonc(content: &str) -> Result<Config> {
        let parsed = jsonc_parser::parse_to_serde_value(content, &Default::default())
            .map_err(|e| MarkdownlintError::Config(format!("Failed to parse JSONC: {:?}", e)))?;

        let value =
            parsed.ok_or_else(|| MarkdownlintError::Config("JSONC config is empty".to_string()))?;

        serde_json::from_value(value)
            .map_err(|e| MarkdownlintError::Config(format!("Invalid config structure: {}", e)))
    }

    fn parse_yaml(content: &str) -> Result<Config> {
        serde_yaml::from_str(content)
            .map_err(|e| MarkdownlintError::Config(format!("Failed to parse YAML: {}", e)))
    }

    fn parse_package_json(content: &str) -> Result<Config> {
        let package: serde_json::Value = serde_json::from_str(content).map_err(|e| {
            MarkdownlintError::Config(format!("Failed to parse package.json: {}", e))
        })?;

        if let Some(config_value) = package.get("markdownlint-cli2") {
            serde_json::from_value(config_value.clone()).map_err(|e| {
                MarkdownlintError::Config(format!("Invalid config in package.json: {}", e))
            })
        } else {
            Ok(Config::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_package_json() {
        let content = r#"{
            "name": "test-package",
            "version": "1.0.0",
            "markdownlint-cli2": {
                "fix": true,
                "globs": ["docs/*.md"]
            }
        }"#;

        let config = ConfigLoader::parse_package_json(content).unwrap();
        assert!(config.fix);
        assert_eq!(config.globs.len(), 1);
    }

    #[test]
    fn test_parse_package_json_no_config() {
        let content = r#"{
            "name": "test-package",
            "version": "1.0.0"
        }"#;

        let config = ConfigLoader::parse_package_json(content).unwrap();
        assert!(!config.fix);
        assert!(config.globs.is_empty());
    }

    #[test]
    fn test_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".markdownlint-cli2.jsonc");

        let mut file = fs::File::create(&config_path).unwrap();
        write!(file, r#"{{ "fix": true, "globs": ["*.md"] }}"#).unwrap();

        let config = ConfigLoader::load_from_file(&config_path).unwrap();
        assert!(config.fix);
        assert_eq!(config.globs.len(), 1);
    }

    #[test]
    fn test_discover_config() {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        let config_path = temp_dir.path().join(".markdownlint-cli2.jsonc");
        let mut file = fs::File::create(&config_path).unwrap();
        write!(file, r#"{{ "fix": true }}"#).unwrap();

        let config = ConfigLoader::discover_config(&sub_dir).unwrap();
        assert!(config.is_some());
        assert!(config.unwrap().fix);
    }
}
