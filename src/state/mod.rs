use std::sync::{Arc, Mutex};
use anyhow::Result;

/// Core Application State Management
pub struct AppState {
    pub is_ready: bool,
    pub active_sessions: usize,
    pub last_error: Option<String>,
}

pub struct StateManager {
    state: Arc<Mutex<AppState>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(AppState {
                is_ready: false,
                active_sessions: 0,
                last_error: None,
            })),
        }
    }

    pub fn set_ready(&self, ready: bool) -> Result<()> {
        let mut lock = self.state.lock().unwrap();
        lock.is_ready = ready;
        Ok(())
    }

    pub fn increment_sessions(&self) -> Result<()> {
        let mut lock = self.state.lock().unwrap();
        lock.active_sessions += 1;
        Ok(())
    }

    pub fn set_error(&self, error: String) -> Result<()> {
        let mut lock = self.state.lock().unwrap();
        lock.last_error = Some(error);
        Ok(())
    }

    pub fn get_state_snapshot(&self) -> AppState {
        let lock = self.state.lock().unwrap();
        AppState {
            is_ready: lock.is_ready,
            active_sessions: lock.active_sessions,
            last_error: lock.last_error.clone(),
        }
    }
}
