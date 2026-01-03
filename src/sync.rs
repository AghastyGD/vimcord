use tokio::sync::watch;
use crate::state::PresenceState;
use crate::discord::Discord;

pub fn spawn_discord_sync (
    mut rx: watch::Receiver<PresenceState>,
    discord: Discord,
) {
    tokio::spawn(async move {
        let mut last = PresenceState::default();

        loop {
            if rx.changed().await.is_err() {
                break;
            }

            let current = rx.borrow().clone();

            if current != last {
                discord.update(
                    &current.details,
                    &current.state,
                );

                last = current;
            }
        }
    });
}