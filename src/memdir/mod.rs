use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use anyhow::Result;

/// Persistent Memory Directory System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub key: String,
    pub content: String,
    pub timestamp: u64,
    pub tags: Vec<String>,
}

pub struct MemDirManager {
    entries: Arc<RwLock<HashMap<String, MemoryEntry>>>,
}

impl MemDirManager {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn save_memory(&self, key: &str, content: &str, tags: Vec<String>) -> Result<()> {
        let entry = MemoryEntry {
            key: key.to_string(),
            content: content.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            tags,
        };

        let mut lock = self.entries.write().unwrap();
        lock.insert(key.to_string(), entry);
        Ok(())
    }

    pub fn get_memory(&self, key: &str) -> Option<MemoryEntry> {
        let lock = self.entries.read().unwrap();
        lock.get(key).cloned()
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<MemoryEntry> {
        let lock = self.entries.read().unwrap();
        lock.values()
            .filter(|e| e.tags.contains(&tag.to_string()))
            .cloned()
            .collect()
    }
}
