use serde::{Deserialize, Serialize};

/// High-Security MCP Protocol Schemas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<MCPError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub trait MCPHandler: Send + Sync {
    fn handle(&self, req: &MCPRequest) -> MCPResponse;
}

pub struct MCPRouter {
    handlers: std::collections::HashMap<String, Box<dyn MCPHandler>>,
}

impl MCPRouter {
    pub fn new() -> Self {
        Self { handlers: std::collections::HashMap::new() }
    }

    pub fn register(&mut self, method: &str, handler: Box<dyn MCPHandler>) {
        self.handlers.insert(method.to_string(), handler);
    }

    pub fn dispatch(&self, req: &MCPRequest) -> MCPResponse {
        if let Some(handler) = self.handlers.get(&req.method) {
            handler.handle(req)
        } else {
            MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            }
        }
    }
}
