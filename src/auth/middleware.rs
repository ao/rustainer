use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::config::SharedJwtConfig;

pub async fn require_auth<B>(
    State(jwt_config): State<SharedJwtConfig>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header or cookie
    let token = extract_token_from_request(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Validate token
    let claims = crate::auth::validate_token(&token, &jwt_config)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Add claims to request extensions
    request.extensions_mut().insert(claims);
    
    // Continue with the request
    Ok(next.run(request).await)
}

fn extract_token_from_request<B>(request: &Request<B>) -> Option<String> {
    // Try to extract from Authorization header
    let auth_header = request.headers().get("Authorization")?;
    let auth_header = auth_header.to_str().ok()?;
    
    if auth_header.starts_with("Bearer ") {
        return Some(auth_header[7..].to_string());
    }
    
    // Try to extract from cookie
    let cookie_header = request.headers().get("Cookie")?;
    let cookie_header = cookie_header.to_str().ok()?;
    
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if cookie.starts_with("auth_token=") {
            return Some(cookie[11..].to_string());
        }
    }
    
    None
}