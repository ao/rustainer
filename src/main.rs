use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{get, post, put, delete},
    Router,
    response::{IntoResponse, Html, Redirect},
    extract::State,
};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
mod web;
mod websocket;
mod config;
mod auth;
mod models;
mod db;
mod docker;
mod api;
mod templates;

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

    // Create JWT config
    let jwt_config = Arc::new(config.create_jwt_config());

    // Create a database connection pool
    let db = match crate::db::init_db_pool("sqlite:data/rustainer.db").await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };
    
    // Initialize default data (admin user)
    if let Err(e) = crate::db::init_default_data(&db).await {
        tracing::error!("Failed to initialize default data: {}", e);
        std::process::exit(1);
    }
    
    // Create the app state
    let app_state = app_state::AppState::new(db, jwt_config.clone());

    // Create Docker event listener using the Docker client from app_state
    let docker_event_listener = websocket::DockerEventListener::with_docker(
        app_state.ws_manager.clone(),
        (*app_state.docker).clone(),
    );

    // Start Docker event listener in a separate task
    let docker_event_task = tokio::spawn(async move {
        if let Err(e) = docker_event_listener.start().await {
            tracing::error!("Docker event listener error: {}", e);
        }
    });

    // Create proxy router for port 80
    let proxy_router = Router::new()
        .fallback(api::proxy::handle_proxy_request)
        .with_state(app_state.clone());

    // Start the proxy server on port 80
    let proxy_addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("Proxy server listening on {}", proxy_addr);

    // Start the proxy server in a separate task
    let proxy_server = tokio::spawn(async move {
        if let Err(e) = axum::Server::bind(&proxy_addr)
            .serve(proxy_router.into_make_service())
            .await
        {
            tracing::error!("Proxy server error: {}", e);
        }
    });

    // Create public auth routes (no auth required)
    let public_auth_routes = Router::new()
        .route("/login", post(auth::handlers::login))
        .route("/login-json", post(auth::handlers::login_json));

    // Create protected auth routes (auth required)
    let protected_auth_routes = Router::new()
        .route("/logout", post(auth::handlers::logout))
        .route("/me", get(auth::handlers::get_current_user))
        .route("/users", get(auth::handlers::get_users));

    // Create API routes for Docker Compose
    let compose_api_router = Router::new()
        .route("/", get(crate::api::compose::list_compose_stacks))
        .route("/", post(crate::api::compose::create_compose_stack))
        .route("/validate", post(crate::api::compose::validate_compose_file))
        .route("/:id", get(crate::api::compose::get_compose_stack))
        .route("/:id", put(crate::api::compose::update_compose_stack))
        .route("/:id/delete", post(crate::api::compose::delete_compose_stack))
        .route("/:id/start", post(crate::api::compose::start_compose_stack))
        .route("/:id/stop", post(crate::api::compose::stop_compose_stack))
        .route("/:id/restart", post(crate::api::compose::restart_compose_stack))
        .route("/:id/logs", get(crate::api::compose::get_compose_stack_logs))
        .route("/:id/scale", post(crate::api::compose::scale_compose_stack));

    // Create API routes for Services
    let services_api_router = Router::new()
        .route("/", get(crate::api::services::list_services))
        .route("/", post(crate::api::services::create_service))
        .route("/:id", get(crate::api::services::get_service))
        .route("/:id", put(crate::api::services::update_service))
        .route("/:id", delete(crate::api::services::delete_service))
        .route("/:id/enable", post(crate::api::services::enable_service))
        .route("/:id/disable", post(crate::api::services::disable_service));

    // Create API routes for Containers
    let containers_api_router = Router::new()
        .route("/", get(crate::api::containers::list_containers))
        .route("/", post(crate::api::containers::create_container))
        .route("/:id/start", post(crate::api::containers::start_container))
        .route("/:id/stop", post(crate::api::containers::stop_container))
        .route("/:id/restart", post(crate::api::containers::restart_container))
        .route("/:id/logs", get(crate::api::containers::container_logs))
        .route("/:id/stats", get(crate::api::containers::container_stats));

    // Create web routes
    let web_router = Router::new()
        .route("/api/theme/toggle", post(web::handlers::theme_toggle_handler))
        .route("/ws", get(websocket::ws_handler))
        .route("/test", get(web::test_handler))
        .nest("/api/auth", protected_auth_routes)
        .nest("/api/compose", compose_api_router)
        .nest("/api/services", services_api_router)
        .nest("/api/containers", containers_api_router);

    // Create a handler for serving the login page
    async fn serve_login_page() -> axum::response::Html<String> {
        let content = std::fs::read_to_string("src/static/login.html").unwrap();
        axum::response::Html(content)
    }
    
    // Create a handler for serving the user management page
    async fn serve_user_management_page() -> axum::response::Html<String> {
        let content = std::fs::read_to_string("src/static/user-management.html").unwrap();
        axum::response::Html(content)
    }
    

    // Create a handler for serving the index.html file for any route
    async fn serve_index_html() -> axum::response::Html<String> {
        let content = std::fs::read_to_string("src/static/index.html").unwrap();
        axum::response::Html(content)
    }

    // Create public routes (no auth required)
    let public_routes = Router::new()
        .route("/login", get(serve_login_page))
        .nest("/api/auth", public_auth_routes);
    
    // Create protected routes (auth required)
    let protected_routes = Router::new()
        .merge(web_router)
        // Serve index.html for authenticated routes
        .route("/applications", get(serve_index_html))
        // Container UI routes
        .route("/containers", get(web::container_handlers::container_list_handler))
        .route("/containers/:id", get(web::container_handlers::container_detail_handler))
        .route("/containers/create", get(web::container_handlers::container_create_handler))
        // Other routes
        .route("/images", get(serve_index_html))
        .route("/volumes", get(serve_index_html))
        .route("/networks", get(serve_index_html))
        .route("/users", get(serve_user_management_page))
        // Docker Compose UI routes
        .route("/compose", get(web::compose_list_handler))
        .route("/compose/create", get(web::compose_create_handler))
        .route("/compose/:id", get(web::compose_detail_handler))
        .route("/compose/:id/edit", get(web::compose_edit_handler))
        // Service routing UI routes
        .route("/services", get(web::service_list_handler))
        .route("/services/create", get(web::service_create_handler))
        .route("/services/:id", get(web::service_detail_handler))
        .route("/services/:id/edit", get(web::service_edit_handler))
        // Add auth middleware
        .layer(axum::middleware::from_fn_with_state(
            jwt_config.clone(),
            auth::middleware::require_auth,
        ));
    
    // Combine all routes
    let app = Router::new()
        // Root route handler - redirects to dashboard if authenticated, login if not
        .route("/", get(|req: axum::http::Request<axum::body::Body>| async move {
            // Extract token from Authorization header or cookie
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_string())
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    req.headers()
                        .get("Cookie")
                        .and_then(|cookie_header| cookie_header.to_str().ok())
                        .and_then(|cookie_str| {
                            for cookie in cookie_str.split(';') {
                                let cookie = cookie.trim();
                                if cookie.starts_with("auth_token=") {
                                    return Some(cookie[11..].to_string());
                                }
                            }
                            None
                        })
                });
            
            // If no valid token, redirect to login
            if token.is_none() {
                return Redirect::to("/login").into_response();
            }
            
            // Redirect to dashboard (token validation will happen in protected routes)
            Redirect::to("/dashboard").into_response()
        }))
        .merge(public_routes)
        .merge(protected_routes)
        // Serve static files
        .nest_service("/static", ServeDir::new("src/static"))
        // Fallback route redirects to login if not authenticated
        .fallback(|req: axum::http::Request<axum::body::Body>| async move {
            // Extract token from Authorization header or cookie
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_string())
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    req.headers()
                        .get("Cookie")
                        .and_then(|cookie_header| cookie_header.to_str().ok())
                        .and_then(|cookie_str| {
                            for cookie in cookie_str.split(';') {
                                let cookie = cookie.trim();
                                if cookie.starts_with("auth_token=") {
                                    return Some(cookie[11..].to_string());
                                }
                            }
                            None
                        })
                });
            
            // If no valid token, redirect to login
            if token.is_none() {
                return Redirect::to("/login").into_response();
            }
            
            // If authenticated, serve the index.html
            let content = std::fs::read_to_string("src/static/index.html").unwrap();
            Html(content).into_response()
        })
        .with_state(app_state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));
    tracing::info!("Server listening on {}", addr);
    
    // Start the server
    if let Err(e) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        tracing::error!("Server error: {}", e);
    }

    // Wait for Docker event listener to finish
    if let Err(e) = docker_event_task.await {
        tracing::error!("Docker event task error: {}", e);
    }
}