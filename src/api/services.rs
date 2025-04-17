//! API handlers for service management.

use crate::app_state::AppState;
use crate::models::service::{CreateServiceRequest, Service, ServiceResponse, UpdateServiceRequest};
use crate::docker::services;
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
    match services::list_services(&app_state.db).await {
        Ok(services) => {
            let responses: Vec<ServiceResponse> = services.into_iter()
                .map(|service| service.into())
                .collect();
            Ok(Json(responses))
        },
        Err(e) => {
            tracing::error!("Failed to list services: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get a service by ID.
pub async fn get_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    match services::get_service(&app_state.db, &service_id).await {
        Ok(Some(service)) => Ok(Json(service.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get service {}: {}", service_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new service.
pub async fn create_service(
    State(app_state): State<AppState>,
    Json(request): Json<CreateServiceRequest>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    match services::create_service(&app_state.db, request).await {
        Ok(service) => Ok(Json(service.into())),
        Err(e) => {
            tracing::error!("Failed to create service: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update a service.
pub async fn update_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
    Json(request): Json<UpdateServiceRequest>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    match services::update_service(&app_state.db, &service_id, request).await {
        Ok(Some(service)) => Ok(Json(service.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update service {}: {}", service_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a service.
pub async fn delete_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match services::delete_service(&app_state.db, &service_id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to delete service {}: {}", service_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Enable a service.
pub async fn enable_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    match services::enable_service(&app_state.db, &service_id).await {
        Ok(Some(service)) => Ok(Json(service.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to enable service {}: {}", service_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Disable a service.
pub async fn disable_service(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<ServiceResponse>, StatusCode> {
    match services::disable_service(&app_state.db, &service_id).await {
        Ok(Some(service)) => Ok(Json(service.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to disable service {}: {}", service_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}