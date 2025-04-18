use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use std::sync::Arc;

use crate::proxy::AppState;

// Handler for toggling theme
pub async fn theme_toggle_handler(
    State(_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Set cookie for theme preference
    let cookie = "theme=dark; Path=/; Max-Age=31536000"; // 1 year

    Response::builder()
        .status(StatusCode::OK)
        .header(header::SET_COOKIE, cookie)
        .body(serde_json::to_string(&serde_json::json!({
            "message": "Theme preference updated"
        })).unwrap())
        .unwrap()
        .into_response()
}