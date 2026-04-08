use anyhow::Result;
use clap::Parser;
use claude_code_rs::{cli, Commands, context, ui};

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
        Commands::handle_command(cmd, &mut ctx).await?;
    } else {
        // Run Ratatui interactive UI loop
        ui::run_interactive_loop(&mut ctx).await?;
    }

    Ok(())
}
