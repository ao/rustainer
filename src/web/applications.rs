use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use std::sync::Arc;

use crate::proxy::AppState;

// Handler for the applications list page
pub async fn applications_list_handler(
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Html(include_str!("../static/applications.html"))
}

// Handler for the application detail page
pub async fn application_detail_handler(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
) -> impl IntoResponse {
    Html(include_str!("../static/application_detail.html"))
}

// Handler for the application create page
pub async fn application_create_handler(
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Html(include_str!("../static/application_create.html"))
}

// Handler for the application edit page
pub async fn application_edit_handler(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<String>,
) -> impl IntoResponse {
    Html(include_str!("../static/application_edit.html"))
}