//! API route definitions.

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::{auth::auth_middleware, state::AppState};

use super::handlers;

/// Build the API router.
pub fn build_router(state: AppState) -> Router {
    // Routes that don't require authentication
    let public_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/games", post(handlers::create_game));

    // Routes that require authentication
    let protected_routes = Router::new()
        .route("/games/{session_id}", get(handlers::get_game))
        .route("/games/{session_id}", delete(handlers::delete_game))
        .route("/games/{session_id}/action", post(handlers::perform_action))
        .route("/games/{session_id}/tick", post(handlers::tick))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/api/v1", public_routes.merge(protected_routes))
        .with_state(state)
}
