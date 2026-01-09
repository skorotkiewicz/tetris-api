//! Classic NES Tetris scoring system.

/// Points awarded for line clears (NES original).
/// Formula: POINTS[lines] × (level + 1)
const LINE_CLEAR_POINTS: [u32; 5] = [
    0,    // 0 lines
    40,   // 1 line (Single)
    100,  // 2 lines (Double)
    300,  // 3 lines (Triple)
    1200, // 4 lines (Tetris)
];

/// Calculate points for clearing lines.
/// 
/// # Arguments
/// * `lines` - Number of lines cleared (1-4)
/// * `level` - Current level
/// 
/// # Returns
/// Points awarded
pub fn calculate_line_clear_points(lines: usize, level: u32) -> u32 {
    let lines = lines.min(4); // Cap at 4 (Tetris)
    LINE_CLEAR_POINTS[lines] * (level + 1)
}

/// Points for soft drop (moving piece down manually).
pub const SOFT_DROP_POINTS: u32 = 1;

/// Calculate new level based on lines cleared.
/// Level increases every 10 lines.
/// 
/// # Arguments
/// * `start_level` - The level the game started at
/// * `total_lines` - Total lines cleared in the game
/// 
/// # Returns
/// Current level
pub fn calculate_level(start_level: u32, total_lines: u32) -> u32 {
    start_level + (total_lines / 10)
}

/// Get the name for a line clear.
pub fn line_clear_name(lines: usize) -> &'static str {
    match lines {
        1 => "Single",
        2 => "Double",
        3 => "Triple",
        4 => "Tetris",
        _ => "None",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_level_0() {
        assert_eq!(calculate_line_clear_points(1, 0), 40);
    }

    #[test]
    fn test_tetris_level_0() {
        assert_eq!(calculate_line_clear_points(4, 0), 1200);
    }

    #[test]
    fn test_tetris_level_5() {
        assert_eq!(calculate_line_clear_points(4, 5), 7200);
    }

    #[test]
    fn test_tetris_level_19() {
        assert_eq!(calculate_line_clear_points(4, 19), 24000);
    }

    #[test]
    fn test_level_progression() {
        assert_eq!(calculate_level(0, 0), 0);
        assert_eq!(calculate_level(0, 9), 0);
        assert_eq!(calculate_level(0, 10), 1);
        assert_eq!(calculate_level(0, 25), 2);
        assert_eq!(calculate_level(5, 30), 8);
    }
}
