//! Core game engine implementing Tetris logic.

use crate::models::{Action, Game, GameEvent, GameStatus, Piece, PieceType};

use super::scoring::{calculate_level, calculate_line_clear_points, SOFT_DROP_POINTS};

/// Result of a game action or tick.
#[derive(Debug)]
pub struct ActionResult {
    /// Whether the action was successful.
    pub success: bool,
    /// Events that occurred.
    pub events: Vec<GameEvent>,
    /// Points earned from this action.
    pub points_earned: u32,
    /// Lines cleared (if any).
    pub lines_cleared: usize,
}

impl ActionResult {
    fn success(events: Vec<GameEvent>) -> Self {
        Self {
            success: true,
            events,
            points_earned: 0,
            lines_cleared: 0,
        }
    }

    fn failure() -> Self {
        Self {
            success: false,
            events: vec![],
            points_earned: 0,
            lines_cleared: 0,
        }
    }

    fn with_points(mut self, points: u32) -> Self {
        self.points_earned = points;
        self
    }

    fn with_lines(mut self, lines: usize) -> Self {
        self.lines_cleared = lines;
        self
    }
}

/// The game engine handles all game logic.
pub struct Engine;

impl Engine {
    /// Process a player action.
    pub fn process_action(game: &mut Game, action: Action) -> ActionResult {
        if game.state.status != GameStatus::Playing {
            return ActionResult::failure();
        }

        game.touch();

        match action {
            Action::Left | Action::Right | Action::Down => Self::handle_movement(game, action),
            Action::Rotate => Self::handle_rotation(game),
            Action::Drop => Self::handle_hard_drop(game),
        }
    }

    /// Process a gravity tick (piece falls down one cell).
    pub fn process_tick(game: &mut Game) -> ActionResult {
        if game.state.status != GameStatus::Playing {
            return ActionResult::failure();
        }

        game.touch();

        let moved_piece = game.state.current_piece.moved(0, 1);

        if game.state.board.can_place(&moved_piece) {
            game.state.current_piece = moved_piece;
            ActionResult::success(vec![GameEvent::PieceDropped])
        } else {
            // Piece can't move down, lock it
            Self::lock_and_spawn(game)
        }
    }

    /// Handle movement actions (left, right, down).
    fn handle_movement(game: &mut Game, action: Action) -> ActionResult {
        let (dx, dy) = action.movement_delta().unwrap();
        let moved_piece = game.state.current_piece.moved(dx, dy);

        if game.state.board.can_place(&moved_piece) {
            game.state.current_piece = moved_piece;

            let mut events = vec![GameEvent::PieceMoved];
            let mut points = 0;

            // Award soft drop point
            if action.is_soft_drop() {
                game.state.score += SOFT_DROP_POINTS;
                points = SOFT_DROP_POINTS;
                events.push(GameEvent::SoftDropPoint);
            }

            ActionResult::success(events).with_points(points)
        } else if action.is_soft_drop() {
            // Can't move down, lock the piece
            Self::lock_and_spawn(game)
        } else {
            ActionResult::failure()
        }
    }

    /// Handle rotation action.
    fn handle_rotation(game: &mut Game) -> ActionResult {
        let rotated_piece = game.state.current_piece.rotated();

        // Classic NES Tetris has no wall kicks
        if game.state.board.can_place(&rotated_piece) {
            game.state.current_piece = rotated_piece;
            ActionResult::success(vec![GameEvent::PieceRotated])
        } else {
            ActionResult::failure()
        }
    }

