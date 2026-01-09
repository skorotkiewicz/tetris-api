//! Tetris API Server
//!
//! Classic NES Tetris as a REST API.

use std::net::SocketAddr;

use tetris_api::{build_router, AppState, Config};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Config::from_env();

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_level.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create application state
    let state = AppState::new(&config.jwt_secret, config.token_duration_hours);

    // Build router with middleware
    let app = build_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Parse address
    let addr: SocketAddr = config.addr().parse().expect("Invalid address");

    tracing::info!("🎮 Tetris API starting on http://{}", addr);
    tracing::info!("📖 Endpoints:");
    tracing::info!("   POST   /api/v1/games          - Create new game");
    tracing::info!("   GET    /api/v1/games/{{id}}     - Get game state");
    tracing::info!("   POST   /api/v1/games/{{id}}/action - Perform action");
    tracing::info!("   POST   /api/v1/games/{{id}}/tick   - Gravity tick");
    tracing::info!("   DELETE /api/v1/games/{{id}}     - End game");
    tracing::info!("   GET    /api/v1/health         - Health check");

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
