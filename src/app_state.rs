//! Application state for sharing between handlers.

use crate::auth::jwt::JwtConfig;
use crate::websocket::WebSocketManager;
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
}

impl AppState {
    /// Create a new application state.
    pub fn new(db: SqlitePool, jwt_config: Arc<JwtConfig>) -> Self {
        Self {
            db,
            jwt_config,
            ws_manager: WebSocketManager::new(),
            theme: Arc::new(Mutex::new("dark".to_string())),
        }
    }
}