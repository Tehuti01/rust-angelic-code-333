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

pub mod components;

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
    let query_engine = crate::QueryEngine::QueryEngine::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Status Bar
                        Constraint::Min(1),    // Message History
                        Constraint::Length(3), // Prompt Input
                    ]
                    .as_ref(),
                )
                .split(f.size());

            components::StatusBar::render(f, chunks[0], ctx);
            components::MessageList::render(f, chunks[1], ctx);
            components::PromptInput::render(f, chunks[2], &input);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        let trimmed = input.trim();
                        if trimmed == "/exit" || trimmed == "/quit" {
                            return Ok(());
                        }
                        if trimmed.to_lowercase().starts_with("/model ") {
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 3 {
                                match parts[1].to_lowercase().as_str() {
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
