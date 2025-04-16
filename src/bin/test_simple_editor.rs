//! A simple binary to test the simple editor template.

use askama::Template;

/// Template for the simple editor page.
#[derive(Template)]
#[template(path = "pages/simple_editor.html")]
struct SimpleEditorTemplate {
    is_edit: bool,
    stack: Option<ComposeStack>,
    compose_content: Option<String>,
}

/// A simplified version of the ComposeStack struct.
struct ComposeStack {
    name: String,
    status: String,
    environment: Option<Vec<(String, String)>>,
}

fn main() {
    // Test with a stack and environment variables
    let mut env_vars = Vec::new();
    env_vars.push(("DATABASE_URL".to_string(), "postgres://localhost/db".to_string()));
    env_vars.push(("PORT".to_string(), "8080".to_string()));
    
    let stack = ComposeStack {
        name: "Test Stack".to_string(),
        status: "Up".to_string(),
        environment: Some(env_vars),
    };
    
    let template_with_stack = SimpleEditorTemplate {
        is_edit: true,
        stack: Some(stack),
        compose_content: Some("version: '3.8'\nservices:\n  web:\n    image: nginx".to_string()),
    };
    
    match template_with_stack.render() {
        Ok(html) => println!("Template with stack rendered successfully:\n{}", html),
        Err(err) => eprintln!("Failed to render template with stack: {}", err),
    }
    
    // Test without a stack
    let template_without_stack = SimpleEditorTemplate {
        is_edit: false,
        stack: None,
        compose_content: None,
    };
    
    match template_without_stack.render() {
        Ok(html) => println!("\nTemplate without stack rendered successfully:\n{}", html),
        Err(err) => eprintln!("Failed to render template without stack: {}", err),
    }
}