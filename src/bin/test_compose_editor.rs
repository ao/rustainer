//! A simple binary to test the compose editor template.

use askama::Template;
use std::collections::HashMap;

/// Template for the compose editor page.
#[derive(Template)]
#[template(path = "pages/compose_editor.html")]
struct ComposeEditorTemplate {
    is_edit: bool,
    stack: Option<ComposeStack>,
    compose_content: Option<String>,
}

/// A simplified version of the ComposeStack struct.
struct ComposeStack {
    id: String,
    name: String,
    file_path: String,
    status: String,
    services: Vec<String>,
    created_at: String,
    updated_at: String,
    environment: Option<Vec<(String, String)>>,
    version: String,
}

fn main() {
    // Test with a stack and environment variables
    let mut env_vars = Vec::new();
    env_vars.push(("DATABASE_URL".to_string(), "postgres://localhost/db".to_string()));
    env_vars.push(("PORT".to_string(), "8080".to_string()));
    
    let stack = ComposeStack {
        id: "stack1".to_string(),
        name: "Test Stack".to_string(),
        file_path: "/path/to/docker-compose.yml".to_string(),
        status: "Up".to_string(),
        services: vec!["web".to_string(), "db".to_string()],
        created_at: "2025-04-16".to_string(),
        updated_at: "2025-04-16".to_string(),
        environment: Some(env_vars),
        version: "3.8".to_string(),
    };
    
    let template_with_stack = ComposeEditorTemplate {
        is_edit: true,
        stack: Some(stack),
        compose_content: Some("version: '3.8'\nservices:\n  web:\n    image: nginx".to_string()),
    };
    
    match template_with_stack.render() {
        Ok(html) => {
            println!("Template with stack rendered successfully!");
            // Print just the first 500 characters to avoid flooding the console
            let preview = if html.len() > 500 {
                format!("{}... (truncated)", &html[0..500])
            } else {
                html
            };
            println!("{}", preview);
        },
        Err(err) => eprintln!("Failed to render template with stack: {}", err),
    }
    
    // Test without a stack
    let template_without_stack = ComposeEditorTemplate {
        is_edit: false,
        stack: None,
        compose_content: None,
    };
    
    match template_without_stack.render() {
        Ok(html) => {
            println!("\nTemplate without stack rendered successfully!");
            // Print just the first 500 characters to avoid flooding the console
            let preview = if html.len() > 500 {
                format!("{}... (truncated)", &html[0..500])
            } else {
                html
            };
            println!("{}", preview);
        },
        Err(err) => eprintln!("Failed to render template without stack: {}", err),
    }
}