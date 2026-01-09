//! Game state model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    board::Board,
    piece::{Piece, PieceType},
};

/// The status of a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    /// Game is in progress.
    Playing,
    /// Game is paused.
    Paused,
    /// Game ended (player topped out).
    GameOver,
}

/// Complete game state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// The game board.
    pub board: Board,
    /// Currently falling piece.
    pub current_piece: Piece,
    /// Next piece to spawn.
    pub next_piece: PieceType,
    /// Current score.
    pub score: u32,
    /// Current level.
    pub level: u32,
    /// Total lines cleared.
    pub lines: u32,
    /// Game status.
    pub status: GameStatus,
}

impl GameState {
    /// Create a new game state with starting level.
    pub fn new(start_level: u32) -> Self {
        let current_piece = Piece::spawn(PieceType::random());
        let next_piece = PieceType::random();

        Self {
            board: Board::new(),
            current_piece,
            next_piece,
            score: 0,
            level: start_level.min(19), // NES max start level is 19
            lines: 0,
            status: GameStatus::Playing,
        }
    }
}

/// A game session with metadata.
#[derive(Debug, Clone, Serialize)]
pub struct Game {
    /// Unique session ID.
    pub session_id: Uuid,
    /// The game state.
    pub state: GameState,
    /// When the game was created.
    pub created_at: DateTime<Utc>,
    /// Last activity time.
    pub last_activity: DateTime<Utc>,
    /// Starting level (for score calculation).
    pub start_level: u32,
}

impl Game {
    /// Create a new game session.
    pub fn new(start_level: u32) -> Self {
        let now = Utc::now();
        Self {
            session_id: Uuid::new_v4(),
            state: GameState::new(start_level),
            created_at: now,
            last_activity: now,
            start_level,
        }
    }

    /// Update last activity timestamp.
    pub fn touch(&mut self) {
        self.last_activity = Utc::now();
    }

    /// Get game duration in seconds.
    pub fn duration_seconds(&self) -> i64 {
        (self.last_activity - self.created_at).num_seconds()
    }

    /// Check if the game is still active (not game over).
    pub fn is_active(&self) -> bool {
        self.state.status == GameStatus::Playing || self.state.status == GameStatus::Paused
    }
}

/// Request to create a new game.
#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    /// Starting level (0-19).
    #[serde(default)]
    pub start_level: u32,
}

/// Response when creating a new game.
#[derive(Debug, Serialize)]
pub struct CreateGameResponse {
    pub session_id: Uuid,
    pub token: String,
    pub state: GameState,
}

/// Response for getting game state.
#[derive(Debug, Serialize)]
pub struct GameStateResponse {
    pub session_id: Uuid,
    pub state: GameState,
}

/// Response when game ends.
#[derive(Debug, Serialize)]
pub struct GameOverResponse {
    pub final_score: u32,
    pub final_level: u32,
    pub lines_cleared: u32,
    pub duration_seconds: i64,
}
