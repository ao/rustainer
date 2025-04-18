pub mod handlers;
pub mod middleware;

use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::SharedJwtConfig;
use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // Subject (user ID)
    pub username: String,    // Username
    pub role: String,        // User role
    pub exp: i64,            // Expiration time (as UTC timestamp)
    pub iat: i64,            // Issued at (as UTC timestamp)
}

pub fn create_token(user: &User, jwt_config: &SharedJwtConfig) -> Result<String> {
    let now = Utc::now();
    let expiration = now + Duration::seconds(jwt_config.expiration as i64);
    
    let claims = Claims {
        sub: user.id.clone(),
        username: user.username.clone(),
        role: user.role.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_config.secret.as_bytes()),
    )
    .context("Failed to create JWT token")
}

pub fn validate_token(token: &str, jwt_config: &SharedJwtConfig) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_config.secret.as_bytes()),
        &Validation::default(),
    )
    .context("Failed to validate JWT token")?;
    
    Ok(token_data.claims)
}