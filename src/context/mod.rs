use anyhow::Result;
use crate::Tool::ToolRegistry;
use crate::Cost::CostTracker;
use crate::Task::TaskState;
use crate::types::Message;
use crate::Services::Services;
use crate::Config::ConfigManager;
use crate::Buddy::BuddyCompanion;
use crate::bridge::BridgeManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Provider {
    OpenRouter,
    Google,
    Nvidia,
}

pub struct Context {
    pub history: Vec<String>,
    pub messages: Vec<Message>,
    pub tasks: HashMap<String, TaskState>,
    pub tools: ToolRegistry,
    pub cost: CostTracker,
    pub provider: Provider,
    pub model: String,
    pub cwd: String,
    pub services: Services,
    pub config: ConfigManager,
    pub buddy: BuddyCompanion,
    pub bridge: BridgeManager,
}

impl Context {
    pub async fn new() -> Result<Self> {
        let cwd = std::env::current_dir()?
            .to_string_lossy()
            .into_owned();

        Ok(Self {
            history: Vec::new(),
            messages: Vec::new(),
            tasks: HashMap::new(),
            tools: ToolRegistry::new(),
            cost: CostTracker::default(),
            provider: Provider::Google,
            model: "gemini-2.0-flash".to_string(),
            cwd,
            services: Services::new(),
            config: ConfigManager::new(),
            buddy: BuddyCompanion::new(),
            bridge: BridgeManager::new(),
        })
    }
}

/// UI and Application Context Providers (Replaces React Contexts)
pub struct UIContexts {
    pub fps_metrics: Arc<Mutex<f32>>,
    pub notifications: Arc<Mutex<Vec<String>>>,
    pub modal_active: Arc<Mutex<bool>>,
    pub voice_active: Arc<Mutex<bool>>,
    pub mailbox_messages: Arc<Mutex<usize>>,
}

impl UIContexts {
    pub fn new() -> Self {
        Self {
            fps_metrics: Arc::new(Mutex::new(60.0)),
            notifications: Arc::new(Mutex::new(Vec::new())),
            modal_active: Arc::new(Mutex::new(false)),
            voice_active: Arc::new(Mutex::new(false)),
            mailbox_messages: Arc::new(Mutex::new(0)),
        }
    }

    pub fn add_notification(&self, msg: &str) {
        let mut lock = self.notifications.lock().unwrap();
        lock.push(msg.to_string());
    }

    pub fn toggle_modal(&self, state: bool) {
        let mut lock = self.modal_active.lock().unwrap();
        *lock = state;
    }
}
