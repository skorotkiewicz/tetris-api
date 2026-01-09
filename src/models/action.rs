//! Player actions for controlling the game.

use serde::{Deserialize, Serialize};

/// Actions that a player can perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// Move piece left.
    Left,
    /// Move piece right.
    Right,
    /// Soft drop (move down, +1 point).
    Down,
    /// Rotate piece clockwise (classic NES only had one rotation direction).
    Rotate,
    /// Hard drop (instant drop and lock).
    Drop,
}

impl Action {
    /// Get the movement delta for this action.
    /// Returns (dx, dy) or None if not a movement action.
    pub fn movement_delta(&self) -> Option<(i32, i32)> {
        match self {
            Action::Left => Some((-1, 0)),
            Action::Right => Some((1, 0)),
            Action::Down => Some((0, 1)),
            _ => None,
        }
    }

    /// Check if this action is a rotation.
    pub fn is_rotation(&self) -> bool {
        matches!(self, Action::Rotate)
    }

    /// Check if this action is a hard drop.
    pub fn is_hard_drop(&self) -> bool {
        matches!(self, Action::Drop)
    }

    /// Check if this action awards soft drop points.
    pub fn is_soft_drop(&self) -> bool {
        matches!(self, Action::Down)
    }
}
