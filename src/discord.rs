use discord_rpc_client::Client as DiscordClient;
use std::sync::{Arc, Mutex};

pub struct Discord {
    client: Arc<Mutex<DiscordClient>>,
}

impl Discord {
    pub fn new(client_id: u64) -> Self {
        let mut client = DiscordClient::new(client_id);
        client.start();
        Self { client: Arc::new(Mutex::new(client)) }
    }

    pub fn update(&self, details: &str, state: &str) {
        if let Ok(mut client) = self.client.lock() {
            let _ = client.set_activity(|a| {
            a.details(details)
             .state(state)
        });
        }

    }

    pub fn clear(&self) {
        if let Ok(mut client) = self.client.lock() {
            let _ = client.clear_activity();
        } 
        
    }
}