use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceState {
    pub details: String,
    pub state: String,
    pub file: Option<String>,
    pub workspace: Option<String>,
}

impl Default for PresenceState {
    fn default() -> Self {
        Self {
            details: "In editor".into(),
            state: "Idle".into(),
            file: None,
            workspace: None,
        }
    }
}