use axum::{Json, Router, routing::post};
use std::sync::{Arc, Mutex};
use crate::state::PresenceState;

pub fn router(state: Arc<Mutex<PresenceState>>) -> Router {
    Router::new()
        .route("/update", post(update))
        .route("/clear", post(clear))
        .with_state(state)
}

async fn update(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<PresenceState>>>,
    Json(payload): Json<PresenceState>,
) -> &'static str {
    let mut s = state.lock().unwrap();
    *s = payload;
    "ok"
}

async fn clear(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<PresenceState>>>,
) -> &'static str {
    let mut s = state.lock().unwrap();
    *s = PresenceState::default();
    "cleared"
}

