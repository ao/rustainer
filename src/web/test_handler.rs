//! Test handler for template testing.

use axum::response::{Html, IntoResponse};

/// Handler for the test page.
pub async fn test_handler() -> impl IntoResponse {
    // Use a direct HTML response
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Test Page</title>
        <link rel="stylesheet" href="/css/styles.css">
    </head>
    <body class="bg-gray-100 dark:bg-gray-900">
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-2xl font-bold text-gray-800 dark:text-white">Test Page</h1>
            <p class="text-gray-600 dark:text-gray-400">This is a test page.</p>
        </div>
    </body>
    </html>
    "#).into_response()
}