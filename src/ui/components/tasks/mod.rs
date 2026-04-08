use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::Rect;
use ratatui::Frame;
use crate::Task::{TaskState, TaskStatus};

pub struct TaskRenderer;

impl TaskRenderer {
    // Ported from components/tasks/BackgroundTaskStatus.tsx
    pub fn render_status(f: &mut Frame, area: Rect, task: &TaskState) {
        let status_str = match task.status {
            TaskStatus::Pending => "Pending",
            TaskStatus::Running => "Running",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
            TaskStatus::Killed => "Killed",
        };
        let text = format!(" Task: {} | Status: {}", task.description, status_str);
        let block = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(" Task Progress "));
        f.render_widget(block, area);
    }

    // Ported from components/tasks/ShellProgress.tsx
    pub fn shell_progress(command: &str, output: &str) -> String {
        format!("Executing: {}\nOutput: {}", command, output)
    }
}
