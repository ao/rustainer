use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::app_state::AppState;
use crate::docker;
use crate::models::{Container, ContainerLogs, ContainerStats, CreateContainerRequest};

/// List all containers
pub async fn list_containers(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Container>>, StatusCode> {
    match docker::list_containers(&app_state.docker).await {
        Ok(containers) => Ok(Json(containers)),
        Err(e) => {
            tracing::error!("Failed to list containers: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Start a container
pub async fn start_container(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> StatusCode {
    match docker::start_container(&app_state.docker, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Failed to start container {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Stop a container
pub async fn stop_container(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> StatusCode {
    match docker::stop_container(&app_state.docker, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Failed to stop container {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Restart a container
pub async fn restart_container(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> StatusCode {
    match docker::restart_container(&app_state.docker, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Failed to restart container {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Get container logs
pub async fn container_logs(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ContainerLogs>, StatusCode> {
    match docker::get_container_logs(&app_state.docker, &id).await {
        Ok(logs) => Ok(Json(logs)),
        Err(e) => {
            tracing::error!("Failed to get logs for container {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new container
pub async fn create_container(
    State(app_state): State<AppState>,
    Json(request): Json<CreateContainerRequest>,
) -> Result<Json<Container>, StatusCode> {
    match docker::create_container(&app_state.docker, request).await {
        Ok(container) => Ok(Json(container)),
        Err(e) => {
            tracing::error!("Failed to create container: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get container stats
pub async fn container_stats(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ContainerStats>, StatusCode> {
    match docker::get_container_stats(&app_state.docker, &id).await {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            tracing::error!("Failed to get stats for container {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}