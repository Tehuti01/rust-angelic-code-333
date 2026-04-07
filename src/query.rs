use anyhow::Result;
use crate::context::{Context, Provider};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

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
        
        match ctx.provider {
            Provider::OpenRouter => self.chat_openrouter(prompt, ctx).await,
            Provider::Google => self.chat_google(prompt, ctx).await,
            Provider::Nvidia => self.chat_nvidia(prompt, ctx).await,
        }
    }

    async fn chat_openrouter(&self, prompt: &str, ctx: &mut Context) -> Result<String> {
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

        let json_res: serde_json::Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string();
        
        // Track usage (mocked for now, in a real API you'd use usage field from JSON)
        ctx.cost.add_usage(prompt.len() as u64 / 4, content.len() as u64 / 4);
        
        ctx.history.push(format!("Assistant (OpenRouter): {}", content));
        Ok(content)
    }

    async fn chat_google(&self, prompt: &str, ctx: &mut Context) -> Result<String> {
        let api_key = std::env::var("GOOGLE_AI_STUDIO_API_KEY")?;
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", ctx.model, api_key);
        let body = json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }]
        });

        let res = self.client.post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let json_res: serde_json::Value = res.json().await?;
        let content = json_res["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("No response").to_string();
        
        // Track usage
        ctx.cost.add_usage(prompt.len() as u64 / 4, content.len() as u64 / 4);
        
        ctx.history.push(format!("Assistant (Google): {}", content));
        Ok(content)
    }

    async fn chat_nvidia(&self, prompt: &str, ctx: &mut Context) -> Result<String> {
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

        let json_res: serde_json::Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string();
        
        // Track usage
        ctx.cost.add_usage(prompt.len() as u64 / 4, content.len() as u64 / 4);
        
        ctx.history.push(format!("Assistant (Nvidia): {}", content));
        Ok(content)
    }

    pub async fn execute_tool_call(&self, name: &str, args: &serde_json::Value, ctx: &mut Context) -> Result<serde_json::Value> {
        ctx.tools.call(name, args).await
    }
}
