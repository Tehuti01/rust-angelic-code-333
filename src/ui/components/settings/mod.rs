use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::Frame;
use crate::Config::Config;

pub struct SettingsRenderer;

impl SettingsRenderer {
    // Ported from components/Settings/Config.tsx
    pub fn render_config(f: &mut Frame, area: Rect, config: &Config) {
        let text = format!(
            " Theme: {}\n Auto Mode: {}\n Model: {}",
            config.theme,
            config.auto_mode_opt_in,
            config.secondary_model.as_deref().unwrap_or("Default")
        );
        let block = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(" Configuration "));
        f.render_widget(block, area);
    }
}
