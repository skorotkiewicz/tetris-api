//! Application state management.

pub mod store;

use std::sync::Arc;

pub use store::{new_game_store, GameStore};

use crate::auth::JwtService;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    /// JWT service for authentication.
    pub jwt: Arc<JwtService>,
    /// In-memory game store.
    pub games: Arc<GameStore>,
}

impl AppState {
    /// Create new application state.
    pub fn new(jwt_secret: &str, token_duration_hours: i64) -> Self {
        Self {
            jwt: Arc::new(JwtService::new(jwt_secret, token_duration_hours)),
            games: Arc::new(new_game_store()),
        }
    }

    /// Get number of active games.
    pub fn active_games_count(&self) -> usize {
        self.games.len()
    }
}
