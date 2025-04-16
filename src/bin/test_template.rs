//! A simple binary to test templates.

use askama::Template;

/// Template for the test page.
#[derive(Template)]
#[template(path = "pages/test.html")]
struct TestTemplate {}

fn main() {
    let template = TestTemplate {};
    match template.render() {
        Ok(html) => println!("Template rendered successfully:\n{}", html),
        Err(err) => eprintln!("Failed to render template: {}", err),
    }
}