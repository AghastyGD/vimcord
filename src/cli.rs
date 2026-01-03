use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vimcord")]
#[command(about = "Editor-agnostic Discord Rich Presence daemon")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start daemon
    Daemon,

    /// Update presence (CLI client mode)
    Update {
        #[arg(long)]
        details: Option<String>,

        #[arg(long)]
        state: Option<String>,

        #[arg(long)]
        file: Option<String>,

        #[arg(long)]
        workspace: Option<String>,
    },

    /// Clear presence
    Clear,
}