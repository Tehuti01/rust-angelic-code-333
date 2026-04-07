use anyhow::Result;
use async_trait::async_trait;
use crate::tools::Tool;
use tokio::process::Command;

pub struct RunShellCommandTool;

#[async_trait]
impl Tool for RunShellCommandTool {
    fn name(&self) -> &'static str { "run_shell_command" }
    fn description(&self) -> &'static str { "Executes a shell command." }
    async fn execute(&self, args: &serde_json::Value) -> Result<serde_json::Value> {
        let command_str = args["command"].as_str().ok_or_else(|| anyhow::anyhow!("Missing command"))?;
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(command_str)
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code();

        Ok(serde_json::json!({
            "stdout": stdout,
            "stderr": stderr,
            "exit_code": exit_code,
        }))
    }
}
