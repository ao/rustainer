//! A simple binary to test the basic template.

use askama::Template;

/// Template for the basic page.
#[derive(Template)]
#[template(path = "pages/basic.html")]
struct BasicTemplate {
    name: String,
    status: String,
    has_env: bool,
    env_vars: Vec<(String, String)>,
}

fn main() {
    // Test with environment variables
    let mut env_vars = Vec::new();
    env_vars.push(("DATABASE_URL".to_string(), "postgres://localhost/db".to_string()));
    env_vars.push(("PORT".to_string(), "8080".to_string()));
    
    let template_with_env = BasicTemplate {
        name: "Test Stack".to_string(),
        status: "Up".to_string(),
        has_env: true,
        env_vars,
    };
    
    match template_with_env.render() {
        Ok(html) => println!("Template with env vars rendered successfully:\n{}", html),
        Err(err) => eprintln!("Failed to render template with env vars: {}", err),
    }
    
    // Test without environment variables
    let template_without_env = BasicTemplate {
        name: "Test Stack".to_string(),
        status: "Up".to_string(),
        has_env: false,
        env_vars: Vec::new(),
    };
    
    match template_without_env.render() {
        Ok(html) => println!("\nTemplate without env vars rendered successfully:\n{}", html),
        Err(err) => eprintln!("Failed to render template without env vars: {}", err),
    }
}