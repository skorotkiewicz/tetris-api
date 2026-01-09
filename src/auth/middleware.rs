//! Authentication middleware for Axum.

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::state::AppState;

/// Extension to store authenticated session ID.
#[derive(Clone)]
pub struct AuthSession {
    pub session_id: Uuid,
}

/// Extract Bearer token from Authorization header.
fn extract_bearer_token(req: &Request) -> Option<&str> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}

/// Authentication middleware.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_bearer_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;

    let session_id = state
        .jwt
        .get_session_id(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Verify the game exists
    if !state.games.contains_key(&session_id) {
        return Err(StatusCode::NOT_FOUND);
    }

    // Add session to request extensions
    req.extensions_mut().insert(AuthSession { session_id });

    Ok(next.run(req).await)
}
