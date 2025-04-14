use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post, delete},
    Router,
};
use bollard::Docker;
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::extract::Extension;

mod api;
mod app_state;
mod auth;
mod config;
mod db;
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

    // Load configuration
    let config = config::Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Initialize database
    let db_pool = db::init_db_pool(&config.database.url).await?;
    tracing::info!("Database initialized");

    // Initialize default templates
    docker::templates::init_default_templates().await?;
    tracing::info!("Default templates initialized");

    // Initialize default data (admin user)
    db::init_default_data(&db_pool).await?;
    tracing::info!("Default data initialized");
    
    // Create JWT configuration
    let jwt_config = Arc::new(config.create_jwt_config());
    tracing::info!("JWT configuration created");

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

    // Create application state
    let app_state = app_state::AppState::new(db_pool.clone(), jwt_config.clone());
    tracing::info!("Application state created");

    // Create auth router
    let auth_router = Router::new()
        .route("/login", post(auth::handlers::login))
        .route("/me", get(auth::handlers::get_current_user))
        .route("/users", get(auth::handlers::get_users))
        .with_state(app_state.clone());

    // Create API router with basic endpoints
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
        .route("/networks", post(api::networks::create_network))
        .route("/networks/:id", get(api::networks::get_network))
        .route("/networks/:id", delete(api::networks::delete_network))
        .route("/networks/:id/connect", post(api::networks::connect_container))
        .route("/networks/:id/disconnect", post(api::networks::disconnect_container))
        .route("/networks/prune", post(api::networks::prune_networks))
        .route("/networks/:id/diagnostics", get(api::networks::get_network_diagnostics))
        // Docker Compose endpoints
        .route("/compose", get(api::compose::list_compose_stacks))
        .route("/compose", post(api::compose::create_compose_stack))
        .route("/compose/:id", get(api::compose::get_compose_stack))
        .route("/compose/:id", post(api::compose::update_compose_stack))
        .route("/compose/:id", delete(api::compose::delete_compose_stack))
        .route("/compose/:id/start", post(api::compose::start_compose_stack))
        .route("/compose/:id/stop", post(api::compose::stop_compose_stack))
        .route("/compose/:id/restart", post(api::compose::restart_compose_stack))
        .route("/compose/:id/logs", get(api::compose::get_compose_stack_logs))
        // Template endpoints
        .route("/templates", get(api::templates::list_templates))
        .route("/templates", post(api::templates::create_template))
        .route("/templates/:id", get(api::templates::get_template))
        .route("/templates/:id", post(api::templates::update_template))
        .route("/templates/:id", delete(api::templates::delete_template))
        .route("/templates/deploy", post(api::templates::deploy_template))
        .with_state(docker.clone())
        .nest("/auth", auth_router);

    // Create services API router
    let services_router = Router::new()
        .route("/services", get(api::services::list_services))
        .route("/services", post(api::services::create_service))
        .route("/services/:id", get(api::services::get_service))
        .route("/services/:id", post(api::services::update_service))
        .route("/services/:id", delete(api::services::delete_service))
        .route("/services/:id/enable", post(api::services::enable_service))
        .route("/services/:id/disable", post(api::services::disable_service))
        .with_state(app_state.clone());

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Combine API routers
    let combined_api_router = Router::new()
        .merge(api_router)
        .merge(services_router);

    // Create admin UI router
    let admin_app = Router::new()
        .nest("/api", combined_api_router.clone())
        // Serve static files from the frontend directory with fallback for SPA routes
        .nest_service(
            "/",
            ServeDir::new("frontend/build")
                .append_index_html_on_directories(true)
                .fallback(tower::service_fn(|_req| async {
                    // Serve index.html for any route that doesn't match a file
                    let index_path = "frontend/build/index.html";
                    match tokio::fs::read(index_path).await {
                        Ok(content) => Ok(axum::http::Response::builder()
                            .status(axum::http::StatusCode::OK)
                            .header("Content-Type", "text/html")
                            .body(axum::body::boxed(axum::body::Full::from(content)))
                            .unwrap()),
                        Err(err) => {
                            tracing::error!("Failed to read index.html: {}", err);
                            Ok(axum::http::Response::builder()
                                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                                .body(axum::body::boxed(axum::body::Full::from("Internal Server Error")))
                                .unwrap())
                        }
                    }
                }))
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors.clone());

    // Create service proxy router
    let service_app = Router::new()
        .fallback(api::proxy::handle_proxy_request)
        .with_state(app_state.clone())
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Start the admin UI server on ports 801/4431
    let admin_addr = SocketAddr::new(
        config.server.host.parse()?,
        801, // Use port 801 for HTTP
    );
    tracing::info!("Admin UI listening on {}", admin_addr);

    // Start the service proxy server on ports 80/443
    let service_addr = SocketAddr::new(
        config.server.host.parse()?,
        80, // Use port 80 for HTTP
    );
    tracing::info!("Service proxy listening on {}", service_addr);

    // Start both servers concurrently
    tokio::select! {
        result = axum::Server::bind(&admin_addr).serve(admin_app.into_make_service()) => {
            if let Err(e) = result {
                tracing::error!("Admin UI server error: {}", e);
                return Err(anyhow::anyhow!("Admin UI server error: {}", e));
            }
        }
        result = axum::Server::bind(&service_addr).serve(service_app.into_make_service()) => {
            if let Err(e) = result {
                tracing::error!("Service proxy server error: {}", e);
                return Err(anyhow::anyhow!("Service proxy server error: {}", e));
            }
        }
    }

    Ok(())
}