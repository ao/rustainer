use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

// Base template context
#[derive(Template)]
#[template(path = "layouts/base.html")]
pub struct BaseTemplate {
    pub title: String,
    pub theme: String,
    pub user: Option<UserContext>,
    pub active_page: String,
}

// User context for templates
pub struct UserContext {
    pub username: String,
    pub role: String,
}

// Login page template
#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
    pub theme: String,
}

// Container list template
#[derive(Template)]
#[template(path = "pages/container_list.html")]
pub struct ContainerListTemplate {
    pub title: String,
    pub theme: String,
    pub user: Option<UserContext>,
    pub active_page: String,
    pub containers: Vec<ContainerContext>,
}

// Container context for templates
pub struct ContainerContext {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
}

// Helper struct to convert Askama templates into responses
pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                eprintln!("Template error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}