//! API handlers for service management.

use crate::app_state::AppState;
use crate::models::service::{CreateServiceRequest, Service, ServiceResponse, UpdateServiceRequest};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

/// List all services.
pub async fn list_services(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ServiceResponse>>, StatusCode> {
    // This is a placeholder since we don't have service functionality yet
    // In the future, this would call docker.get_services() or similar
    Ok(Json(Vec::new()))
}

/// Get a service by ID.
pub async fn get_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    // This is a placeholder since we don't have service functionality yet
    // In the future, this would call docker.get_service(service_id) or similar
    return Err(StatusCode::NOT_IMPLEMENTED);
}

/// Create a new service.
pub async fn create_service(
    State(app_state): State<AppState>,
    Json(request): Json<CreateServiceRequest>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    // This is a placeholder since we don't have service functionality yet
    // In the future, this would call docker.create_service(request) or similar
    return Err(StatusCode::NOT_IMPLEMENTED);
}

/// Update a service.
pub async fn update_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
    Json(request): Json<UpdateServiceRequest>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    // This is a placeholder since we don't have service functionality yet
    // In the future, this would call docker.update_service(service_id, request) or similar
    return Err(StatusCode::NOT_IMPLEMENTED);
}

/// Delete a service.
pub async fn delete_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // This is a placeholder since we don't have service functionality yet
    // In the future, this would call docker.delete_service(service_id) or similar
    return Err(StatusCode::NOT_IMPLEMENTED);
}

/// Enable a service.
pub async fn enable_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    // This is a placeholder since we don't have service functionality yet
    // In the future, this would call docker.enable_service(service_id) or similar
    return Err(StatusCode::NOT_IMPLEMENTED);
}

/// Disable a service.
pub async fn disable_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    // This is a placeholder since we don't have service functionality yet
    // In the future, this would call docker.disable_service(service_id) or similar
    return Err(StatusCode::NOT_IMPLEMENTED);
}