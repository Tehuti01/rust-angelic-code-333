use anyhow::Result;
use clap::Parser;

#[allow(non_snake_case)]
mod cli;
#[allow(non_snake_case)]
mod commands;
#[allow(non_snake_case)]
mod Context;
#[allow(non_snake_case)]
mod QueryEngine;
#[allow(non_snake_case)]
mod Tool;
#[allow(non_snake_case)]
mod ui;
#[allow(non_snake_case)]
mod Cost;
#[allow(non_snake_case)]
mod Task;
#[allow(non_snake_case)]
mod Types;
#[allow(non_snake_case)]
mod file;
#[allow(non_snake_case)]
mod shell;
#[allow(non_snake_case)]
mod replace;
#[allow(non_snake_case)]
mod glob;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env
    dotenv::dotenv().ok();
    
    // Initialize tracing (logging)
    tracing_subscriber::fmt::init();

    // Parse CLI arguments
    let args = cli::Cli::parse();

    // Initialize context
    let mut ctx = Context::Context::new().await?;

    // Handle CLI subcommands if present, or enter interactive mode
    if let Some(cmd) = args.command {
        commands::handle_command(cmd, &mut ctx).await?;
    } else {
        // Run Ratatui interactive UI loop
        ui::run_interactive_loop(&mut ctx).await?;
    }

    Ok(())
}
