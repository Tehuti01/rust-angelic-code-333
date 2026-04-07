use anyhow::Result;
use crate::context::Context;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};

pub async fn run_interactive_loop(ctx: &mut Context) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run loop
    let res = run_app(&mut terminal, ctx).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, ctx: &mut Context) -> Result<()> {
    let mut input = String::new();
    let query_engine = crate::query::QueryEngine::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Status Bar
                        Constraint::Min(1),    // History
                        Constraint::Length(3), // Input
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // 1. Status Bar
            let provider_str = match ctx.provider {
                crate::context::Provider::OpenRouter => "OpenRouter",
                crate::context::Provider::Google => "Google",
                crate::context::Provider::Nvidia => "Nvidia",
            };
            let status_text = format!(
                " Provider: {} | Model: {} | {}",
                provider_str,
                ctx.model,
                ctx.cost.display_string()
            );
            let status_block = Paragraph::new(status_text)
                .block(Block::default().title(" Status ").borders(Borders::ALL));
            f.render_widget(status_block, chunks[0]);

            // 2. History
            let history_text = ctx.history.join("\n");
            let history_block = Paragraph::new(history_text)
                .block(Block::default().title(" Conversation History ").borders(Borders::ALL));
            f.render_widget(history_block, chunks[1]);

            // 3. Input
            let input_block = Paragraph::new(format!("> {}", input))
                .block(Block::default().title(" Input (Type /exit to quit, /model to switch) ").borders(Borders::ALL));
            f.render_widget(input_block, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        let trimmed = input.trim();
                        if trimmed == "/exit" || trimmed == "/quit" {
                            return Ok(());
                        }
                        if trimmed.starts_with("/model ") {
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 3 {
                                match parts[1] {
                                    "openrouter" => {
                                        ctx.provider = crate::context::Provider::OpenRouter;
                                        ctx.model = parts[2].to_string();
                                        ctx.history.push(format!("Model switched to OpenRouter: {}", ctx.model));
                                    }
                                    "google" => {
                                        ctx.provider = crate::context::Provider::Google;
                                        ctx.model = parts[2].to_string();
                                        ctx.history.push(format!("Model switched to Google: {}", ctx.model));
                                    }
                                    "nvidia" => {
                                        ctx.provider = crate::context::Provider::Nvidia;
                                        ctx.model = parts[2].to_string();
                                        ctx.history.push(format!("Model switched to Nvidia: {}", ctx.model));
                                    }
                                    _ => ctx.history.push("Unknown provider. Use: openrouter, google, or nvidia.".to_string()),
                                }
                            } else {
                                ctx.history.push("Usage: /model <provider> <model_name>".to_string());
                            }
                            input.clear();
                            continue;
                        }
                        if !trimmed.is_empty() {
                            let prompt = input.clone();
                            input.clear();
                            // Run the query engine chat
                            query_engine.chat(&prompt, ctx).await?;
                        }
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}
