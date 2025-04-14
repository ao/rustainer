//! Authentication and authorization module for Rustainer.
//!
//! This module provides functionality for user authentication, JWT token management,
//! and role-based access control (RBAC).

pub mod models;
pub mod middleware;
pub mod handlers;
pub mod jwt;

// Re-export commonly used items
pub use models::{User, Role, Claims};
pub use middleware::require_auth;
pub use jwt::JwtConfig;