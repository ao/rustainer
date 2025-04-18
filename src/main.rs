use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post, put, delete},
    Router,
    response::{IntoResponse, Html, Redirect},
    Json,
};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod db;
mod docker;
mod models;
mod proxy;

use crate::proxy::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rustainer...");

    // Load configuration
    let config = match config::Config::from_env() {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Create a database connection pool
    let db = match db::init_db_pool(&config.database.url).await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };
    
    // Initialize default data (admin user)
    if let Err(e) = db::init_default_data(&db).await {
        tracing::error!("Failed to initialize default data: {}", e);
        std::process::exit(1);
    }

    // Connect to Docker
    let docker = match docker::connect_docker().await {
        Ok(docker) => docker,
        Err(e) => {
            tracing::error!("Failed to connect to Docker: {}", e);
            std::process::exit(1);
        }
    };
    
    // Create the app state
    let app_state = Arc::new(AppState {
        db: db.clone(),
        docker,
    });

    // Create API routes
    let api_routes = Router::new()
        // Container routes
        .route("/containers", get(api::list_containers))
        .route("/containers", post(api::create_container))
        .route("/containers/:id", get(api::get_container))
        .route("/containers/:id", delete(api::delete_container))
        .route("/containers/:id/start", post(api::start_container))
        .route("/containers/:id/stop", post(api::stop_container))
        .route("/containers/:id/restart", post(api::restart_container))
        // Image routes
        .route("/images", get(api::list_images))
        .route("/images/pull", post(api::pull_image))
        .route("/images/:id", delete(api::delete_image));

    // Create basic routes
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/login", get(login_handler))
        .route("/dashboard", get(dashboard_handler))
        .route("/applications", get(applications_handler))
        .route("/applications/create", get(application_create_handler))
        .route("/containers", get(containers_handler))
        .route("/containers/create", get(container_create_handler))
        .route("/images", get(images_handler))
        .route("/images/create", get(image_create_handler))
        .route("/settings", get(settings_handler))
        .route("/health", get(health_handler))
        // Nest API routes
        .nest("/api", api_routes)
        // Serve static files
        .nest_service("/static", ServeDir::new("src/static"))
        .with_state(app_state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
    tracing::info!("Server listening on {}", addr);
    
    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

// Basic handlers
async fn index_handler() -> impl IntoResponse {
    Redirect::to("/dashboard")
}

async fn login_handler() -> impl IntoResponse {
    Html(include_str!("static/login.html"))
}

async fn dashboard_handler() -> impl IntoResponse {
    Html(include_str!("static/dashboard.html"))
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn applications_handler() -> impl IntoResponse {
    Html(include_str!("static/applications.html"))
}

async fn application_create_handler() -> impl IntoResponse {
    Html(include_str!("static/application_create.html"))
}

async fn containers_handler() -> impl IntoResponse {
    Html(include_str!("static/containers.html"))
}

async fn container_create_handler() -> impl IntoResponse {
    Html(include_str!("static/container_create.html"))
}

async fn images_handler() -> impl IntoResponse {
    Html(include_str!("static/images.html"))
}

async fn image_create_handler() -> impl IntoResponse {
    Html(include_str!("static/image_create.html"))
}

async fn settings_handler() -> impl IntoResponse {
    Html(include_str!("static/settings.html"))
}