use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bollard::Docker;

use crate::docker;
use crate::models::{
    ConnectContainerRequest, CreateNetworkRequest, DisconnectContainerRequest, Network,
    NetworkDiagnostics,
};

/// List all networks
pub async fn list_networks(
    State(docker): State<Arc<Docker>>,
) -> Result<Json<Vec<docker::networks::Network>>, StatusCode> {
    match docker::networks::list_networks(&docker).await {
        Ok(networks) => Ok(Json(networks)),
        Err(e) => {
            tracing::error!("Failed to list networks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get network details
pub async fn get_network(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<Network>, StatusCode> {
    match docker::networks::get_network(&docker, &id).await {
        Ok(network) => Ok(Json(network)),
        Err(e) => {
            tracing::error!("Failed to get network {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new network
pub async fn create_network(
    State(docker): State<Arc<Docker>>,
    Json(request): Json<CreateNetworkRequest>,
) -> Result<Json<String>, StatusCode> {
    match docker::networks::create_network(&docker, request).await {
        Ok(id) => Ok(Json(id)),
        Err(e) => {
            tracing::error!("Failed to create network: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a network
pub async fn delete_network(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match docker::networks::delete_network(&docker, &id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to delete network {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Connect a container to a network
pub async fn connect_container(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
    Json(request): Json<ConnectContainerRequest>,
) -> Result<StatusCode, StatusCode> {
    match docker::networks::connect_container(&docker, &id, request).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to connect container to network {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Disconnect a container from a network
pub async fn disconnect_container(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
    Json(request): Json<DisconnectContainerRequest>,
) -> Result<StatusCode, StatusCode> {
    match docker::networks::disconnect_container(&docker, &id, request).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            tracing::error!("Failed to disconnect container from network {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Prune unused networks
pub async fn prune_networks(
    State(docker): State<Arc<Docker>>,
) -> Result<Json<Vec<String>>, StatusCode> {
    match docker::networks::prune_networks(&docker).await {
        Ok(networks) => Ok(Json(networks)),
        Err(e) => {
            tracing::error!("Failed to prune networks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get network diagnostics
pub async fn get_network_diagnostics(
    State(docker): State<Arc<Docker>>,
    Path(id): Path<String>,
) -> Result<Json<NetworkDiagnostics>, StatusCode> {
    match docker::networks::get_network_diagnostics(docker, &id).await {
        Ok(diagnostics) => Ok(Json(diagnostics)),
        Err(e) => {
            tracing::error!("Failed to get network diagnostics for {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}