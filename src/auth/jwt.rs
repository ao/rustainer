//! JWT token handling for authentication.

use crate::auth::models::{Claims, Role, User};
use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::sync::Arc;

/// JWT configuration.
#[derive(Clone)]
pub struct JwtConfig {
    /// Secret key for signing tokens.
    pub secret: Arc<String>,
    /// Token expiration time in minutes.
    pub expiration_minutes: i64,
}

impl JwtConfig {
    /// Create a new JWT configuration.
    pub fn new(secret: String, expiration_minutes: i64) -> Self {
        Self {
            secret: Arc::new(secret),
            expiration_minutes,
        }
    }

    /// Generate a JWT token for a user.
    pub fn generate_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let expires_at = now + Duration::minutes(self.expiration_minutes);

        let claims = Claims {
            sub: user.id.to_string(),
            username: user.username.clone(),
            role: user.role,
            iat: now.timestamp(),
            exp: expires_at.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Validate a JWT token and extract claims.
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| anyhow!("Invalid token: {}", e))?;

        Ok(token_data.claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_jwt_token_generation_and_validation() {
        // Create a test user
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            password_hash: "hash".to_string(),
            role: Role::Admin,
            email: Some("test@example.com".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
        };

        // Create JWT config
        let jwt_config = JwtConfig::new("test_secret".to_string(), 60);

        // Generate token
        let token = jwt_config.generate_token(&user).unwrap();
        assert!(!token.is_empty());

        // Validate token
        let claims = jwt_config.validate_token(&token).unwrap();
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.username, user.username);
        assert_eq!(claims.role, user.role);
    }
}