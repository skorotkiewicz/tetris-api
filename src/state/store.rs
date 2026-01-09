//! In-memory game state storage.

use dashmap::DashMap;
use uuid::Uuid;

use crate::models::Game;

/// Thread-safe game store using DashMap.
pub type GameStore = DashMap<Uuid, Game>;

/// Create a new game store.
pub fn new_game_store() -> GameStore {
    DashMap::new()
}
