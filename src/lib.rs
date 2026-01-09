//! Tetris API - Classic NES Tetris as a REST API.
//!
//! This library provides the core game logic and API handlers
//! for a Tetris game server.

pub mod api;
pub mod auth;
pub mod config;
pub mod error;
pub mod game;
pub mod models;
pub mod state;

pub use api::build_router;
pub use config::Config;
pub use state::AppState;
