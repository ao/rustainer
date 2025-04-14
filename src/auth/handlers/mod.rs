//! Authentication and user management handlers.

use crate::app_state::AppState;
use crate::auth::models::{
    AuthResponse, Claims, CreateUserRequest, LoginRequest, Role, User, UserResponse,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::{StatusCode, header},
    Json,
};
use chrono;
use uuid::Uuid;

/// Login a user.
pub async fn login(
    State(app_state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Check if the username and password match the admin credentials
    if request.username != "admin" || request.password != "admin" {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Create admin user
    let now = chrono::Utc::now();
    let user = User {
        id: Uuid::new_v4(),
        username: "admin".to_string(),
        password_hash: "hashed_password".to_string(),
        role: Role::Admin,
        email: Some("admin@example.com".to_string()),
        created_at: now,
        updated_at: now,
        last_login: Some(now),
    };

    // Generate JWT token
    let token = app_state.jwt_config
        .generate_token(&user)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return the token and user info
    Ok(Json(AuthResponse {
        token,
        user: UserResponse::from(user),
    }))
}

/// Get the current user from the token.
pub async fn get_current_user(
    State(_app_state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<UserResponse>, StatusCode> {
    // Extract the token from the Authorization header
    let _auth_header = headers.get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_string())
            } else {
                None
            }
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // For now, just return a mock user
    // In a real implementation, we would validate the token
    let user = User {
        id: Uuid::new_v4(),
        username: "admin".to_string(),
        password_hash: "".to_string(),
        role: Role::Admin,
        email: Some("admin@example.com".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_login: Some(chrono::Utc::now()),
    };
    
    Ok(Json(UserResponse::from(user)))
}

/// Get all users.
pub async fn get_users(
    State(_app_state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    // Extract the token from the Authorization header
    let _auth_header = headers.get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_string())
            } else {
                None
            }
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // For now, return a mock list of users
    let users = vec![
        User {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            password_hash: "hashed_password".to_string(),
            role: Role::Admin,
            email: Some("admin@example.com".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_login: Some(chrono::Utc::now()),
        },
        User {
            id: Uuid::new_v4(),
            username: "operator".to_string(),
            password_hash: "hashed_password".to_string(),
            role: Role::Operator,
            email: Some("operator@example.com".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_login: None,
        },
        User {
            id: Uuid::new_v4(),
            username: "viewer".to_string(),
            password_hash: "hashed_password".to_string(),
            role: Role::Viewer,
            email: Some("viewer@example.com".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            last_login: None,
        },
    ];
    
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}