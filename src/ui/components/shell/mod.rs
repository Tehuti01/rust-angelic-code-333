use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct ShellRenderer;

impl ShellRenderer {
    // Ported from components/shell/OutputLine.tsx
    pub fn render_output(f: &mut Frame, area: Rect, output: &str) {
        let block = Paragraph::new(output)
            .block(Block::default().borders(Borders::ALL).title(" Shell Output "));
        f.render_widget(block, area);
    }
}
