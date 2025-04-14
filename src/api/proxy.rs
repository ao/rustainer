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

    // Get services from the database
    let services = match app_state.get_services_by_domain(domain).await {
        Ok(services) => services,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
    };

    // Find a service that matches the domain
    let service = match services
        .into_iter()
        .find(|s| s.matches_domain(domain))
    {
        Some(service) => service,
        None => return (StatusCode::NOT_FOUND, "Service not found".to_string()),
    };

    // Check if the service is enabled
    if !service.enabled {
        return (StatusCode::SERVICE_UNAVAILABLE, "Service is disabled".to_string());
    }

    // Route the request based on the service type
    match service.service_type {
        ServiceType::Container => {
            // Forward to container
            (StatusCode::OK, format!("Proxied to container: {}", service.target))
        }
        ServiceType::StaticSite => {
            // Serve static files
            (StatusCode::OK, format!("Serving static site from: {}", service.target))
        }
        ServiceType::CustomURL => {
            // Forward to custom URL
            (StatusCode::OK, format!("Proxied to URL: {}", service.target))
        }
    }
}