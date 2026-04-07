use anyhow::Result;
use crate::context::{Context, Provider};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct QueryEngine {
    client: reqwest::Client,
}

impl QueryEngine {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn chat(&self, prompt: &str, ctx: &mut Context) -> Result<String> {
        ctx.history.push(format!("User: {}", prompt));
        
        // This is a simplified agent loop that supports tool calling
        let mut current_prompt = prompt.to_string();
        let mut loop_count = 0;

        loop {
            if loop_count > 10 { break; } // Safety break
            loop_count += 1;

            let response = match ctx.provider {
                Provider::OpenRouter => self.call_openrouter(&current_prompt, ctx).await?,
                Provider::Google => self.call_google(&current_prompt, ctx).await?,
                Provider::Nvidia => self.call_nvidia(&current_prompt, ctx).await?,
            };

            // Parse response for tool calls (simple mock for now, 
            // but structured for real JSON tool call parsing)
            if let Some(tool_call) = self.parse_tool_call(&response) {
                let name = tool_call["name"].as_str().unwrap_or_default();
                let args = &tool_call["args"];
                
                ctx.history.push(format!("Assistant [Thinking]: I need to use {}...", name));
                
                match ctx.tools.call(name, args).await {
                    Ok(result) => {
                        ctx.history.push(format!("Tool {}: Success", name));
                        current_prompt = format!("Tool result: {}", result);
                    }
                    Err(e) => {
                        ctx.history.push(format!("Tool {}: Error: {}", name, e));
                        current_prompt = format!("Tool error: {}", e);
                    }
                }
            } else {
                ctx.history.push(format!("Assistant: {}", response));
                return Ok(response);
            }
        }
        
        Ok("Loop limit reached".to_string())
    }

    fn parse_tool_call(&self, response: &str) -> Option<Value> {
        // In a real implementation, we would parse structured JSON tool calls.
        // For this demo/foundation, we look for a specific trigger pattern:
        // "CALL_TOOL: {"name": "...", "args": {...}}"
        if let Some(pos) = response.find("CALL_TOOL: ") {
            let json_part = &response[pos + 11..];
            if let Ok(val) = serde_json::from_str::<Value>(json_part) {
                return Some(val);
            }
        }
        None
    }

    async fn call_openrouter(&self, prompt: &str, ctx: &mut Context) -> Result<String> {
        let api_key = std::env::var("OPENROUTER_API_KEY")?;
        let body = json!({
            "model": ctx.model,
            "messages": [{"role": "user", "content": prompt}]
        });

        let res = self.client.post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;

        let json_res: Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string();
        ctx.cost.add_usage(prompt.len() as u64 / 4, content.len() as u64 / 4);
        Ok(content)
    }

    async fn call_google(&self, prompt: &str, ctx: &mut Context) -> Result<String> {
        let api_key = std::env::var("GOOGLE_AI_STUDIO_API_KEY")?;
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", ctx.model, api_key);
        let body = json!({
            "contents": [{ "parts": [{"text": prompt}] }]
        });

        let res = self.client.post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let json_res: Value = res.json().await?;
        let content = json_res["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("No response").to_string();
        ctx.cost.add_usage(prompt.len() as u64 / 4, content.len() as u64 / 4);
        Ok(content)
    }

    async fn call_nvidia(&self, prompt: &str, ctx: &mut Context) -> Result<String> {
        let api_key = std::env::var("NVIDIA_API_KEY")?;
        let body = json!({
            "model": ctx.model,
            "messages": [{"role": "user", "content": prompt}]
        });

        let res = self.client.post("https://integrate.api.nvidia.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;

        let json_res: Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string();
        ctx.cost.add_usage(prompt.len() as u64 / 4, content.len() as u64 / 4);
        Ok(content)
    }
}
