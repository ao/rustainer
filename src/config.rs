//! Application configuration.

use crate::auth::JwtConfig;
use anyhow::Result;
use serde::Deserialize;
use std::env;
use std::path::Path;
use std::sync::Arc;

/// Application configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Server configuration.
    pub server: ServerConfig,
    /// Database configuration.
    pub database: DatabaseConfig,
    /// JWT configuration.
    pub jwt: JwtConfigSettings,
}

/// Server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to.
    pub host: String,
    /// Port to listen on.
    pub port: u16,
}

/// Database configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// Database URL.
    pub url: String,
}

/// JWT configuration settings.
#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfigSettings {
    /// Secret key for signing tokens.
    pub secret: String,
    /// Token expiration time in minutes.
    pub expiration_minutes: i64,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        if Path::new(".env").exists() {
            dotenv::dotenv().ok();
        }

        // Default values
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;
        
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./data/rustainer.db".to_string());
        
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            // Generate a random secret if not provided
            let random_bytes: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
            // Use a simple hex encoding instead of base64
            random_bytes.iter().map(|b| format!("{:02x}", b)).collect()
        });
        
        let jwt_expiration = env::var("JWT_EXPIRATION_MINUTES")
            .unwrap_or_else(|_| "60".to_string())
            .parse::<i64>()?;

        Ok(Config {
            server: ServerConfig { host, port },
            database: DatabaseConfig { url: db_url },
            jwt: JwtConfigSettings {
                secret: jwt_secret,
                expiration_minutes: jwt_expiration,
            },
        })
    }

    /// Create JWT configuration.
    pub fn create_jwt_config(&self) -> JwtConfig {
        JwtConfig::new(self.jwt.secret.clone(), self.jwt.expiration_minutes)
    }
}