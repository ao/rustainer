use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub proxy: ProxyConfig,
    pub auth: AuthConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration: u64, // in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        // Default configuration
        let config = Config {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .unwrap_or(3000),
            },
            proxy: ProxyConfig {
                host: std::env::var("PROXY_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("PROXY_PORT")
                    .unwrap_or_else(|_| "80".to_string())
                    .parse()
                    .unwrap_or(80),
            },
            auth: AuthConfig {
                jwt_secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "rustainer_default_secret_please_change_in_production".to_string()),
                jwt_expiration: std::env::var("JWT_EXPIRATION")
                    .unwrap_or_else(|_| "86400".to_string()) // 24 hours
                    .parse()
                    .unwrap_or(86400),
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "sqlite:data/rustainer.db".to_string()),
            },
        };

        Ok(config)
    }

    pub fn create_jwt_config(&self) -> JwtConfig {
        JwtConfig {
            secret: self.auth.jwt_secret.clone(),
            expiration: self.auth.jwt_expiration,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: u64,
}

pub type SharedJwtConfig = Arc<JwtConfig>;