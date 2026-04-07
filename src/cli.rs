use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "claude-code", author, version, about = "An interactive CLI agent")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize the project
    Init,
    /// Run a quick command and exit
    Brief { prompt: String },
    /// Clear history
    Clear,
    /// Login to Claude API
    Login,
    /// Logout from Claude API
    Logout,
    /// Output cost information
    Cost,
}
