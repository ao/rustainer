//! API endpoints for Docker Compose operations.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bollard::Docker;

use crate::docker::{
    ComposeStack, CreateStackRequest, UpdateStackRequest, update_stack,
    create_stack, delete_stack, get_stack, get_stack_logs, list_stacks, restart_stack, start_stack, stop_stack,
};

/// List all Docker Compose stacks.
pub async fn list_compose_stacks(
    State(docker): State<Arc<Docker>>,
) -> Result<Json<Vec<ComposeStack>>, StatusCode> {
    match list_stacks(&docker).await {
        Ok(stacks) => Ok(Json(stacks)),
        Err(e) => {
            tracing::error!("Failed to list compose stacks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get a Docker Compose stack by ID.
pub async fn get_compose_stack(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match get_stack(&docker, &id).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to get compose stack {}: {}", id, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Create a new Docker Compose stack.
pub async fn create_compose_stack(
    State(docker): State<Arc<Docker>>,
    Json(request): Json<CreateStackRequest>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match create_stack(&docker, request).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to create compose stack: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update an existing Docker Compose stack.
pub async fn update_compose_stack(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
    Json(request): Json<UpdateStackRequest>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match update_stack(&docker, &id, request).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to update compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a Docker Compose stack.
pub async fn delete_compose_stack(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match delete_stack(&docker, &id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to delete compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Start a Docker Compose stack.
pub async fn start_compose_stack(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match start_stack(&docker, &id).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to start compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Stop a Docker Compose stack.
pub async fn stop_compose_stack(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match stop_stack(&docker, &id).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to stop compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Restart a Docker Compose stack.
pub async fn restart_compose_stack(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<ComposeStack>, StatusCode> {
    match restart_stack(&docker, &id).await {
        Ok(stack) => Ok(Json(stack)),
        Err(e) => {
            tracing::error!("Failed to restart compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get logs for a Docker Compose stack.
pub async fn get_compose_stack_logs(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<std::collections::HashMap<String, Vec<String>>>, StatusCode> {
    match get_stack_logs(&docker, &id).await {
        Ok(logs) => Ok(Json(logs)),
        Err(e) => {
            tracing::error!("Failed to get logs for compose stack {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}