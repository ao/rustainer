//! Web handlers for Docker Compose UI pages.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
// use askama::Template; // No longer needed
use std::fs;

use crate::app_state::AppState;
use crate::models::compose::StackStatus;

// These structs are no longer used since we're using direct HTML responses
// Keeping them commented out for reference
/*
struct ComposeListTemplate {
    stacks: Vec<crate::models::compose::ComposeStack>,
    theme: String,
}

struct ComposeDetailTemplate {
    stack: crate::models::compose::ComposeStack,
    compose_content: String,
    env_vars: Vec<(String, String)>,
    has_env: bool,
}

struct ComposeEditorTemplate {
    name: String,
    status: String,
    has_env: bool,
    env_vars: Vec<(String, String)>,
}
*/

/// Handler for the Docker Compose stacks list page.
pub async fn compose_list_handler(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match crate::docker::compose::list_stacks(&app_state.docker).await {
        Ok(stacks) => {
            // Generate stacks HTML
            let mut stacks_html = String::new();
            for stack in &stacks {
                // Generate status class
                let status_class = match stack.status {
                    StackStatus::Up => "bg-green-100 text-green-800",
                    StackStatus::Down => "bg-red-100 text-red-800",
                    StackStatus::Partial => "bg-yellow-100 text-yellow-800",
                    StackStatus::Error => "bg-red-100 text-red-800",
                };
                
                stacks_html.push_str(&format!(
                    r#"
                    <tr class="hover:bg-gray-50 dark:hover:bg-gray-700">
                        <td class="px-6 py-4 whitespace-nowrap">
                            <a href="/compose/{}" class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300">{}</a>
                        </td>
                        <td class="px-6 py-4 whitespace-nowrap">
                            <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full {}">{}</span>
                        </td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{}</td>
                        <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                            <a href="/compose/{}/edit" class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300 mr-4">Edit</a>
                            <a href="/api/compose/{}/up" class="text-green-600 hover:text-green-900 dark:text-green-400 dark:hover:text-green-300 mr-4">Start</a>
                            <a href="/api/compose/{}/down" class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300">Stop</a>
                        </td>
                    </tr>
                    "#,
                    stack.id, stack.name, status_class, stack.status, stack.file_path, stack.id, stack.id, stack.id
                ));
            }
            
            // Use a direct HTML response for now
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Docker Compose Stacks</title>
                    <link rel="stylesheet" href="/css/styles.css">
                </head>
                <body class="bg-gray-100 dark:bg-gray-900">
                    <div class="container mx-auto px-4 py-8">
                        <div class="flex justify-between items-center mb-6">
                            <h1 class="text-2xl font-bold text-gray-800 dark:text-white">Docker Compose Stacks</h1>
                            <a href="/compose/create" class="btn btn-primary">Create Stack</a>
                        </div>
                        
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
                            <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                                <thead class="bg-gray-50 dark:bg-gray-700">
                                    <tr>
                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Name</th>
                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Status</th>
                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">File Path</th>
                                        <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Actions</th>
                                    </tr>
                                </thead>
                                <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                                    {}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </body>
                </html>
                "#,
                stacks_html
            )).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list compose stacks: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list compose stacks",
            )
                .into_response()
        }
    }
}

