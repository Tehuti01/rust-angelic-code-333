use anyhow::Result;
use crate::context::Context;
use crate::cli::Commands as CliCommands;
use async_trait::async_trait;

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn execute(&self, args: Vec<String>, ctx: &mut Context) -> Result<()>;
}

pub struct CommandRegistry {
    commands: std::collections::HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            commands: std::collections::HashMap::new(),
        };
        // Register commands as they are ported
        registry
    }

    pub fn register(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name().to_string(), command);
    }

    pub async fn execute(&self, name: &str, args: Vec<String>, ctx: &mut Context) -> Result<()> {
        let cmd = self.commands.get(name).ok_or_else(|| anyhow::anyhow!("Command not found: {}", name))?;
        cmd.execute(args, ctx).await
    }

    pub fn list_commands(&self) -> Vec<(&'static str, &'static str)> {
        let mut list: Vec<_> = self.commands.values().map(|c| (c.name(), c.description())).collect();
        list.sort_by_key(|c| c.0);
        list
    }
}

pub async fn handle_command(cmd: CliCommands, ctx: &mut Context) -> Result<()> {
    match cmd {
        CliCommands::Init => {
            println!("Initializing workspace...");
        }
        CliCommands::Brief { prompt } => {
            println!("Running brief query: {}", prompt);
        }
        CliCommands::Clear => {
            println!("Clearing history...");
            ctx.history.clear();
        }
        CliCommands::Login => {
            println!("Logging in...");
        }
        CliCommands::Logout => {
            println!("Logging out...");
        }
        CliCommands::Cost => {
            println!("Current cost: ${:.4}", ctx.cost.total_cost_usd);
        }
    }
    Ok(())
}
