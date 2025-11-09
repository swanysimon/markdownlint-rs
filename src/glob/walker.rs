use crate::error::Result;
use std::path::{Path, PathBuf};

pub struct FileWalker {
    respect_gitignore: bool,
}

impl FileWalker {
    pub fn new(respect_gitignore: bool) -> Self {
        Self { respect_gitignore }
    }

    pub fn find_markdown_files(&self, _root: &Path) -> Result<Vec<PathBuf>> {
        todo!("Implement file discovery")
    }
}
