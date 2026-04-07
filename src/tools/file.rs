use anyhow::Result;
use async_trait::async_trait;
use crate::tools::Tool;
use std::fs;
use std::path::PathBuf;

pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &'static str { "read_file" }
    fn description(&self) -> &'static str { "Reads the content of a file." }
    async fn execute(&self, args: &serde_json::Value) -> Result<serde_json::Value> {
        let path_str = args["file_path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;
        let content = fs::read_to_string(path_str)?;
        Ok(serde_json::json!({ "content": content }))
    }
}

pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &'static str { "write_file" }
    fn description(&self) -> &'static str { "Writes content to a file, creating parent directories if needed." }
    async fn execute(&self, args: &serde_json::Value) -> Result<serde_json::Value> {
        let path_str = args["file_path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;
        let content = args["content"].as_str().ok_or_else(|| anyhow::anyhow!("Missing content"))?;
        
        let path = PathBuf::from(path_str);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(serde_json::json!({ "status": "success" }))
    }
}

pub struct ListDirectoryTool;

#[async_trait]
impl Tool for ListDirectoryTool {
    fn name(&self) -> &'static str { "list_directory" }
    fn description(&self) -> &'static str { "Lists files in a directory." }
    async fn execute(&self, args: &serde_json::Value) -> Result<serde_json::Value> {
        let path_str = args["dir_path"].as_str().unwrap_or(".");
        let mut entries = Vec::new();
        for entry in fs::read_dir(path_str)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
            let is_dir = path.is_dir();
            entries.push(serde_json::json!({ "name": name, "is_dir": is_dir }));
        }
        Ok(serde_json::json!({ "entries": entries }))
    }
}

pub struct GrepSearchTool;

#[async_trait]
impl Tool for GrepSearchTool {
    fn name(&self) -> &'static str { "grep_search" }
    fn description(&self) -> &'static str { "Searches for a pattern in files." }
    async fn execute(&self, args: &serde_json::Value) -> Result<serde_json::Value> {
        let pattern = args["pattern"].as_str().ok_or_else(|| anyhow::anyhow!("Missing pattern"))?;
        let dir_path = args["dir_path"].as_str().unwrap_or(".");
        
        // Using ripgrep if available, or a simple internal search
        let output = tokio::process::Command::new("grep")
            .arg("-rn")
            .arg(pattern)
            .arg(dir_path)
            .output()
            .await?;

        let results = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(serde_json::json!({ "results": results }))
    }
}

