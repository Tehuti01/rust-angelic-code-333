use std::collections::HashMap;
use anyhow::Result;

/// Event-driven Hooks System
pub type HookCallback = Box<dyn Fn(&str) -> Result<String> + Send + Sync>;

pub struct HookManager {
    hooks: HashMap<String, Vec<HookCallback>>,
}

impl HookManager {
    pub fn new() -> Self {
        Self { hooks: HashMap::new() }
    }

    pub fn register_hook(&mut self, event: &str, callback: HookCallback) {
        self.hooks.entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(callback);
    }

    pub fn trigger_hook(&self, event: &str, payload: &str) -> Result<Vec<String>> {
        let mut results = Vec::new();
        if let Some(callbacks) = self.hooks.get(event) {
            for cb in callbacks {
                // Execute hooks with fault-tolerance
                match cb(payload) {
                    Ok(res) => results.push(res),
                    Err(e) => eprintln!("Hook execution failed for event {}: {}", event, e),
                }
            }
        }
        Ok(results)
    }
}
