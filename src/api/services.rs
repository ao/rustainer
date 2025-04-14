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
    let services = app_state.get_services().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(services.into_iter().map(ServiceResponse::from).collect()))
}

/// Get a service by ID.
pub async fn get_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    let service = app_state.get_service(service_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ServiceResponse::from(service)))
}

/// Create a new service.
pub async fn create_service(
    State(app_state): State<AppState>,
    Json(request): Json<CreateServiceRequest>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    let service = app_state.create_service(request).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ServiceResponse::from(service)))
}

/// Update a service.
pub async fn update_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
    Json(request): Json<UpdateServiceRequest>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    let service = app_state.update_service(service_id, request).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ServiceResponse::from(service)))
}

/// Delete a service.
pub async fn delete_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    app_state.delete_service(service_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

/// Enable a service.
pub async fn enable_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    let service = app_state.enable_service(service_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ServiceResponse::from(service)))
}

/// Disable a service.
pub async fn disable_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    let service = app_state.disable_service(service_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ServiceResponse::from(service)))
}