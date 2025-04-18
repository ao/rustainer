pub mod handlers;
pub mod applications;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

use crate::proxy::AppState;

// Handler for the index page
pub async fn index_handler(
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}

// Handler for the login page
pub async fn login_handler() -> impl IntoResponse {
    Html(include_str!("../static/login.html"))
}

// Handler for the dashboard page
pub async fn dashboard_handler(
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    Html(include_str!("../static/dashboard.html"))
}