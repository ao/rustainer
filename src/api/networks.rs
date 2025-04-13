use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use bollard::Docker;

use crate::docker;
use crate::docker::Network;

/// List all networks
pub async fn list_networks(
    State(docker): State<Arc<Docker>>,
) -> Result<Json<Vec<Network>>, StatusCode> {
    match docker::list_networks(&docker).await {
        Ok(networks) => Ok(Json(networks)),
        Err(e) => {
            tracing::error!("Failed to list networks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}