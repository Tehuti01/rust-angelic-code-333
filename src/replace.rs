use anyhow::Result;
use async_trait::async_trait;
use crate::Tool::Tool;
use std::fs;
use serde_json::{json, Value};

pub struct ReplaceTool;

#[async_trait]
impl Tool for ReplaceTool {
    fn name(&self) -> &'static str { "replace" }
    fn description(&self) -> &'static str { "Replaces a specific string in a file with a new one." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": { "type": "string" },
                "old_string": { "type": "string" },
                "new_string": { "type": "string" }
            },
            "required": ["file_path", "old_string", "new_string"]
        })
    }
    async fn execute(&self, args: &Value) -> Result<Value> {
        let path_str = args["file_path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;
        let old_string = args["old_string"].as_str().ok_or_else(|| anyhow::anyhow!("Missing old_string"))?;
        let new_string = args["new_string"].as_str().ok_or_else(|| anyhow::anyhow!("Missing new_string"))?;
        
        let content = fs::read_to_string(path_str)?;
        let new_content = content.replace(old_string, new_string);
        
        if content == new_content {
            return Err(anyhow::anyhow!("Old string not found or no changes made."));
        }
        
        fs::write(path_str, new_content)?;
        Ok(json!({ "status": "success" }))
    }
}
