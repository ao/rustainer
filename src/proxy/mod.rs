use axum::{
    extract::State,
    http::{Request, StatusCode, Uri},
    response::Response,
};
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use http_body_util::Full;
use std::sync::Arc;
use sqlx::SqlitePool;
use tracing::{error, info};

use crate::models::Application;

pub async fn handle_proxy_request<B>(
    State(state): State<Arc<AppState>>,
    req: Request<B>,
) -> Result<Response<Full<Bytes>>, StatusCode>
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    // Extract host from request
    let host = match req.headers().get("host") {
        Some(host) => match host.to_str() {
            Ok(host) => host.to_string(),
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        },
        None => return Err(StatusCode::BAD_REQUEST),
    };

    // Find application by domain
    let application = match find_application_by_domain(&state.db, &host).await {
        Ok(Some(app)) => app,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Check if application is enabled
    if !application.enabled {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    // Check if container ID is available
    let container_id = match application.container_id {
        Some(id) => id,
        None => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    // Get container IP address
    let container_ip = match get_container_ip(&state.docker, &container_id).await {
        Ok(ip) => ip,
        Err(_) => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    // Create target URI
    let target_uri = format!(
        "http://{}:{}{}", 
        container_ip, 
        application.container_port,
        req.uri().path_and_query().map_or("", |p| p.as_str())
    );

    // Forward request to container
    forward_request(req, &target_uri).await
}

async fn find_application_by_domain(
    pool: &SqlitePool,
    domain: &str,
) -> Result<Option<Application>, sqlx::Error> {
    sqlx::query_as::<_, Application>(
        r#"
        SELECT 
            id, 
            name, 
            domain, 
            container_id, 
            container_port, 
            enabled,
            created_at as "created_at: chrono::DateTime<chrono::Utc>",
            updated_at as "updated_at: chrono::DateTime<chrono::Utc>"
        FROM applications 
        WHERE domain = ?
        "#
    )
    .bind(domain)
    .fetch_optional(pool)
    .await
}

async fn get_container_ip(
    docker: &bollard::Docker,
    container_id: &str,
) -> Result<String, bollard::errors::Error> {
    // Inspect container to get network settings
    let container_info = docker
        .inspect_container(container_id, None::<bollard::query_parameters::InspectContainerOptions>)
        .await?;

    // Extract IP address from container info
    if let Some(network_settings) = container_info.network_settings {
        if let Some(networks) = network_settings.networks {
            // Try to get IP from bridge network first
            if let Some(bridge_network) = networks.get("bridge") {
                if let Some(ip) = &bridge_network.ip_address {
                    if !ip.is_empty() {
                        return Ok(ip.clone());
                    }
                }
            }

            // If bridge network IP is not available, use the first available IP
            for (_, network) in networks {
                if let Some(ip) = network.ip_address {
                    if !ip.is_empty() {
                        return Ok(ip);
                    }
                }
            }
        }
    }

    Err(bollard::errors::Error::IOError {
        err: std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Container IP address not found",
        ),
    })
}

async fn forward_request<B>(
    req: Request<B>,
    target_uri: &str,
) -> Result<Response<Full<Bytes>>, StatusCode>
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    // Parse target URI
    let target_uri = target_uri.parse::<Uri>().map_err(|_| StatusCode::BAD_REQUEST)?;

    // Create HTTP client
    let client = Client::builder(TokioExecutor::new())
        .build::<_, Full<Bytes>>(hyper_util::client::legacy::connect::HttpConnector::new());

    // Create new request with same method, headers, and body
    let (parts, _body) = req.into_parts();
    
    // Simplified body handling - empty body for now
    let body_bytes = Bytes::new();

    // Build new request
    let mut builder = Request::builder()
        .method(parts.method)
        .uri(target_uri);

    // Copy headers
    for (name, value) in parts.headers.iter() {
        builder = builder.header(name, value);
    }

    // Build request with body
    let forwarded_req = builder
        .body(Full::new(body_bytes))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Send request to target
    match client.request(forwarded_req).await {
        Ok(response) => {
            // Convert response back to axum response
            let (parts, _body) = response.into_parts();
            
            // Simplified body handling - empty body for now
            let body_bytes = Bytes::new();
            
            let mut builder = Response::builder()
                .status(parts.status);
            
            // Copy headers
            for (name, value) in parts.headers.iter() {
                builder = builder.header(name, value);
            }
            
            Ok(builder
                .body(Full::new(body_bytes))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
        }
        Err(e) => {
            error!("Error forwarding request: {}", e);
            Err(StatusCode::BAD_GATEWAY)
        }
    }
}

pub struct AppState {
    pub db: SqlitePool,
    pub docker: bollard::Docker,
}