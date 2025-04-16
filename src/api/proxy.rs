//! Service proxy handler for routing requests based on domain.

use crate::app_state::AppState;
use crate::models::service::ServiceType;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
};

/// Handle a request to the service proxy.
pub async fn handle_proxy_request<B>(
    State(app_state): State<AppState>,
    req: Request<B>,
) -> impl IntoResponse
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<axum::BoxError>,
{
    // Extract the host from the request
    let host = match req
        .headers()
        .get(axum::http::header::HOST)
        .and_then(|h| h.to_str().ok())
    {
        Some(host) => host,
        None => return (StatusCode::BAD_REQUEST, "Missing host header".to_string()),
    };

    // Remove port from host if present
    let domain = host.split(':').next().unwrap_or(host);

    // This is a placeholder since we don't have service functionality yet
    // In the future, this would call docker.get_services_by_domain(domain) or similar
    return (StatusCode::NOT_IMPLEMENTED, "Service routing not implemented yet".to_string());
}