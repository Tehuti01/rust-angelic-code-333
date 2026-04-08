use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

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
    pub arguments: serde_json::Value,
}

// Ported from types/permissions.ts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub payload: serde_json::Value,
    pub timestamp: u64,
}

/// Core Type Definitions (Replaces TS Types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandType {
    pub name: String,
    pub description: String,
    pub is_hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginType {
    pub id: String,
    pub manifest_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogType {
    pub level: String,
    pub message: String,
    pub timestamp: u64,
}

pub type TextInputValue = String;

// Generated Protobuf/Event types
pub mod generated {
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Timestamp {
        pub seconds: i64,
        pub nanos: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AuthEvent {
        pub token_id: String,
        pub status: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InternalEvent {
        pub event_name: String,
        pub payload: serde_json::Value,
    }
}
