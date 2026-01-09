//! API route handlers.

use axum::{
    extract::{Extension, Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    auth::AuthSession,
    error::ApiError,
    game::Engine,
    models::{
        CreateGameRequest, CreateGameResponse, Game, GameOverResponse, GameStateResponse,
        GameStatus,
    },
    state::AppState,
};

use super::responses::{ActionRequest, ActionResponse, HealthResponse};

/// Health check endpoint.
pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        active_games: state.active_games_count(),
    })
}

/// Create a new game.
pub async fn create_game(
    State(state): State<AppState>,
    Json(req): Json<CreateGameRequest>,
) -> Result<Json<CreateGameResponse>, ApiError> {
    let start_level = req.start_level.min(19); // Cap at level 19
    let game = Game::new(start_level);
    let session_id = game.session_id;

    // Create JWT token
    let token = state.jwt.create_token(session_id)?;

    // Store the game
    let game_state = game.state.clone();
    state.games.insert(session_id, game);

    Ok(Json(CreateGameResponse {
        session_id,
        token,
        state: game_state,
    }))
}

/// Get game state.
pub async fn get_game(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthSession>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<GameStateResponse>, ApiError> {
    // Verify session ID matches token
    if auth.session_id != session_id {
        return Err(ApiError::Unauthorized(
            "Token does not match session".to_string(),
        ));
    }

    let game = state
        .games
        .get(&session_id)
        .ok_or_else(|| ApiError::NotFound("Game not found".to_string()))?;

    Ok(Json(GameStateResponse {
        session_id,
        state: game.state.clone(),
    }))
}

/// Perform an action.
pub async fn perform_action(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthSession>,
    Path(session_id): Path<Uuid>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<ActionResponse>, ApiError> {
    // Verify session ID matches token
    if auth.session_id != session_id {
        return Err(ApiError::Unauthorized(
            "Token does not match session".to_string(),
        ));
    }

    let mut game = state
        .games
        .get_mut(&session_id)
        .ok_or_else(|| ApiError::NotFound("Game not found".to_string()))?;

    // Check if game is already over
    if game.state.status == GameStatus::GameOver {
        return Err(ApiError::GameOver);
    }

    let result = Engine::process_action(&mut game, req.action);

    Ok(Json(ActionResponse {
        success: result.success,
        state: game.state.clone(),
        events: result.events,
        lines_cleared_count: if result.lines_cleared > 0 {
            Some(result.lines_cleared)
        } else {
            None
        },
        points_earned: if result.points_earned > 0 {
            Some(result.points_earned)
        } else {
            None
        },
    }))
}

/// Gravity tick.
pub async fn tick(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthSession>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ActionResponse>, ApiError> {
    // Verify session ID matches token
    if auth.session_id != session_id {
        return Err(ApiError::Unauthorized(
            "Token does not match session".to_string(),
        ));
    }

    let mut game = state
        .games
        .get_mut(&session_id)
        .ok_or_else(|| ApiError::NotFound("Game not found".to_string()))?;

    // Check if game is already over
    if game.state.status == GameStatus::GameOver {
        return Err(ApiError::GameOver);
    }

    let result = Engine::process_tick(&mut game);

    Ok(Json(ActionResponse {
        success: result.success,
        state: game.state.clone(),
        events: result.events,
        lines_cleared_count: if result.lines_cleared > 0 {
            Some(result.lines_cleared)
        } else {
            None
        },
        points_earned: if result.points_earned > 0 {
            Some(result.points_earned)
        } else {
            None
        },
    }))
}

/// End/delete a game.
pub async fn delete_game(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthSession>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<GameOverResponse>, ApiError> {
    // Verify session ID matches token
    if auth.session_id != session_id {
        return Err(ApiError::Unauthorized(
            "Token does not match session".to_string(),
        ));
    }

    let (_, game) = state
        .games
        .remove(&session_id)
        .ok_or_else(|| ApiError::NotFound("Game not found".to_string()))?;

    Ok(Json(GameOverResponse {
        final_score: game.state.score,
        final_level: game.state.level,
        lines_cleared: game.state.lines,
        duration_seconds: game.duration_seconds(),
    }))
}
