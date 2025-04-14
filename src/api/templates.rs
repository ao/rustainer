use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use bollard::Docker;

use crate::docker::templates;
use crate::models::{
    ContainerTemplate, CreateTemplateRequest, UpdateTemplateRequest, DeployTemplateRequest,
};

/// List all container templates
pub async fn list_templates() -> Result<Json<Vec<ContainerTemplate>>, StatusCode> {
    match templates::list_templates().await {
        Ok(templates) => Ok(Json(templates)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get a specific template by ID
pub async fn get_template(
    Path(id): Path<Uuid>,
) -> Result<Json<ContainerTemplate>, StatusCode> {
    match templates::get_template(&id).await {
        Ok(Some(template)) => Ok(Json(template)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Create a new container template
pub async fn create_template(
    Json(request): Json<CreateTemplateRequest>,
) -> Result<Json<ContainerTemplate>, StatusCode> {
    match templates::create_template(request).await {
        Ok(template) => Ok(Json(template)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Update an existing template
pub async fn update_template(
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTemplateRequest>,
) -> Result<Json<ContainerTemplate>, StatusCode> {
    match templates::update_template(&id, request).await {
        Ok(Some(template)) => Ok(Json(template)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Delete a template
pub async fn delete_template(
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match templates::delete_template(&id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Deploy a container from a template
pub async fn deploy_template(
    State(docker): State<Arc<Docker>>,
    Json(request): Json<DeployTemplateRequest>,
) -> Result<Json<String>, StatusCode> {
    match templates::deploy_from_template(docker, request).await {
        Ok(container_id) => Ok(Json(container_id)),
        Err(e) => {
            tracing::error!("Failed to deploy template: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}