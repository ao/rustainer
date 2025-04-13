use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use bollard::Docker;

use crate::docker;
use crate::docker::Volume;

/// List all volumes
pub async fn list_volumes(
    State(docker): State<Arc<Docker>>,
) -> Result<Json<Vec<Volume>>, StatusCode> {
    match docker::list_volumes(&docker).await {
        Ok(volumes) => Ok(Json(volumes)),
        Err(e) => {
            tracing::error!("Failed to list volumes: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}