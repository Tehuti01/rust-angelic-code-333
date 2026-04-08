use anyhow::Result;
use crate::context::Context;
use crate::types::{Message, Role, ToolCall};
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
        ctx.messages.push(Message {
            role: Role::User,
            content: prompt.to_string(),
            tool_calls: None,
            tool_use_id: None,
        });
        
        let mut loop_count = 0;
        let mut last_response = String::new();

        loop {
            if loop_count > 10 { break; }
            loop_count += 1;

            let response = match ctx.provider {
                crate::context::Provider::OpenRouter => self.call_openrouter(ctx).await?,
                crate::context::Provider::Google => self.call_google(ctx).await?,
                crate::context::Provider::Nvidia => self.call_nvidia(ctx).await?,
            };

            // Enhanced Tool Call Detection
            if let Some(tool_calls) = self.extract_tool_calls(&response) {
                ctx.messages.push(Message {
                    role: Role::Assistant,
                    content: response.clone(),
                    tool_calls: Some(tool_calls.clone()),
                    tool_use_id: None,
                });

                for tc in tool_calls {
                    ctx.history.push(format!("Assistant [Tool Use]: calling {}...", tc.name));
                    
                    match ctx.tools.call(&tc.name, &tc.arguments).await {
                        Ok(result) => {
                            ctx.history.push(format!("Tool {}: Success", tc.name));
                            ctx.messages.push(Message {
                                role: Role::Tool,
                                content: result.to_string(),
                                tool_calls: None,
                                tool_use_id: Some(tc.id),
                            });
                        }
                        Err(e) => {
                            ctx.history.push(format!("Tool {}: Error: {}", tc.name, e));
                            ctx.messages.push(Message {
                                role: Role::Tool,
                                content: format!("Error: {}", e),
                                tool_calls: None,
                                tool_use_id: Some(tc.id),
                            });
                        }
                    }
                }
            } else {
                last_response = response.clone();
                ctx.messages.push(Message {
                    role: Role::Assistant,
                    content: response,
                    tool_calls: None,
                    tool_use_id: None,
                });
                ctx.history.push(format!("Assistant: {}", last_response));
                break;
            }
        }
        
        Ok(last_response)
    }

    fn extract_tool_calls(&self, response: &str) -> Option<Vec<ToolCall>> {
        if let Some(pos) = response.find("CALL_TOOL: ") {
            let json_part = &response[pos + 11..];
            if let Ok(val) = serde_json::from_str::<Value>(json_part) {
                let tc = ToolCall {
                    id: val["id"].as_str().unwrap_or("default").to_string(),
                    name: val["name"].as_str().unwrap_or_default().to_string(),
                    arguments: val["args"].clone(),
                };
                return Some(vec![tc]);
            }
        }
        None
    }

    async fn call_openrouter(&self, ctx: &Context) -> Result<String> {
        let api_key = std::env::var("OPENROUTER_API_KEY")?;
        let body = json!({
            "model": ctx.model,
            "messages": ctx.messages
        });

        let res = self.client.post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;

        let json_res: Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string();
        Ok(content)
    }

    async fn call_google(&self, ctx: &Context) -> Result<String> {
        let api_key = std::env::var("GOOGLE_AI_STUDIO_API_KEY")?;
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", ctx.model, api_key);
        
        // Map our messages to Google format
        let contents: Vec<Value> = ctx.messages.iter().filter_map(|m| {
            match m.role {
                Role::User => Some(json!({ "role": "user", "parts": [{ "text": m.content }] })),
                Role::Assistant => Some(json!({ "role": "model", "parts": [{ "text": m.content }] })),
                _ => None, // Simplified for now
            }
        }).collect();

        let body = json!({ "contents": contents });

        let res = self.client.post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let json_res: Value = res.json().await?;
        let content = json_res["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("No response").to_string();
        Ok(content)
    }

    async fn call_nvidia(&self, ctx: &Context) -> Result<String> {
        let api_key = std::env::var("NVIDIA_API_KEY")?;
        let body = json!({
            "model": ctx.model,
            "messages": ctx.messages
        });

        let res = self.client.post("https://integrate.api.nvidia.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await?;

        let json_res: Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"].as_str().unwrap_or("No response").to_string();
        Ok(content)
    }
}
