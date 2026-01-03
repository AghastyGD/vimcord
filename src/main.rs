mod cli;
mod state;
mod discord;
mod server;
mod sync;

use cli::{Cli, Command};
use clap::Parser;
use state::PresenceState;
use tokio::sync::watch;
use discord::Discord;
use sync::spawn_discord_sync;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Daemon) {
        Command::Daemon => {
            let (tx, rx) = watch::channel(PresenceState::default());

            let client_id = std::env::var("DISCORD_CLIENT_ID")
                .expect("DISCORD_CLIENT_ID environment variable must be set")
                .parse::<u64>()
                .expect("DISCORD_CLIENT_ID must be a valid u64");

            let discord = Discord::new(client_id);

            spawn_discord_sync(rx, discord);
            
            let app = server::router(tx);

            println!("vimcord daemon running on :8787");

            let listener = tokio::net::TcpListener::bind("0.0.0.0:8787").await.unwrap();
            
            axum::serve(listener, app).await.expect("server crashed");
        }

        Command::Update { details, state, file, workspace } => {
            println!("CLI update")
        }

        Command::Clear => {
            println!("CLI clear")
        }
    }
}