use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceState {
    pub details: String,
    pub state: String,
    pub start_timestamp: u64,
    pub file: Option<String>,
    pub workspace: Option<String>,
}

impl Default for PresenceState {
    fn default() -> Self {
        Self {
            details: "In editor".into(),
            state: "Idle".into(),
            start_timestamp: 0,
            file: None,
            workspace: None,
        }
    }
}