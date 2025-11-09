use crate::error::Result;
use globset::GlobSet;
use std::path::Path;

pub struct GlobMatcher {
    includes: GlobSet,
    excludes: GlobSet,
}

impl GlobMatcher {
    pub fn new(include_patterns: &[String], exclude_patterns: &[String]) -> Result<Self> {
        todo!("Implement glob matcher")
    }

    pub fn matches(&self, _path: &Path) -> bool {
        todo!("Implement glob matching")
    }
}
