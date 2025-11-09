use crate::types::Violation;
use std::collections::HashMap;

pub trait Rule: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn tags(&self) -> &[&str];
    fn check(&self, content: &str) -> Vec<Violation>;
}

#[derive(Default)]
pub struct RuleRegistry {
    rules: HashMap<String, Box<dyn Rule>>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, rule: Box<dyn Rule>) {
        self.rules.insert(rule.name().to_string(), rule);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Rule> {
        self.rules.get(name).map(|r| r.as_ref())
    }

    pub fn all_rules(&self) -> impl Iterator<Item = &dyn Rule> {
        self.rules.values().map(|r| r.as_ref())
    }
}
