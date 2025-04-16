//! Web handlers for Docker Compose UI pages.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use askama::Template;
use std::fs;

use crate::app_state::AppState;
use crate::models::compose::StackStatus;

/// Template for the Docker Compose stacks list page.
#[derive(Template)]
#[template(path = "pages/compose_list.html")]
struct ComposeListTemplate {
    stacks: Vec<crate::models::compose::ComposeStack>,
    theme: String,
}

/// Template for the Docker Compose stack detail page.
#[derive(Template)]
#[template(path = "pages/compose_detail.html")]
struct ComposeDetailTemplate {
    stack: crate::models::compose::ComposeStack,
    compose_content: String,
    env_vars: Vec<(String, String)>,
    has_env: bool,
}

/// Template for the Docker Compose stack editor page.
#[derive(Template)]
#[template(path = "pages/basic.html")]
struct ComposeEditorTemplate {
    name: String,
    status: String,
    has_env: bool,
    env_vars: Vec<(String, String)>,
}

impl std::fmt::Display for StackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackStatus::Up => write!(f, "Up"),
            StackStatus::Down => write!(f, "Down"),
            StackStatus::Partial => write!(f, "Partial"),
            StackStatus::Error => write!(f, "Error"),
        }
    }
}

/// Handler for the Docker Compose stacks list page.
pub async fn compose_list_handler(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match crate::docker::compose::list_stacks(&app_state.docker).await {
        Ok(stacks) => {
            // Get theme from app state
            let theme = {
                let theme_lock = app_state.theme.lock().unwrap();
                theme_lock.clone()
            };
            
            let template = ComposeListTemplate { stacks, theme };
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(err) => {
                    tracing::error!("Failed to render compose list template: {}", err);
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to render template",
                    )
                        .into_response()
                }
            }
        }
        Err(err) => {
            tracing::error!("Failed to list compose stacks: {}", err);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list compose stacks",
            )
                .into_response()
        }
    }
}

/// Handler for the Docker Compose stack detail page.
pub async fn compose_detail_handler(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match crate::docker::compose::get_stack(&app_state.docker, &id).await {
        Ok(stack) => {
            // Read the compose file content
            let compose_content = match fs::read_to_string(&stack.file_path) {
                Ok(content) => content,
                Err(err) => {
                    tracing::error!("Failed to read compose file: {}", err);
                    "# Failed to read compose file".to_string()
                }
            };
            
            // Extract environment variables
            let (env_vars, has_env) = if let Some(env) = &stack.environment {
                let vars: Vec<(String, String)> = env.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                (vars, !vars.is_empty())
            } else {
                (Vec::new(), false)
            };
            
            let template = ComposeDetailTemplate {
                stack,
                compose_content,
                env_vars,
                has_env,
            };
            
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(err) => {
                    tracing::error!("Failed to render compose detail template: {}", err);
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to render template",
                    )
                        .into_response()
                }
            }
        }
        Err(err) => {
            tracing::error!("Failed to get compose stack {}: {}", id, err);
            (
                axum::http::StatusCode::NOT_FOUND,
                "Compose stack not found",
            )
                .into_response()
        }
    }
}

/// Handler for the Docker Compose stack creation page.
pub async fn compose_create_handler() -> impl IntoResponse {
    let template = ComposeEditorTemplate {
        name: "New Stack".to_string(),
        status: "New".to_string(),
        has_env: false,
        env_vars: Vec::new(),
    };
    
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            tracing::error!("Failed to render compose editor template: {}", err);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response()
        }
    }
}

/// Handler for the Docker Compose stack edit page.
pub async fn compose_edit_handler(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match crate::docker::compose::get_stack(&app_state.docker, &id).await {
        Ok(stack) => {
            // Extract environment variables
            let env_vars = match &stack.environment {
                Some(env) => env.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<Vec<(String, String)>>(),
                None => Vec::new(),
            };
            
            let template = ComposeEditorTemplate {
                name: stack.name.clone(),
                status: stack.status.to_string(),
                has_env: !env_vars.is_empty(),
                env_vars,
            };
            
            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(err) => {
                    tracing::error!("Failed to render compose editor template: {}", err);
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to render template",
                    )
                        .into_response()
                }
            }
        }
        Err(err) => {
            tracing::error!("Failed to get compose stack {}: {}", id, err);
            (
                axum::http::StatusCode::NOT_FOUND,
                "Compose stack not found",
            )
                .into_response()
        }
    }
}