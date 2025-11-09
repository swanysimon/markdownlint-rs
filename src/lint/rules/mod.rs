mod md009;
mod md010;
mod md012;

pub use md009::MD009;
pub use md010::MD010;
pub use md012::MD012;

use crate::lint::rule::RuleRegistry;

/// Create a registry with all built-in rules
pub fn create_default_registry() -> RuleRegistry {
    let mut registry = RuleRegistry::new();

    registry.register(Box::new(MD009));
    registry.register(Box::new(MD010));
    registry.register(Box::new(MD012));

    registry
}
