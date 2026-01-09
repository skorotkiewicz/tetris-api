//! Tetromino piece definitions with classic NES rotation system.

use rand::Rng;
use serde::{Deserialize, Serialize};

/// The 7 classic Tetromino types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceType {
    /// Get all piece types.
    pub const ALL: [PieceType; 7] = [
        PieceType::I,
        PieceType::O,
        PieceType::T,
        PieceType::S,
        PieceType::Z,
        PieceType::J,
        PieceType::L,
    ];

    /// Get a random piece type.
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::ALL[rng.gen_range(0..7)]
    }

    /// Get the shape cells for this piece at a given rotation.
    /// Returns a 4x4 grid where true = filled cell.
    /// Classic NES uses right-hand rotation system.
    pub fn cells(&self, rotation: u8) -> [[bool; 4]; 4] {
        let rotation = rotation % self.rotation_count();
        
        match self {
            PieceType::I => Self::rotate_cells(&I_SHAPES, rotation),
            PieceType::O => Self::rotate_cells(&O_SHAPES, rotation),
            PieceType::T => Self::rotate_cells(&T_SHAPES, rotation),
            PieceType::S => Self::rotate_cells(&S_SHAPES, rotation),
            PieceType::Z => Self::rotate_cells(&Z_SHAPES, rotation),
            PieceType::J => Self::rotate_cells(&J_SHAPES, rotation),
            PieceType::L => Self::rotate_cells(&L_SHAPES, rotation),
        }
    }

    /// Number of unique rotations for this piece.
    pub fn rotation_count(&self) -> u8 {
        match self {
            PieceType::O => 1, // Square doesn't rotate
            PieceType::I | PieceType::S | PieceType::Z => 2, // 2-state pieces
            PieceType::T | PieceType::J | PieceType::L => 4, // 4-state pieces
        }
    }

    /// Get spawn position (x, y) for classic NES Tetris.
    /// Pieces spawn at top-center of the board.
    pub fn spawn_position(&self) -> (i32, i32) {
        match self {
            PieceType::I => (3, 0),
            PieceType::O => (4, 0),
            _ => (3, 0),
        }
    }

    fn rotate_cells(shapes: &[[[bool; 4]; 4]], rotation: u8) -> [[bool; 4]; 4] {
        shapes[rotation as usize]
    }
}

// Classic NES piece shapes (4x4 grids)
// Each piece has its rotation states defined

