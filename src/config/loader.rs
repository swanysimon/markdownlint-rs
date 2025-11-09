use crate::config::Config;
use crate::error::{MarkdownlintError, Result};
use std::path::Path;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_file(_path: &Path) -> Result<Config> {
        todo!("Implement config file loading")
    }

    pub fn discover_config(_start_dir: &Path) -> Result<Option<Config>> {
        todo!("Implement config discovery")
    }
}