/// Handler for the Docker Compose stack detail page.
pub async fn compose_detail_handler(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match crate::docker::compose::get_stack(&app_state.docker, &id).await {
        Ok(stack) => {
            // Read the compose file content
            let compose_content = match fs::read_to_string(&stack.file_path) {
                Ok(content) => content,
                Err(err) => {
                    tracing::error!("Failed to read compose file: {}", err);
                    "# Failed to read compose file".to_string()
                }
            };
            
            // Generate environment variables HTML
            let mut env_vars_html = String::new();
            if let Some(env) = &stack.environment {
                env_vars_html.push_str(r#"<div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg mb-6">
                    <div class="px-4 py-5 sm:px-6">
                        <h2 class="text-lg font-medium">Environment Variables</h2>
                    </div>
                    <div class="border-t border-gray-200 dark:border-gray-700">
                        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                            <thead class="bg-gray-50 dark:bg-gray-700">
                                <tr>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Key</th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Value</th>
                                </tr>
                            </thead>
                            <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">"#);
                
                for (key, value) in env {
                    env_vars_html.push_str(&format!(
                        r#"
                        <tr>
                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white">{}</td>
                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{}</td>
                        </tr>"#,
                        key, value
                    ));
                }
                
                env_vars_html.push_str("</table></div>");
            }
            
            // Generate status class
            let status_class = match stack.status {
                StackStatus::Up => "bg-green-100 text-green-800",
                StackStatus::Down => "bg-red-100 text-red-800",
                StackStatus::Partial => "bg-yellow-100 text-yellow-800",
                StackStatus::Error => "bg-red-100 text-red-800",
            };
            
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Stack: {}</title>
                    <link rel="stylesheet" href="/css/styles.css">
                </head>
                <body class="bg-gray-100 dark:bg-gray-900">
                    <div class="container mx-auto px-4 py-8">
                        <div class="flex justify-between items-center mb-6">
                            <h1 class="text-2xl font-bold text-gray-800 dark:text-white">Stack: {}</h1>
                            <div>
                                <a href="/compose" class="btn btn-secondary mr-2">Back to Stacks</a>
                                <a href="/compose/{}/edit" class="btn btn-primary">Edit Stack</a>
                            </div>
                        </div>
                        
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg mb-6">
                            <div class="px-4 py-5 sm:px-6">
                                <h2 class="text-lg font-medium">Stack Details</h2>
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5 sm:p-6">
                                <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
                                    <div>
                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">Name</dt>
                                        <dd class="mt-1 text-sm text-gray-900 dark:text-white">{}</dd>
                                    </div>
                                    <div>
                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">Status</dt>
                                        <dd class="mt-1 text-sm">
                                            <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full {}">{}</span>
                                        </dd>
                                    </div>
                                    <div>
                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">File Path</dt>
                                        <dd class="mt-1 text-sm text-gray-900 dark:text-white">{}</dd>
                                    </div>
                                </dl>
                            </div>
                        </div>
                        
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg mb-6">
                            <div class="px-4 py-5 sm:px-6">
                                <h2 class="text-lg font-medium">Compose File</h2>
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700">
                                <pre class="p-4 overflow-x-auto text-sm">{}</pre>
                            </div>
                        </div>
                        
                        {}
                        
                        <div class="mt-6 flex space-x-3">
                            <a href="/api/compose/{}/up" class="btn btn-success">Start Stack</a>
                            <a href="/api/compose/{}/down" class="btn btn-danger">Stop Stack</a>
                            <a href="/api/compose/{}/delete" class="btn btn-warning" onclick="return confirm('Are you sure you want to delete this stack?')">Delete Stack</a>
                        </div>
                    </div>
                </body>
                </html>
                "#,
                stack.name,
                stack.name,
                stack.id,
                stack.name,
                status_class,
                stack.status,
                stack.file_path,
                compose_content,
                env_vars_html,
                stack.id,
                stack.id,
                stack.id
            )).into_response()
        }
        Err(err) => {
            tracing::error!("Failed to get compose stack {}: {}", id, err);
            (
                axum::http::StatusCode::NOT_FOUND,
                "Compose stack not found",
            )
                .into_response()
        }
    }
}

/// Handler for the Docker Compose stack creation page.
pub async fn compose_create_handler() -> impl IntoResponse {
    // Use a direct HTML response for now
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Create Docker Compose Stack</title>
            <link rel="stylesheet" href="/css/styles.css">
        </head>
        <body class="bg-gray-100 dark:bg-gray-900">
            <div class="container mx-auto px-4 py-8">
                <div class="flex justify-between items-center mb-6">
                    <h1 class="text-2xl font-bold text-gray-800 dark:text-white">Create New Stack</h1>
                    <div>
                        <a href="/compose" class="btn btn-secondary">Back to Stacks</a>
                    </div>
                </div>
                
                <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg mb-6">
                    <div class="px-4 py-5 sm:px-6">
                        <h2 class="text-lg font-medium">Stack Configuration</h2>
                    </div>
                    <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5 sm:p-6">
                        <form action="/api/compose" method="POST" class="space-y-6">
                            <div>
                                <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Stack Name</label>
                                <input type="text" name="name" id="name" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" required>
                            </div>
                            
                            <div>
                                <label for="compose_file" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Compose File</label>
                                <textarea name="compose_file" id="compose_file" rows="15" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 font-mono" placeholder="version: '3'
services:
  web:
    image: nginx:latest
    ports:
      - '80:80'"></textarea>
                            </div>
                            
                            <div>
                                <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Environment Variables</h3>
                                <div id="env-vars" class="space-y-2">
                                    <!-- Environment variables will be added here -->
                                </div>
                                <button type="button" id="add-env-var" class="mt-2 inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded-md text-indigo-700 bg-indigo-100 hover:bg-indigo-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                    Add Environment Variable
                                </button>
                            </div>
                            
                            <div class="flex justify-end">
                                <button type="submit" class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                    Create Stack
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
            
            <script src="/js/env-vars.js"></script>
        </body>
        </html>
        "#
    )).into_response()
}

