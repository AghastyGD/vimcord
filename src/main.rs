mod cli;
mod state;
mod discord;
mod server;

use cli::{Cli, Command};
use clap::Parser;
use std::sync::{Arc, Mutex};
use state::PresenceState;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Daemon) {
        Command::Daemon => {
            let state = Arc::new(Mutex::new(PresenceState::default()));

            let app = server::router(state.clone());

            println!("vimcord daemon running on :8787");

            let listener = tokio::net::TcpListener::bind("0.0.0.0:8787").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }

        Command::Update { details, state, file, workspace } => {
            println!("CLI update")
        }

        Command::Clear => {
            println!("CLI clear")
        }
    }
}