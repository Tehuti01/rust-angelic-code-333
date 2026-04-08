use serde::{Deserialize, Serialize};
use crate::Types::SessionId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub poll_interval_ms: u64,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "https://bridge.anthropic.com".to_string(),
            poll_interval_ms: 1000,
        }
    }
}

pub struct BridgeSession {
    pub id: SessionId,
    pub config: BridgeConfig,
    pub active: bool,
}

impl BridgeSession {
    pub fn new(id: SessionId) -> Self {
        Self {
            id,
            config: BridgeConfig::default(),
            active: false,
        }
    }

    pub async fn connect(&mut self) -> anyhow::Result<()> {
        // Rust implementation of REPL bridge connection logic
        self.active = true;
        Ok(())
    }
}

pub struct BridgeManager {
    pub sessions: std::collections::HashMap<SessionId, BridgeSession>,
}

impl BridgeManager {
    pub fn new() -> Self {
        Self { sessions: std::collections::HashMap::new() }
    }
}
