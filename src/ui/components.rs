pub mod messages;
pub mod tasks;
pub mod settings;
pub mod shell;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Frame,
};
use crate::context::Context;

pub struct StatusBar;

impl StatusBar {
    pub fn render(f: &mut Frame, area: Rect, ctx: &Context) {
        let provider_str = match ctx.provider {
            crate::context::Provider::OpenRouter => "OpenRouter",
            crate::context::Provider::Google => "Google",
            crate::context::Provider::Nvidia => "Nvidia",
        };
        let text = format!(
            " [ {} | {} ] | {}",
            provider_str,
            ctx.model,
            ctx.cost.display_string()
        );
        let block = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(" Agent Status "));
        f.render_widget(block, area);
    }
}

pub struct MessageList;

impl MessageList {
    pub fn render(f: &mut Frame, area: Rect, ctx: &Context) {
        let items: Vec<ListItem> = ctx.history.iter().map(|h: &String| {
            ListItem::new(h.as_str())
        }).collect();
        
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Conversation "));
        f.render_widget(list, area);
    }
}

pub struct PromptInput;

impl PromptInput {
    pub fn render(f: &mut Frame, area: Rect, input: &str) {
        let block = Paragraph::new(format!("> {}", input))
            .block(Block::default().borders(Borders::ALL).title(" Input "));
        f.render_widget(block, area);
    }
}
