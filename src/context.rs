use anyhow::Result;
use crate::tools::ToolRegistry;
use crate::cost::CostTracker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Provider {
    OpenRouter,
    Google,
    Nvidia,
}

pub struct Context {
    pub history: Vec<String>,
    pub tools: ToolRegistry,
    pub cost: CostTracker,
    pub provider: Provider,
    pub model: String,
}

impl Context {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            history: Vec::new(),
            tools: ToolRegistry::new(),
            cost: CostTracker::default(),
            provider: Provider::Google,
            model: "gemini-2.0-flash".to_string(),
        })
    }
}
