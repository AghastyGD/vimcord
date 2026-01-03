mod cli;
mod state;
mod discord;
mod server;
mod sync;
mod net;

use cli::{Cli, Command};
use clap::Parser;
use state::PresenceState;
use tokio::sync::watch;
use discord::Discord;
use sync::spawn_discord_sync;
use net::resolve_host;
use reqwest::Client;


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
            run_cli_update(details, state, file, workspace).await;
        }

        Command::Clear => {
            run_cli_clear().await;
        }
    }
}

async fn run_cli_update(
    details: Option<String>,
    state: Option<String>,
    file: Option<String>,
    workspace: Option<String>,
) {
    let host = resolve_host();
    let url = format!("http://{}:8787/update", host);

    let payload = PresenceState {
        details: details.unwrap_or_else(|| "In Editor".into()),
        state: state.unwrap_or_else(|| "Editing".into()),
        file,
        workspace,
    };

    let client = Client::new();
    match client.post(url).json(&payload).send().await {
        Ok(_) => println!("vimcord: presence updated"),
        Err(e) => eprintln!("vimcord error: {}", e),
    }
}

async fn run_cli_clear() {
    let host = resolve_host();
    let url = format!("http://{}:8787/clear", host);

    let client = Client::new();
    match client.post(url).send().await {
        Ok(_) => println!("vimcord: presence cleared"),
        Err(e) => eprintln!("vimcord error: {}", e)
    }
}