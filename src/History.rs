use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub timestamp: u64,
    pub session_id: String,
    pub title: String,
}

pub struct HistoryManager {
    pub entries: Vec<SessionEntry>,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, session_id: String, title: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.entries.push(SessionEntry { timestamp, session_id, title });
    }

    pub fn list_recent(&self, count: usize) -> Vec<SessionEntry> {
        let mut sorted = self.entries.clone();
        sorted.sort_by_key(|e| std::cmp::Reverse(e.timestamp));
        sorted.into_iter().take(count).collect()
    }
}