const I_SHAPES: [[[bool; 4]; 4]; 2] = [
    // Horizontal
    [
        [false, false, false, false],
        [true,  true,  true,  true ],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // Vertical
    [
        [false, false, true,  false],
        [false, false, true,  false],
        [false, false, true,  false],
        [false, false, true,  false],
    ],
];

const O_SHAPES: [[[bool; 4]; 4]; 1] = [
    [
        [false, false, false, false],
        [false, true,  true,  false],
        [false, true,  true,  false],
        [false, false, false, false],
    ],
];

const T_SHAPES: [[[bool; 4]; 4]; 4] = [
    // Spawn (T pointing down)
    [
        [false, false, false, false],
        [true,  true,  true,  false],
        [false, true,  false, false],
        [false, false, false, false],
    ],
    // 90° CW
    [
        [false, true,  false, false],
        [true,  true,  false, false],
        [false, true,  false, false],
        [false, false, false, false],
    ],
    // 180°
    [
        [false, true,  false, false],
        [true,  true,  true,  false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // 270° CW
    [
        [false, true,  false, false],
        [false, true,  true,  false],
        [false, true,  false, false],
        [false, false, false, false],
    ],
];

const S_SHAPES: [[[bool; 4]; 4]; 2] = [
    // Horizontal
    [
        [false, false, false, false],
        [false, true,  true,  false],
        [true,  true,  false, false],
        [false, false, false, false],
    ],
    // Vertical
    [
        [true,  false, false, false],
        [true,  true,  false, false],
        [false, true,  false, false],
        [false, false, false, false],
    ],
];

const Z_SHAPES: [[[bool; 4]; 4]; 2] = [
    // Horizontal
    [
        [false, false, false, false],
        [true,  true,  false, false],
        [false, true,  true,  false],
        [false, false, false, false],
    ],
    // Vertical
    [
        [false, true,  false, false],
        [true,  true,  false, false],
        [true,  false, false, false],
        [false, false, false, false],
    ],
];

const J_SHAPES: [[[bool; 4]; 4]; 4] = [
    // Spawn
    [
        [false, false, false, false],
        [true,  true,  true,  false],
        [false, false, true,  false],
        [false, false, false, false],
    ],
    // 90° CW
    [
        [false, true,  false, false],
        [false, true,  false, false],
        [true,  true,  false, false],
        [false, false, false, false],
    ],
    // 180°
    [
        [true,  false, false, false],
        [true,  true,  true,  false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // 270° CW
    [
        [false, true,  true,  false],
        [false, true,  false, false],
        [false, true,  false, false],
        [false, false, false, false],
    ],
];

const L_SHAPES: [[[bool; 4]; 4]; 4] = [
    // Spawn
    [
        [false, false, false, false],
        [true,  true,  true,  false],
        [true,  false, false, false],
        [false, false, false, false],
    ],
    // 90° CW
    [
        [true,  true,  false, false],
        [false, true,  false, false],
        [false, true,  false, false],
        [false, false, false, false],
    ],
    // 180°
    [
        [false, false, true,  false],
        [true,  true,  true,  false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    // 270° CW
    [
        [false, true,  false, false],
        [false, true,  false, false],
        [false, true,  true,  false],
        [false, false, false, false],
    ],
];

/// An active piece on the board with position and rotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Piece {
    #[serde(rename = "type")]
    pub piece_type: PieceType,
    pub x: i32,
    pub y: i32,
    pub rotation: u8,
}

impl Piece {
    /// Create a new piece at spawn position.
    pub fn spawn(piece_type: PieceType) -> Self {
        let (x, y) = piece_type.spawn_position();
        Self {
            piece_type,
            x,
            y,
            rotation: 0,
        }
    }

    /// Get the cells of this piece at current rotation.
    pub fn cells(&self) -> [[bool; 4]; 4] {
        self.piece_type.cells(self.rotation)
    }

    /// Get all filled cell positions in board coordinates.
    pub fn board_cells(&self) -> Vec<(i32, i32)> {
        let cells = self.cells();
        let mut positions = Vec::new();
        
        for (dy, row) in cells.iter().enumerate() {
            for (dx, &filled) in row.iter().enumerate() {
                if filled {
                    positions.push((self.x + dx as i32, self.y + dy as i32));
                }
            }
        }
        
        positions
    }

    /// Move the piece by delta.
    pub fn moved(&self, dx: i32, dy: i32) -> Self {
        Self {
            piece_type: self.piece_type,
            x: self.x + dx,
            y: self.y + dy,
            rotation: self.rotation,
        }
    }

    /// Rotate the piece clockwise.
    pub fn rotated(&self) -> Self {
        Self {
            piece_type: self.piece_type,
            x: self.x,
            y: self.y,
            rotation: (self.rotation + 1) % self.piece_type.rotation_count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_spawn_positions() {
        let t = Piece::spawn(PieceType::T);
        assert_eq!(t.x, 3);
        assert_eq!(t.y, 0);

        let i = Piece::spawn(PieceType::I);
        assert_eq!(i.x, 3);
        assert_eq!(i.y, 0);

        let o = Piece::spawn(PieceType::O);
        assert_eq!(o.x, 4);
        assert_eq!(o.y, 0);
    }

    #[test]
    fn test_piece_rotation_count() {
        assert_eq!(PieceType::O.rotation_count(), 1);
        assert_eq!(PieceType::I.rotation_count(), 2);
        assert_eq!(PieceType::T.rotation_count(), 4);
    }

    #[test]
    fn test_piece_board_cells() {
        let t = Piece::spawn(PieceType::T);
        let cells = t.board_cells();
        // T piece in spawn rotation should have 4 cells
        assert_eq!(cells.len(), 4);
    }
}
