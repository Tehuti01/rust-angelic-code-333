use anyhow::Result;
use crate::cli::Commands;
use crate::context::Context;

pub async fn handle_command(cmd: Commands, ctx: &mut Context) -> Result<()> {
    match cmd {
        Commands::Init => {
            println!("Initializing workspace...");
        }
        Commands::Brief { prompt } => {
            println!("Running brief query: {}", prompt);
        }
        Commands::Clear => {
            println!("Clearing history...");
            ctx.history.clear();
        }
        Commands::Login => {
            println!("Logging in...");
        }
        Commands::Logout => {
            println!("Logging out...");
        }
        Commands::Cost => {
            println!("Current cost: $0.00");
        }
    }
    Ok(())
}
