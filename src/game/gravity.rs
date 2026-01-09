//! NES Tetris gravity (drop speed per level).

/// Frames per gravity drop for each level (NES 60fps).
/// Index = level (0-29+)
const FRAMES_PER_DROP: [u32; 30] = [
    48, // Level 0
    43, // Level 1
    38, // Level 2
    33, // Level 3
    28, // Level 4
    23, // Level 5
    18, // Level 6
    13, // Level 7
    8,  // Level 8
    6,  // Level 9
    5,  // Level 10
    5,  // Level 11
    5,  // Level 12
    4,  // Level 13
    4,  // Level 14
    4,  // Level 15
    3,  // Level 16
    3,  // Level 17
    3,  // Level 18
    2,  // Level 19
    2,  // Level 20
    2,  // Level 21
    2,  // Level 22
    2,  // Level 23
    2,  // Level 24
    2,  // Level 25
    2,  // Level 26
    2,  // Level 27
    2,  // Level 28
    1,  // Level 29+ (kill screen)
];

/// Get the number of frames between gravity drops for a level.
pub fn frames_per_drop(level: u32) -> u32 {
    let index = (level as usize).min(FRAMES_PER_DROP.len() - 1);
    FRAMES_PER_DROP[index]
}

/// Get the approximate drop speed in cells per second (at 60fps).
pub fn drops_per_second(level: u32) -> f64 {
    60.0 / frames_per_drop(level) as f64
}

/// Calculate milliseconds between gravity drops.
pub fn ms_per_drop(level: u32) -> u32 {
    let frames = frames_per_drop(level);
    // At 60fps, each frame is ~16.67ms
    (frames as f64 * (1000.0 / 60.0)).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_0_gravity() {
        assert_eq!(frames_per_drop(0), 48);
        assert_eq!(ms_per_drop(0), 800); // ~0.8 seconds per drop
    }

    #[test]
    fn test_level_29_gravity() {
        assert_eq!(frames_per_drop(29), 1);
        // 60 drops per second!
    }

    #[test]
    fn test_level_beyond_29() {
        // Levels beyond 29 should use level 29 speed
        assert_eq!(frames_per_drop(50), 1);
        assert_eq!(frames_per_drop(100), 1);
    }
}