/// Handler for the Docker Compose stack edit page.
pub async fn compose_edit_handler(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match crate::docker::compose::get_stack(&app_state.docker, &id).await {
        Ok(stack) => {
            // Read the compose file content
            let compose_content = match fs::read_to_string(&stack.file_path) {
                Ok(content) => content,
                Err(err) => {
                    tracing::error!("Failed to read compose file: {}", err);
                    "# Failed to read compose file".to_string()
                }
            };
            
            // Extract environment variables
            let env_vars = match &stack.environment {
                Some(env) => env.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<Vec<(String, String)>>(),
                None => Vec::new(),
            };
            
            // Generate environment variables HTML
            let mut env_vars_html = String::new();
            for (i, (key, value)) in env_vars.iter().enumerate() {
                env_vars_html.push_str(&format!(
                    r#"
                    <div class="flex space-x-2 mb-2" id="env-var-{i}">
                        <input type="text" name="env_keys[]" value="{key}" placeholder="KEY" class="w-1/3 rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500">
                        <input type="text" name="env_values[]" value="{value}" placeholder="value" class="w-2/3 rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500">
                        <button type="button" class="remove-env-var text-red-600 hover:text-red-800" onclick="document.getElementById('env-var-{i}').remove();">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </button>
                    </div>
                    "#
                ));
            }
            
            // Generate status class
            let status_class = match stack.status {
                StackStatus::Up => "bg-green-100 text-green-800",
                StackStatus::Down => "bg-red-100 text-red-800",
                StackStatus::Partial => "bg-yellow-100 text-yellow-800",
                StackStatus::Error => "bg-red-100 text-red-800",
            };
            
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Edit Stack: {}</title>
                    <link rel="stylesheet" href="/css/styles.css">
                </head>
                <body class="bg-gray-100 dark:bg-gray-900">
                    <div class="container mx-auto px-4 py-8">
                        <div class="flex justify-between items-center mb-6">
                            <h1 class="text-2xl font-bold text-gray-800 dark:text-white">Edit Stack: {}</h1>
                            <div>
                                <a href="/compose/{}" class="btn btn-secondary mr-2">Back to Stack</a>
                                <a href="/compose" class="btn btn-secondary">All Stacks</a>
                            </div>
                        </div>
                        
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg mb-6">
                            <div class="px-4 py-5 sm:px-6">
                                <h2 class="text-lg font-medium">Stack Configuration</h2>
                                <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                                    Current Status: <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full {}">{}</span>
                                </p>
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5 sm:p-6">
                                <form action="/api/compose/{}" method="POST" class="space-y-6">
                                    <input type="hidden" name="_method" value="PUT">
                                    
                                    <div>
                                        <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Stack Name</label>
                                        <input type="text" name="name" id="name" value="{}" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500" required>
                                    </div>
                                    
                                    <div>
                                        <label for="compose_file" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Compose File</label>
                                        <textarea name="compose_file" id="compose_file" rows="15" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 font-mono">{}</textarea>
                                    </div>
                                    
                                    <div>
                                        <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Environment Variables</h3>
                                        <div id="env-vars" class="space-y-2">
                                            {}
                                        </div>
                                        <button type="button" id="add-env-var" class="mt-2 inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded-md text-indigo-700 bg-indigo-100 hover:bg-indigo-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                            Add Environment Variable
                                        </button>
                                    </div>
                                    
                                    <div class="flex justify-end">
                                        <button type="submit" class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                            Save Changes
                                        </button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    </div>
                    
                    <script src="/js/env-vars-edit.js"></script>
                </body>
                </html>
                "#,
                stack.name,
                stack.name,
                stack.id,
                status_class,
                stack.status,
                stack.id,
                stack.name,
                compose_content,
                env_vars_html
            )).into_response()
        }
        Err(err) => {
            tracing::error!("Failed to get compose stack {}: {}", id, err);
            (
                axum::http::StatusCode::NOT_FOUND,
                "Compose stack not found",
            )
                .into_response()
        }
    }
}