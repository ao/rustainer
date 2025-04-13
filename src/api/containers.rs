use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bollard::Docker;

use crate::docker;
use crate::models::{Container, ContainerLogs, ContainerStats, CreateContainerRequest};

/// List all containers
pub async fn list_containers(
    State(docker): State<Arc<Docker>>,
) -> Result<Json<Vec<Container>>, StatusCode> {
    match docker::list_containers(&docker).await {
        Ok(containers) => Ok(Json(containers)),
        Err(e) => {
            tracing::error!("Failed to list containers: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Start a container
pub async fn start_container(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> StatusCode {
    match docker::start_container(&docker, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Failed to start container {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Stop a container
pub async fn stop_container(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> StatusCode {
    match docker::stop_container(&docker, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Failed to stop container {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Restart a container
pub async fn restart_container(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> StatusCode {
    match docker::restart_container(&docker, &id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) => {
            tracing::error!("Failed to restart container {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Get container logs
pub async fn container_logs(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<ContainerLogs>, StatusCode> {
    match docker::get_container_logs(&docker, &id).await {
        Ok(logs) => Ok(Json(logs)),
        Err(e) => {
            tracing::error!("Failed to get logs for container {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new container
pub async fn create_container(
    State(docker): State<Arc<Docker>>,
    Json(request): Json<CreateContainerRequest>,
) -> Result<Json<Container>, StatusCode> {
    match docker::create_container(&docker, request).await {
        Ok(container) => Ok(Json(container)),
        Err(e) => {
            tracing::error!("Failed to create container: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get container stats
pub async fn container_stats(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<ContainerStats>, StatusCode> {
    match docker::get_container_stats(&docker, &id).await {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            tracing::error!("Failed to get stats for container {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}