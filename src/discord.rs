use discord_rpc_client::Client as DiscordClient;
use std::sync::Mutex;

pub struct Discord {
    client: Mutex<DiscordClient>,
}

impl Discord {
    pub fn new(client_id: u64) -> Self {
        let mut client = DiscordClient::new(client_id);
        client.start();

        Self {
            client: Mutex::new(client),
        }
    }

    pub fn update(
        &self,
        details: &str,
        state: &str,
        start_timestamp: u64,
    ) {
        if let Ok(mut client) = self.client.lock() {
            let _ = client.set_activity(|a| {
                a.details(details)
                 .state(state)
                 .timestamps(|t| t.start(start_timestamp))
            });
        }
    }

    pub fn refresh(
        &self,
        details: &str,
        state: &str,
    ) {
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
