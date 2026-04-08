use anyhow::Result;
use crate::Tool::ToolRegistry;
use crate::Cost::CostTracker;
use crate::Task::TaskState;
use crate::Types::Message;
use crate::Services::Services;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        })
    }
}
