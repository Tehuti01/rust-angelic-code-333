use anyhow::Result;
use async_trait::async_trait;
use crate::tools::Tool;
use glob::glob;
use serde_json::{json, Value};

pub struct GlobTool;

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &'static str { "glob" }
    fn description(&self) -> &'static str { "Finds files matching a glob pattern." }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": { "type": "string" }
            },
            "required": ["pattern"]
        })
    }
    async fn execute(&self, args: &Value) -> Result<Value> {
        let pattern = args["pattern"].as_str().ok_or_else(|| anyhow::anyhow!("Missing pattern"))?;
        let mut files = Vec::new();
        for entry in glob(pattern)? {
            match entry {
                Ok(path) => files.push(path.to_string_lossy().to_string()),
                Err(e) => return Err(anyhow::anyhow!("Glob error: {:?}", e)),
            }
        }
        Ok(json!({ "files": files }))
    }
}
