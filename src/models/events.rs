//! Game events that occur during gameplay.

use serde::{Deserialize, Serialize};

/// Events that can occur during a game tick or action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameEvent {
    /// Piece moved (left, right, or down).
    PieceMoved,
    /// Piece rotated.
    PieceRotated,
    /// Piece dropped by gravity.
    PieceDropped,
    /// Piece locked into place.
    PieceLocked,
    /// Soft drop point awarded.
    SoftDropPoint,
    /// Lines were cleared.
    LinesCleared,
    /// Level increased.
    LevelUp,
    /// New piece spawned.
    NewPieceSpawned,
    /// Game ended (topped out).
    GameOver,
}
