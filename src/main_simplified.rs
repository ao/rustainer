use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

mod templates;
mod web;

#[tokio::main]
async fn main() {
    // Create web routes
    let web_router = Router::new()
        .route("/", get(web::container_list_handler))
        .route("/login", get(web::login_page_handler))
        .route("/api/auth/login", post(web::login_post_handler))
        .route("/api/auth/logout", post(web::logout_handler))
        .route("/api/theme/toggle", post(web::theme_toggle_handler));

    // Create app state (mock)
    let app_state = ();

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