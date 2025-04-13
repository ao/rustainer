use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use bollard::Docker;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod docker;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rustainer...");

    // Connect to Docker
    let docker = match Docker::connect_with_local_defaults() {
        Ok(docker) => {
            tracing::info!("Connected to Docker Engine");
            Arc::new(docker)
        }
        Err(e) => {
            tracing::error!("Failed to connect to Docker Engine: {}", e);
            return Err(anyhow::anyhow!("Failed to connect to Docker Engine"));
        }
    };

    // Verify Docker connection
    match docker.ping().await {
        Ok(_) => tracing::info!("Docker Engine is responsive"),
        Err(e) => {
            tracing::error!("Docker Engine is not responsive: {}", e);
            return Err(anyhow::anyhow!("Docker Engine is not responsive"));
        }
    }

    // Create API router
    let api_router = Router::new()
        // Container endpoints
        .route("/containers", get(api::containers::list_containers))
        .route("/containers/:id/start", post(api::containers::start_container))
        .route("/containers/:id/stop", post(api::containers::stop_container))
        .route(
            "/containers/:id/restart",
            post(api::containers::restart_container),
        )
        .route("/containers/:id/logs", get(api::containers::container_logs))
        .route("/containers/create", post(api::containers::create_container))
        .route(
            "/containers/:id/stats",
            get(api::containers::container_stats),
        )
        // Volume endpoints
        .route("/volumes", get(api::volumes::list_volumes))
        // Network endpoints
        .route("/networks", get(api::networks::list_networks))
        .with_state(docker);

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create main router
    let app = Router::new()
        .nest("/api", api_router)
        // Serve static files from the frontend directory
        .nest_service("/", ServeDir::new("/app/frontend").append_index_html_on_directories(true))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}