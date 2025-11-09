mod md001;
mod md009;
mod md010;
mod md011;
mod md012;
mod md013;

pub use md001::MD001;
pub use md009::MD009;
pub use md010::MD010;
pub use md011::MD011;
pub use md012::MD012;
pub use md013::MD013;

use crate::lint::rule::RuleRegistry;

/// Create a registry with all built-in rules
pub fn create_default_registry() -> RuleRegistry {
    let mut registry = RuleRegistry::new();

    registry.register(Box::new(MD001));
    registry.register(Box::new(MD009));
    registry.register(Box::new(MD010));
    registry.register(Box::new(MD011));
    registry.register(Box::new(MD012));
    registry.register(Box::new(MD013));

    registry
}
