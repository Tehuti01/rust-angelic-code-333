use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    Tool,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_use_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

// Ported from types/permissions.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionMode {
    Manual,
    Auto,
    Yolo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResult {
    pub granted: bool,
    pub reason: Option<String>,
}

// Ported from types/ids.ts
pub type AgentId = String;
pub type TaskId = String;
pub type SessionId = String;

// Ported from services/mcp/types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
}

// Ported from types/generated/...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalEvent {
    pub event_type: String,
    pub payload: Value,
    pub timestamp: u64,
}
