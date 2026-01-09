//! Configuration management.

use std::env;

/// Application configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Server host.
    pub host: String,
    /// Server port.
    pub port: u16,
    /// JWT secret key.
    pub jwt_secret: String,
    /// JWT token duration in hours.
    pub token_duration_hours: i64,
    /// Log level.
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "tetris-api-super-secret-key-change-in-production".to_string()),
            token_duration_hours: env::var("TOKEN_DURATION_HOURS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(24),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        }
    }

    /// Get the server address.
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}
