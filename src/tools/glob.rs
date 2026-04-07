use anyhow::Result;
use async_trait::async_trait;
use crate::tools::Tool;
use glob::glob;

pub struct GlobTool;

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &'static str { "glob" }
    fn description(&self) -> &'static str { "Finds files matching a glob pattern." }
    async fn execute(&self, args: &serde_json::Value) -> Result<serde_json::Value> {
        let pattern = args["pattern"].as_str().ok_or_else(|| anyhow::anyhow!("Missing pattern"))?;
        let mut files = Vec::new();
        for entry in glob(pattern)? {
            match entry {
                Ok(path) => files.push(path.to_string_lossy().to_string()),
                Err(e) => return Err(anyhow::anyhow!("Glob error: {:?}", e)),
            }
        }
        Ok(serde_json::json!({ "files": files }))
    }
}
