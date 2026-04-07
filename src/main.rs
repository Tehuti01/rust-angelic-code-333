use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod context;
mod query_engine;
mod tools;
mod ui;
mod cost;
mod task;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env
    dotenv::dotenv().ok();
    
    // Initialize tracing (logging)
    tracing_subscriber::fmt::init();

    // Parse CLI arguments
    let args = cli::Cli::parse();

    // Initialize context
    let mut ctx = context::Context::new().await?;

    // Handle CLI subcommands if present, or enter interactive mode
    if let Some(cmd) = args.command {
        commands::handle_command(cmd, &mut ctx).await?;
    } else {
        // Run Ratatui interactive UI loop
        ui::run_interactive_loop(&mut ctx).await?;
    }

    Ok(())
}
