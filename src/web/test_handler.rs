//! Test handler for template testing.

use axum::response::{Html, IntoResponse};
use askama::Template;

/// Template for the test page.
#[derive(Template)]
#[template(path = "pages/test.html")]
struct TestTemplate {}

/// Handler for the test page.
pub async fn test_handler() -> impl IntoResponse {
    let template = TestTemplate {};
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            tracing::error!("Failed to render test template: {}", err);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response()
        }
    }
}