use anyhow::Result;
use async_trait::async_trait;
use crate::Tool::Tool;
use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};

pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &'static str { "read_file" }
    fn description(&self) -> &'static str { "Reads the content of a file." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": { "type": "string" }
            },
            "required": ["file_path"]
        })
    }
    async fn execute(&self, args: &Value) -> Result<Value> {
        let path_str = args["file_path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;
        let content = fs::read_to_string(path_str)?;
        Ok(json!({ "content": content }))
    }
}

pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &'static str { "write_file" }
    fn description(&self) -> &'static str { "Writes content to a file, creating parent directories if needed." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "file_path": { "type": "string" },
                "content": { "type": "string" }
            },
            "required": ["file_path", "content"]
        })
    }
    async fn execute(&self, args: &Value) -> Result<Value> {
        let path_str = args["file_path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;
        let content = args["content"].as_str().ok_or_else(|| anyhow::anyhow!("Missing content"))?;
        
        let path = PathBuf::from(path_str);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(json!({ "status": "success" }))
    }
}

pub struct ListDirectoryTool;

#[async_trait]
impl Tool for ListDirectoryTool {
    fn name(&self) -> &'static str { "list_directory" }
    fn description(&self) -> &'static str { "Lists files in a directory." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "dir_path": { "type": "string" }
            }
        })
    }
    async fn execute(&self, args: &Value) -> Result<Value> {
        let path_str = args["dir_path"].as_str().unwrap_or(".");
        let mut entries = Vec::new();
        for entry in fs::read_dir(path_str)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
            let is_dir = path.is_dir();
            entries.push(json!({ "name": name, "is_dir": is_dir }));
        }
        Ok(json!({ "entries": entries }))
    }
}

pub struct GrepSearchTool;

#[async_trait]
impl Tool for GrepSearchTool {
    fn name(&self) -> &'static str { "grep_search" }
    fn description(&self) -> &'static str { "Searches for a pattern in files." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": { "type": "string" },
                "dir_path": { "type": "string" }
            },
            "required": ["pattern"]
        })
    }
    async fn execute(&self, args: &Value) -> Result<Value> {
        let pattern = args["pattern"].as_str().ok_or_else(|| anyhow::anyhow!("Missing pattern"))?;
        let dir_path = args["dir_path"].as_str().unwrap_or(".");
        
        let output = tokio::process::Command::new("grep")
            .arg("-rn")
            .arg(pattern)
            .arg(dir_path)
            .output()
            .await?;

        let results = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(json!({ "results": results }))
    }
}
