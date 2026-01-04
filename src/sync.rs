use std::sync::Arc;

use tokio::sync::watch;
use tokio::time::{interval, Duration};
use crate::state::PresenceState;
use crate::discord::Discord;

pub fn spawn_discord_sync(
    mut rx: watch::Receiver<PresenceState>,
    discord: Arc<Discord>,
) {
    tokio::spawn(async move {
        let mut last = rx.borrow().clone();

        if last.start_timestamp != 0 {
            discord.update(
                &last.details,
                &last.state,
                last.start_timestamp,
            );
        }

        let mut ticker = interval(Duration::from_secs(15));

        loop {
            tokio::select! {
                Ok(_) = rx.changed() => {
                    let current = rx.borrow().clone();

                    if current != last {
                        discord.update(
                            &current.details,
                            &current.state,
                            current.start_timestamp,
                        );
                        last = current;
                    }
                }

                _ = ticker.tick() => {
                    if last.start_timestamp != 0 {
                        discord.refresh(
                            &last.details,
                            &last.state,
                        );
                    }
                }
            }
        }
    });
}