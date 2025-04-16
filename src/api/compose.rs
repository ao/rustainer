//! API endpoints for Docker Compose operations.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bollard::Docker;
use serde::{Deserialize, Serialize};

use crate::models::compose::{
    ComposeStack, CreateStackRequest, UpdateStackRequest, ScaleStackRequest, StackResponse
};
use crate::docker::{
    update_stack, create_stack, delete_stack, get_stack, get_stack_logs,
    list_stacks, restart_stack, start_stack, stop_stack, scale_stack
};

/// List all Docker Compose stacks.
pub async fn list_compose_stacks(
    State(app_state): State<crate::app_state::AppState>,
) -> Result<Json<Vec<ComposeStack>>, StatusCode> {
    match list_stacks(&app_state.docker).await {
        Ok(stacks) => Ok(Json(stacks)),
        Err(e) => {
            tracing::error!("Failed to list compose stacks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get a Docker Compose stack by ID.
pub async fn get_compose_stack(
    State(app_state): State<crate::app_state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match get_stack(&app_state.docker, &id).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to get compose stack {}: {}", id, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Create a new Docker Compose stack.
pub async fn create_compose_stack(
    State(app_state): State<crate::app_state::AppState>,
    Json(request): Json<CreateStackRequest>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match create_stack(&app_state.docker, request).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to create compose stack: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update an existing Docker Compose stack.
pub async fn update_compose_stack(
    State(app_state): State<crate::app_state::AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateStackRequest>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match update_stack(&app_state.docker, &id, request).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to update compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a Docker Compose stack.
pub async fn delete_compose_stack(
    State(app_state): State<crate::app_state::AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match delete_stack(&app_state.docker, &id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to delete compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Start a Docker Compose stack.
pub async fn start_compose_stack(
    State(app_state): State<crate::app_state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match start_stack(&app_state.docker, &id).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to start compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Stop a Docker Compose stack.
pub async fn stop_compose_stack(
    State(app_state): State<crate::app_state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match stop_stack(&app_state.docker, &id).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to stop compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Restart a Docker Compose stack.
pub async fn restart_compose_stack(
    State(app_state): State<crate::app_state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match restart_stack(&app_state.docker, &id).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to restart compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Request to validate a Docker Compose file.
#[derive(Debug, Deserialize)]
pub struct ValidateComposeRequest {
    /// Content of the compose file to validate.
    pub compose_content: String,
}

/// Response for validation.
#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    /// Whether the compose file is valid.
    pub valid: bool,
    /// Error message if validation failed.
    pub error: Option<String>,
}

/// Validate a Docker Compose file.
pub async fn validate_compose_file(
    State(app_state): State<crate::app_state::AppState>,
    Json(request): Json<ValidateComposeRequest>,
) -> Result<Json<ValidationResponse>, StatusCode> {
    match crate::docker::validate_compose_content(&request.compose_content) {
        Ok(_) => Ok(Json(ValidationResponse {
            valid: true,
            error: None,
        })),
        Err(e) => Ok(Json(ValidationResponse {
            valid: false,
            error: Some(e.to_string()),
        })),
    }
}

/// Get logs for a Docker Compose stack.
pub async fn get_compose_stack_logs(
    State(app_state): State<crate::app_state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<std::collections::HashMap<String, Vec<String>>>, StatusCode> {
    match get_stack_logs(&app_state.docker, &id).await {
        Ok(logs) => Ok(Json(logs)),
        Err(e) => {
            tracing::error!("Failed to get logs for compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Scale services in a Docker Compose stack.
pub async fn scale_compose_stack(
    State(app_state): State<crate::app_state::AppState>,
    Path(id): Path<String>,
    Json(request): Json<ScaleStackRequest>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match scale_stack(&app_state.docker, &id, request).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to scale compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}