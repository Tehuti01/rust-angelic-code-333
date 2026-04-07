use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ToolValidationResult {
    pub is_valid: bool,
    pub message: Option<String>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn input_schema(&self) -> Value;
    
    async fn validate(&self, _args: &Value) -> Result<ToolValidationResult> {
        Ok(ToolValidationResult {
            is_valid: true,
            message: None,
        })
    }

    async fn execute(&self, args: &Value) -> Result<Value>;
}

pub mod file;
pub mod shell;
pub mod replace;
pub mod glob;

pub struct ToolRegistry {
    tools: std::collections::HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: std::collections::HashMap::new(),
        };
        registry.register(Box::new(file::ReadFileTool));
        registry.register(Box::new(file::WriteFileTool));
        registry.register(Box::new(file::ListDirectoryTool));
        registry.register(Box::new(file::GrepSearchTool));
        registry.register(Box::new(shell::RunShellCommandTool));
        registry.register(Box::new(replace::ReplaceTool));
        registry.register(Box::new(glob::GlobTool));
        registry
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|b| b.as_ref())
    }

    pub async fn call(&self, name: &str, args: &Value) -> Result<Value> {
        let tool = self.tools.get(name).ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?;
        
        let validation = tool.validate(args).await?;
        if !validation.is_valid {
            return Err(anyhow::anyhow!("Validation failed: {}", validation.message.unwrap_or_default()));
        }

        tool.execute(args).await
    }

    pub fn list_tools(&self) -> Vec<Value> {
        self.tools.values().map(|t| {
            serde_json::json!({
                "name": t.name(),
                "description": t.description(),
                "input_schema": t.input_schema(),
            })
        }).collect()
    }
}
