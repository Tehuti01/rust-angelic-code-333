use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
    pub auto_mode_opt_in: bool,
    pub secondary_model: Option<String>,
    pub custom_instructions: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            auto_mode_opt_in: false,
            secondary_model: None,
            custom_instructions: HashMap::new(),
        }
    }
}

pub struct ConfigManager {
    pub current: Config,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self { current: Config::default() }
    }

    pub fn update(&mut self, new_config: Config) {
        self.current = new_config;
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        // Simple mock for dynamic config lookup
        match key {
            "theme" => Some(self.current.theme.clone()),
            _ => None,
        }
    }
}

pub struct RemoteSettings {
    pub last_sync: u64,
}

impl RemoteSettings {
    pub fn new() -> Self {
        Self { last_sync: 0 }
    }

    pub async fn sync(&mut self) -> Result<()> {
        // Logic for syncing with Anthropic's managed settings API
        Ok(())
    }
}
