//! Classic Tetris board (10×20 playfield).

use serde::{Deserialize, Serialize};

use super::piece::Piece;

/// Board dimensions (classic NES Tetris).
pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

/// The game board - a 10×20 grid.
/// Each cell is 0 (empty) or 1-7 (piece color/type).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    /// Grid cells [row][col], row 0 is top.
    cells: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /// Create an empty board.
    pub fn new() -> Self {
        Self {
            cells: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    /// Get the cells grid.
    pub fn cells(&self) -> &[[u8; BOARD_WIDTH]; BOARD_HEIGHT] {
        &self.cells
    }

    /// Check if a cell is empty.
    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= BOARD_WIDTH as i32 || y < 0 || y >= BOARD_HEIGHT as i32 {
            return false; // Out of bounds = not empty
        }
        self.cells[y as usize][x as usize] == 0
    }

    /// Check if a position is within bounds.
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < BOARD_WIDTH as i32 && y >= 0 && y < BOARD_HEIGHT as i32
    }

    /// Check if a piece can be placed at its current position.
    pub fn can_place(&self, piece: &Piece) -> bool {
        for (x, y) in piece.board_cells() {
            // Allow y < 0 for pieces spawning above the board
            if x < 0 || x >= BOARD_WIDTH as i32 || y >= BOARD_HEIGHT as i32 {
                return false;
            }
            // Only check collision if the cell is within the visible board
            if y >= 0 && self.cells[y as usize][x as usize] != 0 {
                return false;
            }
        }
        true
    }

    /// Lock a piece onto the board.
    /// Returns the rows that were filled (for line clearing).
    pub fn lock_piece(&mut self, piece: &Piece) -> Vec<usize> {
        let piece_value = Self::piece_type_to_value(piece.piece_type);
        let mut affected_rows = Vec::new();

        for (x, y) in piece.board_cells() {
            if y >= 0 && y < BOARD_HEIGHT as i32 && x >= 0 && x < BOARD_WIDTH as i32 {
                self.cells[y as usize][x as usize] = piece_value;
                if !affected_rows.contains(&(y as usize)) {
                    affected_rows.push(y as usize);
                }
            }
        }

        affected_rows
    }

    /// Clear completed lines and return count of lines cleared.
    pub fn clear_lines(&mut self) -> usize {
        let mut lines_cleared = 0;
        let mut y = BOARD_HEIGHT as i32 - 1;

        while y >= 0 {
            if self.is_row_full(y as usize) {
                self.remove_row(y as usize);
                lines_cleared += 1;
                // Don't decrement y, check the same row again (new row dropped down)
            } else {
                y -= 1;
            }
        }

        lines_cleared
    }

    /// Check if a row is completely filled.
    fn is_row_full(&self, row: usize) -> bool {
        self.cells[row].iter().all(|&cell| cell != 0)
    }

    /// Remove a row and drop all rows above it.
    fn remove_row(&mut self, row: usize) {
        // Move all rows above down by one
        for y in (1..=row).rev() {
            self.cells[y] = self.cells[y - 1];
        }
        // Clear the top row
        self.cells[0] = [0; BOARD_WIDTH];
    }

    /// Check if the board has any cells in the top rows (game over condition).
    pub fn is_topped_out(&self) -> bool {
        // Check if any cells in the top 2 rows are filled
        self.cells[0].iter().any(|&c| c != 0) || self.cells[1].iter().any(|&c| c != 0)
    }

    /// Convert piece type to cell value (for coloring).
    fn piece_type_to_value(piece_type: super::piece::PieceType) -> u8 {
        use super::piece::PieceType;
        match piece_type {
            PieceType::I => 1,
            PieceType::O => 2,
            PieceType::T => 3,
            PieceType::S => 4,
            PieceType::Z => 5,
            PieceType::J => 6,
            PieceType::L => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::piece::PieceType;

    #[test]
    fn test_empty_board() {
        let board = Board::new();
        for row in board.cells() {
            for &cell in row {
                assert_eq!(cell, 0);
            }
        }
    }

    #[test]
    fn test_can_place_empty_board() {
        let board = Board::new();
        let piece = Piece::spawn(PieceType::T);
        assert!(board.can_place(&piece));
    }

    #[test]
    fn test_out_of_bounds() {
        let board = Board::new();
        let mut piece = Piece::spawn(PieceType::T);
        piece.x = -5;
        assert!(!board.can_place(&piece));
    }

    #[test]
    fn test_line_clear() {
        let mut board = Board::new();
        // Fill bottom row
        for x in 0..BOARD_WIDTH {
            board.cells[BOARD_HEIGHT - 1][x] = 1;
        }
        let cleared = board.clear_lines();
        assert_eq!(cleared, 1);
        // Bottom row should now be empty
        assert!(board.cells[BOARD_HEIGHT - 1].iter().all(|&c| c == 0));
    }
}
