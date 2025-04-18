use axum::{
    extract::{Json, State},
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    auth,
    config::SharedJwtConfig,
    models::{User, user::UserRole},
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    id: String,
    username: String,
    role: String,
}

pub async fn login(
    State(pool): State<SqlitePool>,
    State(jwt_config): State<SharedJwtConfig>,
    Json(login_req): Json<LoginRequest>,
) -> impl IntoResponse {
    // Find user by username
    let user = match sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, 
            username, 
            password_hash, 
            role as "role: String", 
            created_at as "created_at: chrono::DateTime<chrono::Utc>", 
            updated_at as "updated_at: chrono::DateTime<chrono::Utc>"
        FROM users 
        WHERE username = ?
        "#,
        login_req.username
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(user)) => User {
            id: user.id,
            username: user.username,
            password_hash: user.password_hash,
            role: UserRole::from(user.role),
            created_at: user.created_at,
            updated_at: user.updated_at,
        },
        _ => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid username or password"
            }))).into_response();
        }
    };

    // Verify password
    let is_valid = match argon2::verify_encoded(&user.password_hash, login_req.password.as_bytes()) {
        Ok(valid) => valid,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
                "error": "Invalid username or password"
            }))).into_response();
        }
    };

    if !is_valid {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "error": "Invalid username or password"
        }))).into_response();
    }

    // Generate JWT token
    let token = match auth::create_token(&user, &jwt_config) {
        Ok(token) => token,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to generate token"
            }))).into_response();
        }
    };

    // Create response with cookie
    let user_info = UserInfo {
        id: user.id,
        username: user.username,
        role: user.role.to_string(),
    };

    let response = LoginResponse {
        token: token.clone(),
        user: user_info,
    };

    // Build response with cookie
    let cookie = format!(
        "auth_token={}; HttpOnly; Path=/; Max-Age={}",
        token, jwt_config.expiration
    );

    Response::builder()
        .status(StatusCode::OK)
        .header(header::SET_COOKIE, cookie)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&response).unwrap())
        .unwrap()
        .into_response()
}

pub async fn logout() -> impl IntoResponse {
    // Clear the auth cookie
    let cookie = "auth_token=; HttpOnly; Path=/; Max-Age=0";

    Response::builder()
        .status(StatusCode::OK)
        .header(header::SET_COOKIE, cookie)
        .body(serde_json::to_string(&serde_json::json!({
            "message": "Logged out successfully"
        })).unwrap())
        .unwrap()
        .into_response()
}

pub async fn get_current_user(
    claims: axum::extract::Extension<auth::Claims>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "id": claims.sub,
        "username": claims.username,
        "role": claims.role,
    }))
}

pub async fn get_users(
    State(pool): State<SqlitePool>,
    claims: axum::extract::Extension<auth::Claims>,
) -> impl IntoResponse {
    // Only admin can list users
    if claims.role != "admin" {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({
            "error": "Insufficient permissions"
        }))).into_response();
    }

    // Get all users
    let users = match sqlx::query!(
        r#"
        SELECT id, username, role, created_at, updated_at
        FROM users
        ORDER BY username
        "#
    )
    .fetch_all(&pool)
    .await
    {
        Ok(users) => users,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": "Failed to fetch users"
            }))).into_response();
        }
    };

    // Convert to JSON
    let users = users
        .into_iter()
        .map(|user| {
            serde_json::json!({
                "id": user.id,
                "username": user.username,
                "role": user.role,
                "created_at": user.created_at,
                "updated_at": user.updated_at,
            })
        })
        .collect::<Vec<_>>();

    Json(users).into_response()
}