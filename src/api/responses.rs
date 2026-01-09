//! API request and response types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{Action, GameEvent, GameState};

/// Request to perform an action.
#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    pub action: Action,
}

/// Response after performing an action or tick.
#[derive(Debug, Serialize)]
pub struct ActionResponse {
    pub success: bool,
    pub state: GameState,
    pub events: Vec<GameEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_cleared_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_earned: Option<u32>,
}

/// Health check response.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub active_games: usize,
}

/// Game info for listing.
#[derive(Debug, Serialize)]
pub struct GameInfo {
    pub session_id: Uuid,
    pub score: u32,
    pub level: u32,
    pub lines: u32,
    pub status: String,
}
