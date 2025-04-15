use std::sync::{Arc, Mutex};
use crate::auth::jwt::JwtConfig;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    /// User theme preference (dark or light)
    pub theme: Arc<Mutex<String>>,
    /// JWT configuration
    pub jwt_config: Option<Arc<JwtConfig>>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            // Default to dark theme
            theme: Arc::new(Mutex::new("dark".to_string())),
            jwt_config: None,
        }
    }

    /// Set JWT configuration
    pub fn with_jwt_config(mut self, jwt_config: Arc<JwtConfig>) -> Self {
        self.jwt_config = Some(jwt_config);
        self
    }
}