//! Service proxy handler for routing requests based on domain.

use crate::app_state::AppState;
use crate::models::service::ServiceType;
use crate::docker::services;
use axum::{
    extract::State,
    http::{Request, StatusCode, Uri},
    response::IntoResponse,
    body::Body,
};
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use std::path::Path;
use tower_http::services::ServeDir;
use tower::util::ServiceExt;
use std::convert::TryFrom;

type HttpClient = Client<HttpsConnector<HttpConnector>>;

/// Handle a request to the service proxy.
pub async fn handle_proxy_request<B>(
    State(app_state): State<AppState>,
    req: Request<B>,
) -> Response
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
        None => return (StatusCode::BAD_REQUEST, "Missing host header".to_string()).into_response(),
    };

    // Remove port from host if present
    let domain = host.split(':').next().unwrap_or(host);
    
    // Find service by domain
    let service = match services::get_service_by_domain(&app_state.db, domain).await {
        Ok(Some(service)) if service.enabled => service,
        Ok(Some(_)) => return (StatusCode::SERVICE_UNAVAILABLE, "Service is disabled".to_string()).into_response(),
        Ok(None) => return (StatusCode::NOT_FOUND, format!("No service found for domain: {}", domain)).into_response(),
        Err(e) => {
            tracing::error!("Error finding service for domain {}: {}", domain, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()).into_response();
        }
    };
    
    // Forward the request based on service type
    match service.service_type {
        ServiceType::Container => {
            forward_to_container(app_state, service, req).await
        },
        ServiceType::StaticSite => {
            serve_static_site(service, req).await
        },
        ServiceType::CustomURL => {
            forward_to_url(service, req).await
        },
    }
}

/// Forward a request to a container.
async fn forward_to_container<B>(
    app_state: AppState,
    service: crate::models::service::Service,
    req: Request<B>,
) -> Response
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<axum::BoxError>,
{
    // Convert the incoming request to a hyper request with a Body
    let (parts, body) = req.into_parts();
    let body_bytes = match hyper::body::to_bytes(body.map_err(|e| e.into())).await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Failed to read request body: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read request body".to_string()).into_response();
        }
    };
    
    // Create a new request with the same method, headers, etc.
    let mut new_req = Request::builder()
        .method(parts.method)
        .uri(format!("http://{}:{}{}", service.target, service.port, parts.uri.path_and_query().map_or("", |p| p.as_str())))
        .body(Body::from(body_bytes))
        .unwrap();
    
    // Copy headers
    *new_req.headers_mut() = parts.headers;
    
    // Add custom headers from service configuration
    for (name, value) in &service.headers {
        if let Ok(header_name) = hyper::header::HeaderName::from_bytes(name.as_bytes()) {
            if let Ok(header_value) = hyper::header::HeaderValue::from_str(value) {
                new_req.headers_mut().insert(header_name, header_value);
            }
        }
    }
    
    // Create HTTP client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    
    // Forward the request
    match client.request(new_req).await {
        Ok(response) => {
            // Convert the response to an axum response
            response.into_response()
        },
        Err(e) => {
            tracing::error!("Failed to forward request to container: {}", e);
            (StatusCode::BAD_GATEWAY, format!("Failed to forward request: {}", e)).into_response()
        }
    }
}

/// Serve static files.
async fn serve_static_site<B>(
    service: crate::models::service::Service,
    req: Request<B>,
) -> Response
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<axum::BoxError>,
{
    // Check if the target directory exists
    let path = Path::new(&service.target);
    if !path.exists() || !path.is_dir() {
        return (StatusCode::NOT_FOUND, "Static site directory not found".to_string()).into_response();
    }
    
    // Create a ServeDir service
    let serve_dir = ServeDir::new(&service.target);
    
    // Extract the path from the request
    let uri_path = req.uri().path();
    let path = if uri_path == "/" {
        "/index.html"
    } else {
        uri_path
    };
    
    // Create a new request with just the path
    let uri = match Uri::try_from(path) {
        Ok(uri) => uri,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid path".to_string()).into_response(),
    };
    
    // Create a new request with the new URI
    let mut new_req = Request::builder()
        .method(req.method())
        .uri(uri);
    
    // Copy all headers from the original request
    for (name, value) in req.headers() {
        new_req = new_req.header(name, value);
    }
    
    // Build the request with the original body
    let new_req = match new_req.body(req.into_body()) {
        Ok(req) => req,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create request".to_string()).into_response();
        }
    };
    
    // Serve the file
    match serve_dir.oneshot(new_req).await {
        Ok(response) => response.into_response(),
        Err(err) => {
            tracing::error!("Failed to serve static file: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to serve static file".to_string()).into_response()
        }
    }
}

/// Forward a request to a URL.
async fn forward_to_url<B>(
    service: crate::models::service::Service,
    req: Request<B>,
) -> Response
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<axum::BoxError>,
{
    // Convert the incoming request to a hyper request with a Body
    let (parts, body) = req.into_parts();
    let body_bytes = match hyper::body::to_bytes(body.map_err(|e| e.into())).await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Failed to read request body: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read request body".to_string()).into_response();
        }
    };
    
    // Create the target URL
    let target_url = format!("{}{}",
        service.target,
        parts.uri.path_and_query().map_or("", |p| p.as_str())
    );
    
    // Create a new request with the same method, headers, etc.
    let uri = match Uri::try_from(&target_url) {
        Ok(uri) => uri,
        Err(e) => {
            tracing::error!("Invalid target URL {}: {}", target_url, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid target URL".to_string()).into_response();
        }
    };
    
    let mut new_req = Request::builder()
        .method(parts.method)
        .uri(uri)
        .body(Body::from(body_bytes))
        .unwrap();
    
    // Copy headers
    *new_req.headers_mut() = parts.headers;
    
    // Add custom headers from service configuration
    for (name, value) in &service.headers {
        if let Ok(header_name) = hyper::header::HeaderName::from_bytes(name.as_bytes()) {
            if let Ok(header_value) = hyper::header::HeaderValue::from_str(value) {
                new_req.headers_mut().insert(header_name, header_value);
            }
        }
    }
    
    // Create HTTP client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    
    // Forward the request
    match client.request(new_req).await {
        Ok(response) => {
            // Convert the response to an axum response
            response.into_response()
        },
        Err(e) => {
            tracing::error!("Failed to forward request to URL: {}", e);
            (StatusCode::BAD_GATEWAY, format!("Failed to forward request: {}", e)).into_response()
        }
    }
}

// Helper type for converting hyper responses
type Response = axum::response::Response<axum::body::BoxBody>;