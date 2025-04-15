use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::web::app_state::AppState;
use crate::web::handlers::{
    container_list_handler, login_page_handler, login_post_handler, logout_handler, theme_toggle_handler
};

mod web;

#[tokio::main]
async fn main() {
    println!("Starting Rustainer...");

    // Create app state
    let app_state = AppState::new();

    // Create web routes
    let web_router = Router::new()
        .route("/", get(container_list_handler))
        .route("/login", get(login_page_handler))
        .route("/api/auth/login", post(login_post_handler))
        .route("/api/auth/logout", post(logout_handler))
        .route("/api/theme/toggle", post(theme_toggle_handler));

    // Create admin UI router
    let app = Router::new()
        .merge(web_router)
        // Serve static files
        .nest_service("/static", ServeDir::new("src/static"))
        .with_state(app_state);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}