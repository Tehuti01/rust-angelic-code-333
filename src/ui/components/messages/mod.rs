use ratatui::widgets::ListItem;
use ratatui::style::{Color, Style};

pub struct MessageRenderer;

impl MessageRenderer {
    // Ported from components/messages/UserPromptMessage.tsx
    pub fn user_prompt(text: &str) -> ListItem {
        ListItem::new(format!("> {}", text)).style(Style::default().fg(Color::Cyan))
    }

    // Ported from components/messages/AssistantThinkingMessage.tsx
    pub fn thinking(text: &str) -> ListItem {
        ListItem::new(format!("[Thinking] {}", text)).style(Style::default().fg(Color::DarkGray))
    }

    // Ported from components/messages/UserToolResultMessage/UserToolSuccessMessage.tsx
    pub fn tool_success<'a>(name: &'a str, result: &'a str) -> ListItem<'a> {
        ListItem::new(format!("✅ Tool {}: {}", name, result)).style(Style::default().fg(Color::Green))
    }

    // Ported from components/messages/UserToolResultMessage/UserToolErrorMessage.tsx
    pub fn tool_error<'a>(name: &'a str, error: &'a str) -> ListItem<'a> {
        ListItem::new(format!("❌ Tool {}: {}", name, error)).style(Style::default().fg(Color::Red))
    }

    // Ported from components/messages/SystemTextMessage.tsx
    pub fn system(text: &str) -> ListItem {
        ListItem::new(format!("!!! {}", text)).style(Style::default().fg(Color::Yellow))
    }

    // Ported from components/messages/AssistantTextMessage.tsx
    pub fn assistant_text(text: &str) -> ListItem {
        ListItem::new(text).style(Style::default().fg(Color::White))
    }
}
