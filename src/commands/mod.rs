use anyhow::Result;
use crate::cli::Commands;
use crate::Context::Context;

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
            println!("Current cost: ${:.4}", ctx.cost.total_cost_usd);
        }
    }
    Ok(())
}
