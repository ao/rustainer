//! Application state for sharing between handlers.

use crate::auth::jwt::JwtConfig;
use crate::websocket::WebSocketManager;
use bollard::Docker;
use sqlx::SqlitePool;
use std::sync::{Arc, Mutex};

/// Combined application state.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool.
    pub db: SqlitePool,
    /// JWT configuration.
    pub jwt_config: Arc<JwtConfig>,
    /// WebSocket manager.
    pub ws_manager: WebSocketManager,
    /// User theme preference (dark or light)
    pub theme: Arc<Mutex<String>>,
    /// Docker client.
    pub docker: Arc<Docker>,
}

impl AppState {
    /// Create a new application state.
    pub fn new(db: SqlitePool, jwt_config: Arc<JwtConfig>) -> Self {
        // Connect to the Docker daemon
        let docker = match Docker::connect_with_local_defaults() {
            Ok(docker) => Arc::new(docker),
            Err(e) => {
                tracing::error!("Failed to connect to Docker daemon: {}", e);
                std::process::exit(1);
            }
        };

        Self {
            db,
            jwt_config,
            ws_manager: WebSocketManager::new(),
            theme: Arc::new(Mutex::new("dark".to_string())),
            docker,
        }
    }
}