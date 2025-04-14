//! Application state for sharing between handlers.

use crate::auth::JwtConfig;
use sqlx::SqlitePool;
use std::sync::Arc;

/// Combined application state.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool.
    pub db: SqlitePool,
    /// JWT configuration.
    pub jwt_config: Arc<JwtConfig>,
}

impl AppState {
    /// Create a new application state.
    pub fn new(db: SqlitePool, jwt_config: Arc<JwtConfig>) -> Self {
        Self { db, jwt_config }
    }
}