//! JWT authentication for game sessions.

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;

/// JWT claims for a game session.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (session_id).
    pub sub: Uuid,
    /// Expiration time (Unix timestamp).
    pub exp: i64,
    /// Issued at (Unix timestamp).
    pub iat: i64,
}

impl Claims {
    /// Create new claims for a session.
    pub fn new(session_id: Uuid, duration_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: session_id,
            exp: (now + Duration::hours(duration_hours)).timestamp(),
            iat: now.timestamp(),
        }
    }

    /// Get the session ID from claims.
    pub fn session_id(&self) -> Uuid {
        self.sub
    }
}

/// JWT service for creating and validating tokens.
#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    token_duration_hours: i64,
}

impl JwtService {
    /// Create a new JWT service with the given secret.
    pub fn new(secret: &str, token_duration_hours: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            token_duration_hours,
        }
    }

    /// Create a token for a session.
    pub fn create_token(&self, session_id: Uuid) -> Result<String, ApiError> {
        let claims = Claims::new(session_id, self.token_duration_hours);
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| ApiError::Internal(format!("Failed to create token: {}", e)))
    }

    /// Validate a token and extract claims.
    pub fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, ApiError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| ApiError::Unauthorized(format!("Invalid token: {}", e)))
    }

    /// Extract session ID from a token.
    pub fn get_session_id(&self, token: &str) -> Result<Uuid, ApiError> {
        let token_data = self.validate_token(token)?;
        Ok(token_data.claims.session_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_validate_token() {
        let service = JwtService::new("test-secret-key-12345", 24);
        let session_id = Uuid::new_v4();

        let token = service.create_token(session_id).unwrap();
        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.claims.session_id(), session_id);
    }

    #[test]
    fn test_invalid_token() {
        let service = JwtService::new("test-secret-key-12345", 24);
        let result = service.validate_token("invalid-token");
        assert!(result.is_err());
    }
}
