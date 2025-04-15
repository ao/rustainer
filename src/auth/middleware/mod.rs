//! Authentication and authorization middleware.

use crate::auth::jwt::JwtConfig;
use crate::auth::models::Role;
use axum::{
    extract::State,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    body::Body,
};
use axum::http::Request;
use std::sync::Arc;

/// Extract and validate JWT token from the Authorization header or cookie.
pub async fn require_auth(
    State(jwt_config): State<Arc<JwtConfig>>,
    request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode> {
    // Try to extract token from Authorization header first
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_string())
            } else {
                None
            }
        });
    
    // If no token in Authorization header, try cookie
    let token = if token.is_none() {
        request
            .headers()
            .get("Cookie")
            .and_then(|cookie_header| cookie_header.to_str().ok())
            .and_then(|cookie_str| {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if cookie.starts_with("auth_token=") {
                        return Some(cookie[11..].to_string());
                    }
                }
                None
            })
    } else {
        token
    };

    // If no token is provided, return 401 Unauthorized
    let token = token.ok_or(StatusCode::UNAUTHORIZED)?;

    // Validate the token
    let claims = jwt_config
        .validate_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Create a new request with the claims attached
    let mut request = request;
    request.extensions_mut().insert(claims);

    // Continue with the request
    Ok(next.run(request).await)
}

/// Middleware to check if the user has the required role.
pub async fn require_role(
    role: Role,
    request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode> {
    // Extract the claims from the request extensions
    let claims = request
        .extensions()
        .get::<crate::auth::models::Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if the user has the required role
    if claims.role as u8 >= role as u8 {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

/// Middleware to check if the user has permission to perform an action.
pub async fn require_permission(
    action: &'static str,
    request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode> {
    // Extract the claims from the request extensions
    let claims = request
        .extensions()
        .get::<crate::auth::models::Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if the user has permission to perform the action
    if claims.role.can(action) {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

// Remove the factory functions as they're not needed for now