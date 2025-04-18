use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::Application;
use crate::proxy::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateApplicationRequest {
    pub name: String,
    pub domain: String,
    pub container_id: Option<String>,
    pub container_port: u16,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApplicationRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub container_id: Option<String>,
    pub container_port: Option<u16>,
    pub enabled: Option<bool>,
}

pub async fn list_applications(
    State(_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    // For now, return a simplified response
    let applications = vec![
        Application {
            id: "1".to_string(),
            name: "Example App".to_string(),
            domain: "example.com".to_string(),
            container_id: Some("abc123".to_string()),
            container_port: 8080,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    ];
    
    Ok(Json(applications))
}

pub async fn get_application(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // For now, return a simplified response
    let application = Application {
        id: "1".to_string(),
        name: "Example App".to_string(),
        domain: "example.com".to_string(),
        container_id: Some("abc123".to_string()),
        container_port: 8080,
        enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    Ok(Json(application))
}

pub async fn create_application(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<CreateApplicationRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // For now, return a simplified response
    let application = Application {
        id: uuid::Uuid::new_v4().to_string(),
        name: req.name,
        domain: req.domain,
        container_id: req.container_id,
        container_port: req.container_port as i64, // Convert u16 to i64
        enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    Ok((StatusCode::CREATED, Json(application)))
}

pub async fn update_application(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
    Json(_req): Json<UpdateApplicationRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // For now, return a simplified response
    let application = Application {
        id: "1".to_string(),
        name: "Updated App".to_string(),
        domain: "example.com".to_string(),
        container_id: Some("abc123".to_string()),
        container_port: 8080,
        enabled: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    Ok(Json(application))
}

pub async fn delete_application(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // For now, just return a success status
    Ok(StatusCode::NO_CONTENT)
}

pub async fn enable_application(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // For now, just return a success status
    Ok(StatusCode::OK)
}

pub async fn disable_application(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // For now, just return a success status
    Ok(StatusCode::OK)
}