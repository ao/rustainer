//! A simple binary to test the simple list template.

use askama::Template;

/// Template for the simple list page.
#[derive(Template)]
#[template(path = "pages/simple_list.html")]
struct SimpleListTemplate {
    items: Vec<String>,
}

fn main() {
    let items = vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
        "Item 4".to_string(),
        "Item 5".to_string(),
    ];
    
    let template = SimpleListTemplate { items };
    match template.render() {
        Ok(html) => println!("Template rendered successfully:\n{}", html),
        Err(err) => eprintln!("Failed to render template: {}", err),
    }
}