use anyhow::Result;
use async_trait::async_trait;
use crate::tools::Tool;
use std::fs;

pub struct ReplaceTool;

#[async_trait]
impl Tool for ReplaceTool {
    fn name(&self) -> &'static str { "replace" }
    fn description(&self) -> &'static str { "Replaces a specific string in a file with a new one." }
    async fn execute(&self, args: &serde_json::Value) -> Result<serde_json::Value> {
        let path_str = args["file_path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;
        let old_string = args["old_string"].as_str().ok_or_else(|| anyhow::anyhow!("Missing old_string"))?;
        let new_string = args["new_string"].as_str().ok_or_else(|| anyhow::anyhow!("Missing new_string"))?;
        
        let content = fs::read_to_string(path_str)?;
        let new_content = content.replace(old_string, new_string);
        
        if content == new_content {
            return Err(anyhow::anyhow!("Old string not found or no changes made."));
        }
        
        fs::write(path_str, new_content)?;
        Ok(serde_json::json!({ "status": "success" }))
    }
}
