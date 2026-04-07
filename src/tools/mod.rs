use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn execute(&self, args: &serde_json::Value) -> Result<serde_json::Value>;
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

    pub async fn call(&self, name: &str, args: &serde_json::Value) -> Result<serde_json::Value> {
        let tool = self.tools.get(name).ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?;
        tool.execute(args).await
    }
}
