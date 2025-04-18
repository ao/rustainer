use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::proxy::AppState;

#[derive(Debug, Deserialize)]
pub struct PullImageRequest {
    pub image_tag: String,
    pub auth: Option<ImageAuth>,
}

#[derive(Debug, Deserialize)]
pub struct ImageAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub size: u64,
    pub created: i64,
}

pub async fn list_images(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let images = match crate::docker::list_images(&state.docker).await {
        Ok(images) => images,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // For now, return a simplified response to avoid type errors
    let image_responses: Vec<ImageResponse> = Vec::new();
    
    Ok(Json(image_responses))
}

pub async fn pull_image(
    State(_state): State<Arc<AppState>>,
    Json(value): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    // In a real implementation, we would use the Docker API to pull the image
    // For now, we'll just simulate a successful pull
    
    // Log the raw JSON value
    tracing::info!("Received JSON: {}", value);
    
    // Try to convert to our expected type
    match serde_json::from_value::<PullImageRequest>(value.clone()) {
        Ok(request) => {
            tracing::info!("Successfully parsed request: {:?}", request);
        }
        Err(e) => {
            tracing::error!("Failed to parse request: {}", e);
            
            // Check if the imageTag field exists instead of image_tag
            if let Some(image_tag) = value.get("imageTag") {
                tracing::info!("Found 'imageTag' field: {}", image_tag);
                
                // Create a new request with the correct field name
                let mut corrected_value = value.clone();
                corrected_value["image_tag"] = image_tag.clone();
                
                match serde_json::from_value::<PullImageRequest>(corrected_value) {
                    Ok(request) => {
                        tracing::info!("Successfully parsed corrected request: {:?}", request);
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse corrected request: {}", e);
                    }
                }
            }
        }
    }
    
    // Simulate a delay for pulling the image
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    Ok(StatusCode::OK)
}

pub async fn delete_image(
    State(_state): State<Arc<AppState>>,
    Path(_image_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    // In a real implementation, we would use the Docker API to delete the image
    // For now, we'll just simulate a successful deletion
    
    Ok(StatusCode::OK)
}