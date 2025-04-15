use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
};
use crate::app_state::AppState;

// Handler for logout
pub async fn logout_handler() -> impl IntoResponse {
    Redirect::to("/")
}

// Handler for theme toggle
pub async fn theme_toggle_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Get current theme
    let current_theme = get_theme_from_cookie(&headers).unwrap_or_else(|| {
        let theme_guard = state.theme.lock().unwrap();
        theme_guard.clone()
    });

    // Toggle theme
    let new_theme = if current_theme == "dark" {
        "light"
    } else {
        "dark"
    };

    // Update theme in state
    {
        let mut theme_guard = state.theme.lock().unwrap();
        *theme_guard = new_theme.to_string();
    }

    // Set cookie and redirect
    let mut response = Redirect::to("/").into_response();
    response.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        format!("theme={}; Path=/; Max-Age=31536000", new_theme)
            .parse()
            .unwrap(),
    );

    response
}

// Helper function to get theme from cookie
fn get_theme_from_cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::COOKIE)
        .and_then(|cookie_header| {
            let cookie_str = cookie_header.to_str().ok()?;
            cookie_str
                .split(';')
                .find_map(|cookie| {
                    let cookie = cookie.trim();
                    if cookie.starts_with("theme=") {
                        Some(cookie[6..].to_string())
                    } else {
                        None
                    }
                })
        })
}