    /// Handle hard drop (instant drop and lock).
    fn handle_hard_drop(game: &mut Game) -> ActionResult {
        let mut drop_distance = 0;

        // Move piece down until it can't move anymore
        loop {
            let moved_piece = game.state.current_piece.moved(0, 1);
            if game.state.board.can_place(&moved_piece) {
                game.state.current_piece = moved_piece;
                drop_distance += 1;
            } else {
                break;
            }
        }

        // Award soft drop points for the distance traveled
        let drop_points = drop_distance * SOFT_DROP_POINTS;
        game.state.score += drop_points;

        // Lock the piece
        let mut result = Self::lock_and_spawn(game);
        result.points_earned += drop_points;
        result.events.insert(0, GameEvent::PieceDropped);
        result
    }

    /// Lock the current piece and spawn a new one.
    fn lock_and_spawn(game: &mut Game) -> ActionResult {
        let state = &mut game.state;
        let mut events = vec![GameEvent::PieceLocked];
        let mut total_points = 0;

        // Lock the piece onto the board
        state.board.lock_piece(&state.current_piece);

        // Clear completed lines
        let lines_cleared = state.board.clear_lines();

        if lines_cleared > 0 {
            events.push(GameEvent::LinesCleared);

            // Calculate points
            let line_points = calculate_line_clear_points(lines_cleared, state.level);
            state.score += line_points;
            total_points += line_points;

            // Update lines count
            state.lines += lines_cleared as u32;

            // Check for level up
            let new_level = calculate_level(game.start_level, state.lines);
            if new_level > state.level {
                state.level = new_level;
                events.push(GameEvent::LevelUp);
            }
        }

        // Spawn new piece
        let new_piece = Piece::spawn(state.next_piece);
        state.next_piece = PieceType::random();

        // Check if the new piece can be placed (game over check)
        if state.board.can_place(&new_piece) {
            state.current_piece = new_piece;
            events.push(GameEvent::NewPieceSpawned);
        } else {
            // Game over - can't spawn new piece
            state.current_piece = new_piece; // Still set it for display
            state.status = GameStatus::GameOver;
            events.push(GameEvent::GameOver);
        }

        ActionResult::success(events)
            .with_points(total_points)
            .with_lines(lines_cleared)
    }

    /// Get ghost piece position (where the piece would land).
    pub fn get_ghost_position(game: &Game) -> Piece {
        let mut ghost = game.state.current_piece.clone();

        loop {
            let moved = ghost.moved(0, 1);
            if game.state.board.can_place(&moved) {
                ghost = moved;
            } else {
                break;
            }
        }

        ghost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_game() -> Game {
        Game::new(0)
    }

    #[test]
    fn test_move_left() {
        let mut game = create_test_game();
        let initial_x = game.state.current_piece.x;
        
        let result = Engine::process_action(&mut game, Action::Left);
        
        assert!(result.success);
        assert_eq!(game.state.current_piece.x, initial_x - 1);
    }

    #[test]
    fn test_move_right() {
        let mut game = create_test_game();
        let initial_x = game.state.current_piece.x;
        
        let result = Engine::process_action(&mut game, Action::Right);
        
        assert!(result.success);
        assert_eq!(game.state.current_piece.x, initial_x + 1);
    }

    #[test]
    fn test_soft_drop_awards_point() {
        let mut game = create_test_game();
        let initial_score = game.state.score;
        
        let result = Engine::process_action(&mut game, Action::Down);
        
        assert!(result.success);
        assert_eq!(game.state.score, initial_score + 1);
        assert!(result.events.contains(&GameEvent::SoftDropPoint));
    }

    #[test]
    fn test_rotation() {
        let mut game = create_test_game();
        let initial_rotation = game.state.current_piece.rotation;
        
        let result = Engine::process_action(&mut game, Action::Rotate);
        
        // Success depends on piece type and position
        if result.success {
            assert_ne!(game.state.current_piece.rotation, initial_rotation);
        }
    }

    #[test]
    fn test_gravity_tick() {
        let mut game = create_test_game();
        let initial_y = game.state.current_piece.y;
        
        let result = Engine::process_tick(&mut game);
        
        assert!(result.success);
        assert_eq!(game.state.current_piece.y, initial_y + 1);
    }
}
