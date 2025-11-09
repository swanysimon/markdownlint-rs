mod loader;
mod merge;
mod types;

pub use loader::ConfigLoader;
pub use merge::merge_configs;
pub use types::{Config, OutputFormat, RuleConfig};
