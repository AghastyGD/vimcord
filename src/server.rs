use tokio::sync::watch;
use axum::{Json, Router, routing::post};
use crate::state::PresenceState;

pub fn router(tx: watch::Sender<PresenceState>) -> Router {
    Router::new()
        .route("/update", post(update))
        .route("/clear", post(clear))
        .with_state(tx)
}

async fn update(
    axum::extract::State(tx): axum::extract::State<watch::Sender<PresenceState>>,
    Json(payload): Json<PresenceState>,
) -> &'static str {
    let _ = tx.send(payload);
    "ok"
}

async fn clear(
    axum::extract::State(tx): axum::extract::State<watch::Sender<PresenceState>>,
) -> &'static str {
    let _ = tx.send(PresenceState::default());
    "cleared"
}

