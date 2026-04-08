use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// High-Performance Keybinding Engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybinding {
    pub command: String,
    pub key: String,
    pub description: String,
}

pub struct KeybindingManager {
    bindings: HashMap<String, Keybinding>,
}

impl KeybindingManager {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn load_defaults(&mut self) {
        self.bindings.insert(
            "ctrl+c".to_string(),
            Keybinding {
                command: "exit".to_string(),
                key: "ctrl+c".to_string(),
                description: "Exit application".to_string(),
            },
        );
        self.bindings.insert(
            "ctrl+r".to_string(),
            Keybinding {
                command: "search_history".to_string(),
                key: "ctrl+r".to_string(),
                description: "Search history".to_string(),
            },
        );
    }

    pub fn register(&mut self, key: &str, command: &str, desc: &str) {
        self.bindings.insert(
            key.to_string(),
            Keybinding {
                command: command.to_string(),
                key: key.to_string(),
                description: desc.to_string(),
            },
        );
    }

    pub fn resolve(&self, key: &str) -> Option<String> {
        self.bindings.get(key).map(|b| b.command.clone())
    }
}
