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
    extract::{State, Form},
    http::{StatusCode, header, HeaderMap},
    Json, response::{IntoResponse, Response},
};
use chrono::{Utc, Duration};
use sqlx::SqlitePool;
use uuid::Uuid;
use std::sync::Arc;

/// Login a user.
pub async fn login(
    State(app_state): State<AppState>,
    Form(login_request): Form<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("Login attempt for user: {}", login_request.username);
    
    // Get JWT config and database pool
    let jwt_config = &app_state.jwt_config;
    let db = &app_state.db;
    
    // Query the database for the user
    let user_result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(&login_request.username)
    .fetch_optional(db)
    .await;
    
    // Handle database errors
    let user = match user_result {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // Verify password
    tracing::info!("Password hash from DB: {}", user.password_hash);
    
    // For testing purposes, allow plaintext password comparison
    let password_verified = if user.password_hash == "admin" && login_request.password == "admin" {
        tracing::info!("Using plaintext password comparison for testing");
        true
    } else {
        // Try Argon2 verification
        match PasswordHash::new(&user.password_hash) {
            Ok(parsed_hash) => {
                match Argon2::default().verify_password(login_request.password.as_bytes(), &parsed_hash) {
                    Ok(_) => {
                        tracing::info!("Password verified successfully with Argon2");
                        true
                    },
                    Err(e) => {
                        tracing::error!("Password verification failed: {}", e);
                        false
                    }
                }
            },
            Err(e) => {
                tracing::error!("Failed to parse password hash: {}", e);
                false
            }
        }
    };
    
    if password_verified {
        // Update last login time
        let now = Utc::now();
        sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
            .bind(now)
            .bind(user.id.to_string())
            .execute(db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        // Generate JWT token
        let token = jwt_config.generate_token(&user)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Create auth response
        let auth_response = AuthResponse {
            token: token.clone(),
            user: UserResponse::from(user),
        };

        // Create response with cookie
        let json_response = axum::response::Json(auth_response);
        let mut response = json_response.into_response();
        
        // Add cookie header
        response.headers_mut().insert(
            header::SET_COOKIE,
            format!(
                "auth_token={}; Path=/; HttpOnly; Max-Age=86400; SameSite=Strict",
                token
            ).parse().unwrap(),
        );

        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
/// Login a user with JSON.
pub async fn login_json(
    State(app_state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("JSON login attempt for user: {}", login_request.username);
    
    // Get JWT config and database pool
    let jwt_config = &app_state.jwt_config;
    let db = &app_state.db;
    
    // Add detailed error handling and logging
    tracing::info!("Starting login process for user: {}", login_request.username);
    
    // Log request details
    tracing::info!("Login request: username={}, password=***", login_request.username);
    
    
    // Query the database for the user
    tracing::info!("Querying database for user: {}", login_request.username);
    let user_result = match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(&login_request.username)
    .fetch_optional(db)
    .await {
        Ok(result) => {
            tracing::info!("Database query successful");
            Ok(result)
        },
        Err(e) => {
            tracing::error!("Database query error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }?;
    // Handle database errors
    let user = match user_result {
        Some(user) => {
            tracing::info!("User found in database: {}", user.username);
            user
        },
        None => {
            tracing::error!("User not found in database: {}", login_request.username);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    
    
    // Verify password
    tracing::info!("Password hash from DB: {}", user.password_hash);
    
    // For testing purposes, allow plaintext password comparison
    let password_verified = if user.password_hash == "admin" && login_request.password == "admin" {
        tracing::info!("Using plaintext password comparison for testing - SUCCESS");
        true
    } else {
        tracing::info!("Attempting Argon2 password verification");
        // Try Argon2 verification
        match PasswordHash::new(&user.password_hash) {
            Ok(parsed_hash) => {
                tracing::info!("Successfully parsed password hash");
                match Argon2::default().verify_password(login_request.password.as_bytes(), &parsed_hash) {
                    Ok(_) => {
                        tracing::info!("Password verified successfully with Argon2");
                        true
                    },
                    Err(e) => {
                        tracing::error!("Password verification failed: {}", e);
                        false
                    }
                }
            },
            Err(e) => {
                tracing::error!("Failed to parse password hash: {}", e);
                false
            }
        }
    };
    
    if password_verified {
        tracing::info!("Password verification successful, generating token");
        // Update last login time
        let now = Utc::now();
        match sqlx::query("UPDATE users SET last_login = ? WHERE id = ?")
            .bind(now)
            .bind(user.id.to_string())
            .execute(db)
            .await {
                Ok(_) => tracing::info!("Updated last login time"),
                Err(e) => tracing::error!("Failed to update last login time: {}", e)
            }
        
        // Generate JWT token
        tracing::info!("Generating JWT token");
        let token = match jwt_config.generate_token(&user)
            .map_err(|e| {
                tracing::error!("Failed to generate token: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })? {
                token => {
                    tracing::info!("JWT token generated successfully");
                    token
                }
            };

        // Store username for logging
        let username = user.username.clone();
        
        // Create auth response
        tracing::info!("Creating auth response");
        let auth_response = AuthResponse {
            token: token.clone(),
            user: UserResponse::from(user),
        };

        // Create response with cookie
        let json_response = axum::response::Json(auth_response);
        let mut response = json_response.into_response();
        
        // Add cookie header
        tracing::info!("Adding cookie header");
        response.headers_mut().insert(
            header::SET_COOKIE,
            match format!(
                "auth_token={}; Path=/; HttpOnly; Max-Age=86400; SameSite=Strict",
                token
            ).parse() {
                Ok(cookie) => {
                    tracing::info!("Cookie header created successfully");
                    cookie
                },
                Err(e) => {
                    tracing::error!("Failed to create cookie header: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            },
        );

        tracing::info!("Login successful for user: {}", username);
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Get the current user from the token.
pub async fn get_current_user(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, StatusCode> {
    // Get JWT config
    let jwt_config = &app_state.jwt_config;
    
    // Extract the token from the Authorization header or cookie
    let token = extract_token_from_request(&headers)?;
    
    // Validate the token
    let claims = jwt_config.validate_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // In a real implementation, we would fetch the user from the database
    // For now, create a user from the claims
    let user = User {
        id: Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        username: claims.username,
        password_hash: "".to_string(),
        role: claims.role,
        email: Some("admin@example.com".to_string()),
        created_at: Utc::now() - Duration::hours(24), // Fake creation time
        updated_at: Utc::now(),
        last_login: Some(Utc::now()),
    };
    
    Ok(Json(UserResponse::from(user)))
}

/// Extract token from request (either Authorization header or cookie).
fn extract_token_from_request(headers: &HeaderMap) -> Result<String, StatusCode> {
    // Try Authorization header first
    if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
        if let Ok(auth_value) = auth_header.to_str() {
            if auth_value.starts_with("Bearer ") {
                return Ok(auth_value[7..].to_string());
            }
        }
    }
    
    // Try cookie next
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("auth_token=") {
                    return Ok(cookie[11..].to_string());
                }
            }
        }
    }
    
    Err(StatusCode::UNAUTHORIZED)
}

/// Get all users.
pub async fn get_users(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    // Get JWT config
    let jwt_config = &app_state.jwt_config;
    
    // Extract the token from the Authorization header or cookie
    let token = extract_token_from_request(&headers)?;
    
    // Validate the token
    let claims = jwt_config.validate_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Check if the user has admin role
    if claims.role != Role::Admin {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // In a real implementation, we would fetch users from the database
    // For now, return a mock list of users
    let users = vec![
        User {
            id: Uuid::new_v4(),
            username: "admin".to_string(),
            password_hash: "hashed_password".to_string(),
            role: Role::Admin,
            email: Some("admin@example.com".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Some(Utc::now()),
        },
        User {
            id: Uuid::new_v4(),
            username: "operator".to_string(),
            password_hash: "hashed_password".to_string(),
            role: Role::Operator,
            email: Some("operator@example.com".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
        },
        User {
            id: Uuid::new_v4(),
            username: "viewer".to_string(),
            password_hash: "hashed_password".to_string(),
            role: Role::Viewer,
            email: Some("viewer@example.com".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: None,
        },
    ];
    
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

/// Logout a user.
pub async fn logout() -> impl IntoResponse {
    // Create response with cookie that expires immediately
    let json_response = axum::response::Json(serde_json::json!({
        "message": "Logged out successfully"
    }));
    let mut response = json_response.into_response();
    
    // Add cookie header
    response.headers_mut().insert(
        header::SET_COOKIE,
        "auth_token=; Path=/; HttpOnly; Max-Age=0; SameSite=Strict".parse().unwrap(),
    );

    response
}

/// Debug endpoint to log request information.
pub async fn debug(
    method: axum::http::Method,
    uri: axum::http::Uri,
    headers: axum::http::HeaderMap,
) -> Result<String, StatusCode> {
    tracing::info!("Debug endpoint called with method: {}", method);
    tracing::info!("URI: {}", uri);
    tracing::info!("Headers: {:?}", headers);
    
    Ok(format!("Debug endpoint called with method: {}", method))
}