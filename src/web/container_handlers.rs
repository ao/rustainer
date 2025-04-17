//! Web handlers for container management UI.

use crate::app_state::AppState;
use crate::docker;
use crate::models::container::{Container, ContainerStats, ContainerLogs};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use std::sync::Arc;

/// Handler for the container list page.
pub async fn container_list_handler(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match docker::containers::list_containers(&app_state.docker).await {
        Ok(containers) => {
            // Generate HTML for each container
            let mut containers_html = String::new();
            for container in &containers {
                // Determine status class for visual indicator
                let status_class = if container.state.to_lowercase() == "running" {
                    "bg-green-100 text-green-800"
                } else if container.state.to_lowercase() == "exited" {
                    "bg-red-100 text-red-800"
                } else {
                    "bg-yellow-100 text-yellow-800"
                };

                // Get container name (first name in the list or ID if no names)
                let container_name = if !container.names.is_empty() {
                    &container.names[0]
                } else {
                    &container.id[..12] // First 12 chars of ID
                };

                // Format created time
                let created_time = container.created.format("%Y-%m-%d %H:%M:%S").to_string();

                // Generate port mappings HTML
                let mut ports_html = String::new();
                if !container.ports.is_empty() {
                    for port in &container.ports {
                        let host_port = port.host_port.map_or_else(|| "".to_string(), |p| p.to_string());
                        ports_html.push_str(&format!(
                            "<div>{}{} → {}/{}</div>",
                            if !host_port.is_empty() { &host_port } else { "" },
                            if !host_port.is_empty() { ":" } else { "" },
                            port.container_port,
                            port.protocol
                        ));
                    }
                } else {
                    ports_html = "<div>None</div>".to_string();
                }

                containers_html.push_str(&format!(
                    r#"
                    <tr class="hover:bg-gray-50 dark:hover:bg-gray-700">
                        <td class="px-6 py-4 whitespace-nowrap">
                            <a href="/containers/{}" class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300">{}</a>
                        </td>
                        <td class="px-6 py-4 whitespace-nowrap">
                            <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full {}">{}</span>
                        </td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{}</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{}</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">{}</td>
                        <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                            <div class="flex justify-end space-x-2">
                                <a href="/containers/{}" class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300">Details</a>
                                {}
                                <a href="/api/containers/{}/logs" class="text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300">Logs</a>
                            </div>
                        </td>
                    </tr>
                    "#,
                    container.id, container_name, status_class, container.state, container.image, created_time, ports_html,
                    container.id,
                    if container.state.to_lowercase() == "running" {
                        format!(
                            r#"<a href="/api/containers/{}/stop" class="text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300">Stop</a>
                            <a href="/api/containers/{}/restart" class="text-yellow-600 hover:text-yellow-900 dark:text-yellow-400 dark:hover:text-yellow-300">Restart</a>"#,
                            container.id, container.id
                        )
                    } else {
                        format!(
                            r#"<a href="/api/containers/{}/start" class="text-green-600 hover:text-green-900 dark:text-green-400 dark:hover:text-green-300">Start</a>"#,
                            container.id
                        )
                    },
                    container.id
                ));
            }

            // Use a direct HTML response
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Containers</title>
                    <link rel="stylesheet" href="/static/css/styles.css">
                    <script src="/static/js/htmx.min.js"></script>
                    <script src="/static/js/alpine.min.js" defer></script>
                </head>
                <body class="bg-gray-100 dark:bg-gray-900">
                    <div class="container mx-auto px-4 py-8">
                        <div class="flex justify-between items-center mb-6">
                            <h1 class="text-2xl font-bold text-gray-800 dark:text-white">Containers</h1>
                            <a href="/containers/create" class="btn btn-primary">Create Container</a>
                        </div>
                        
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
                            <div class="px-4 py-5 sm:px-6 flex justify-between items-center">
                                <h2 class="text-lg font-medium text-gray-900 dark:text-white">Container List</h2>
                                <div class="flex space-x-2">
                                    <input type="text" id="container-search" placeholder="Search containers..." 
                                           class="rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white">
                                    <select id="status-filter" class="rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white">
                                        <option value="all">All</option>
                                        <option value="running">Running</option>
                                        <option value="exited">Exited</option>
                                        <option value="created">Created</option>
                                    </select>
                                </div>
                            </div>
                            <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                                <thead class="bg-gray-50 dark:bg-gray-700">
                                    <tr>
                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Name</th>
                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Status</th>
                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Image</th>
                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Created</th>
                                        <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Ports</th>
                                        <th scope="col" class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Actions</th>
                                    </tr>
                                </thead>
                                <tbody class="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700" id="container-list">
                                    {}
                                </tbody>
                            </table>
                        </div>
                    </div>

                    <script>
                        // Filter containers by search term
                        document.getElementById('container-search').addEventListener('input', function(e) {{
                            const searchTerm = e.target.value.toLowerCase();
                            const rows = document.querySelectorAll('#container-list tr');
                            const statusFilter = document.getElementById('status-filter').value;
                            
                            rows.forEach(row => {{
                                const name = row.querySelector('td:first-child').textContent.toLowerCase();
                                const status = row.querySelector('td:nth-child(2)').textContent.toLowerCase();
                                const matchesSearch = name.includes(searchTerm);
                                const matchesStatus = statusFilter === 'all' || status === statusFilter;
                                
                                row.style.display = (matchesSearch && matchesStatus) ? '' : 'none';
                            }});
                        }});
                        
                        // Filter containers by status
                        document.getElementById('status-filter').addEventListener('change', function(e) {{
                            const status = e.target.value;
                            const rows = document.querySelectorAll('#container-list tr');
                            const searchTerm = document.getElementById('container-search').value.toLowerCase();
                            
                            rows.forEach(row => {{
                                const name = row.querySelector('td:first-child').textContent.toLowerCase();
                                const rowStatus = row.querySelector('td:nth-child(2)').textContent.toLowerCase();
                                const matchesSearch = name.includes(searchTerm);
                                const matchesStatus = status === 'all' || rowStatus === status;
                                
                                row.style.display = (matchesSearch && matchesStatus) ? '' : 'none';
                            }});
                        }});
                    </script>
                </body>
                </html>
                "#,
                containers_html
            )).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list containers: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list containers",
            )
                .into_response()
        }
    }
}

/// Handler for the container detail page.
pub async fn container_detail_handler(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Get container details
    let container_result = docker::containers::list_containers(&app_state.docker).await
        .map(|containers| containers.into_iter().find(|c| c.id == id));
    
    // Get container stats
    let stats_result = docker::containers::get_container_stats(&app_state.docker, &id).await;
    
    // Get container logs
    let logs_result = docker::containers::get_container_logs(&app_state.docker, &id).await;
    
    match container_result {
        Ok(Some(container)) => {
            // Get container name (first name in the list or ID if no names)
            let container_name = if !container.names.is_empty() {
                container.names[0].clone()
            } else {
                container.id[..12].to_string() // First 12 chars of ID
            };
            
            // Determine status class for visual indicator
            let status_class = if container.state.to_lowercase() == "running" {
                "bg-green-100 text-green-800"
            } else if container.state.to_lowercase() == "exited" {
                "bg-red-100 text-red-800"
            } else {
                "bg-yellow-100 text-yellow-800"
            };
            
            // Format created time
            let created_time = container.created.format("%Y-%m-%d %H:%M:%S").to_string();
            
            // Generate port mappings HTML
            let mut ports_html = String::new();
            if !container.ports.is_empty() {
                for port in &container.ports {
                    let host_port = port.host_port.map_or_else(|| "".to_string(), |p| p.to_string());
                    ports_html.push_str(&format!(
                        "<div class='mb-1'>{}{} → {}/{}</div>",
                        if !host_port.is_empty() { &host_port } else { "" },
                        if !host_port.is_empty() { ":" } else { "" },
                        port.container_port,
                        port.protocol
                    ));
                }
            } else {
                ports_html = "<div>None</div>".to_string();
            }
            
            // Generate stats HTML if available
            let stats_html = match stats_result {
                Ok(stats) => format!(
                    r#"
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
                            <div class="px-4 py-5 sm:px-6">
                                <h3 class="text-lg font-medium text-gray-900 dark:text-white">CPU Usage</h3>
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5">
                                <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
                                    <div class="bg-blue-600 h-2.5 rounded-full" style="width: {}%"></div>
                                </div>
                                <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">{:.2}%</p>
                            </div>
                        </div>
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
                            <div class="px-4 py-5 sm:px-6">
                                <h3 class="text-lg font-medium text-gray-900 dark:text-white">Memory Usage</h3>
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5">
                                <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
                                    <div class="bg-green-600 h-2.5 rounded-full" style="width: {}%"></div>
                                </div>
                                <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">{:.2}% ({} MB)</p>
                            </div>
                        </div>
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
                            <div class="px-4 py-5 sm:px-6">
                                <h3 class="text-lg font-medium text-gray-900 dark:text-white">Network I/O</h3>
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5">
                                <p class="text-sm text-gray-500 dark:text-gray-400">In: {} MB</p>
                                <p class="text-sm text-gray-500 dark:text-gray-400">Out: {} MB</p>
                            </div>
                        </div>
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
                            <div class="px-4 py-5 sm:px-6">
                                <h3 class="text-lg font-medium text-gray-900 dark:text-white">Block I/O</h3>
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5">
                                <p class="text-sm text-gray-500 dark:text-gray-400">Read: {} MB</p>
                                <p class="text-sm text-gray-500 dark:text-gray-400">Write: {} MB</p>
                            </div>
                        </div>
                    </div>
                    "#,
                    stats.cpu_usage_percent,
                    stats.cpu_usage_percent,
                    stats.memory_usage_percent,
                    stats.memory_usage_percent,
                    stats.memory_usage_bytes / 1024 / 1024,
                    stats.network_input_bytes / 1024 / 1024,
                    stats.network_output_bytes / 1024 / 1024,
                    stats.block_input_bytes / 1024 / 1024,
                    stats.block_output_bytes / 1024 / 1024
                ),
                Err(_) => "<div class='mb-6'>Stats not available</div>".to_string(),
            };
            
            // Generate logs HTML if available
            let logs_html = match logs_result {
                Ok(logs) => {
                    let mut logs_content = String::new();
                    for log in logs.logs {
                        logs_content.push_str(&format!("<div class='mb-1'>{}</div>", log));
                    }
                    
                    // Create the logs HTML with string concatenation to avoid format issues
                    let refresh_button = format!("<button hx-get=\"/api/containers/{}/logs\" hx-target=\"#container-logs\" class=\"text-sm text-indigo-600 hover:text-indigo-900 dark:text-indigo-400 dark:hover:text-indigo-300\">Refresh</button>", id);
                    
                    format!(
                        r#"
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg mb-6">
                            <div class="px-4 py-5 sm:px-6 flex justify-between items-center">
                                <h3 class="text-lg font-medium text-gray-900 dark:text-white">Container Logs</h3>
                                {}
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5">
                                <div id="container-logs" class="font-mono text-sm bg-gray-100 dark:bg-gray-900 p-4 rounded-md overflow-auto max-h-96">
                                    {}
                                </div>
                            </div>
                        </div>
                        "#,
                        refresh_button,
                        logs_content
                    )
                },
                Err(_) => "<div class='mb-6'>Logs not available</div>".to_string(),
            };
            
            // Generate action buttons based on container state
            let action_buttons = if container.state.to_lowercase() == "running" {
                format!(
                    r#"
                    <a href="/api/containers/{}/stop" class="btn btn-danger">Stop Container</a>
                    <a href="/api/containers/{}/restart" class="btn btn-warning">Restart Container</a>
                    "#,
                    id, id
                )
            } else {
                format!(
                    r#"
                    <a href="/api/containers/{}/start" class="btn btn-success">Start Container</a>
                    "#,
                    id
                )
            };
            
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Container: {}</title>
                    <link rel="stylesheet" href="/static/css/styles.css">
                    <script src="/static/js/htmx.min.js"></script>
                    <script src="/static/js/alpine.min.js" defer></script>
                </head>
                <body class="bg-gray-100 dark:bg-gray-900">
                    <div class="container mx-auto px-4 py-8">
                        <div class="flex justify-between items-center mb-6">
                            <h1 class="text-2xl font-bold text-gray-800 dark:text-white">Container: {}</h1>
                            <div>
                                <a href="/containers" class="btn btn-secondary">Back to Containers</a>
                            </div>
                        </div>
                        
                        <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg mb-6">
                            <div class="px-4 py-5 sm:px-6">
                                <h2 class="text-lg font-medium text-gray-900 dark:text-white">Container Details</h2>
                                <p class="mt-1 max-w-2xl text-sm text-gray-500 dark:text-gray-400">ID: {}</p>
                            </div>
                            <div class="border-t border-gray-200 dark:border-gray-700">
                                <dl>
                                    <div class="bg-gray-50 dark:bg-gray-700 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">Name</dt>
                                        <dd class="mt-1 text-sm text-gray-900 dark:text-white sm:mt-0 sm:col-span-2">{}</dd>
                                    </div>
                                    <div class="bg-white dark:bg-gray-800 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">Status</dt>
                                        <dd class="mt-1 text-sm text-gray-900 dark:text-white sm:mt-0 sm:col-span-2">
                                            <span class="px-2 py-1 inline-flex text-xs leading-5 font-semibold rounded-full {}">{}</span>
                                        </dd>
                                    </div>
                                    <div class="bg-gray-50 dark:bg-gray-700 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">Image</dt>
                                        <dd class="mt-1 text-sm text-gray-900 dark:text-white sm:mt-0 sm:col-span-2">{}</dd>
                                    </div>
                                    <div class="bg-white dark:bg-gray-800 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">Created</dt>
                                        <dd class="mt-1 text-sm text-gray-900 dark:text-white sm:mt-0 sm:col-span-2">{}</dd>
                                    </div>
                                    <div class="bg-gray-50 dark:bg-gray-700 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                                        <dt class="text-sm font-medium text-gray-500 dark:text-gray-400">Ports</dt>
                                        <dd class="mt-1 text-sm text-gray-900 dark:text-white sm:mt-0 sm:col-span-2">{}</dd>
                                    </div>
                                </dl>
                            </div>
                        </div>
                        
                        {}
                        
                        {}
                        
                        <div class="flex space-x-3">
                            {}
                        </div>
                    </div>
                </body>
                </html>
                "#,
                container_name, container_name, container.id, container_name,
                status_class, container.state, container.image, created_time, ports_html,
                stats_html, logs_html, action_buttons
            )).into_response()
        }
        Ok(None) => {
            tracing::error!("Container not found: {}", id);
            Redirect::to("/containers").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to get container {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get container",
            )
                .into_response()
        }
    }
}

/// Handler for the container creation page.
pub async fn container_create_handler() -> impl IntoResponse {
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Create Container</title>
            <link rel="stylesheet" href="/static/css/styles.css">
            <script src="/static/js/alpine.min.js" defer></script>
        </head>
        <body class="bg-gray-100 dark:bg-gray-900">
            <div class="container mx-auto px-4 py-8">
                <div class="flex justify-between items-center mb-6">
                    <h1 class="text-2xl font-bold text-gray-800 dark:text-white">Create Container</h1>
                    <div>
                        <a href="/containers" class="btn btn-secondary">Back to Containers</a>
                    </div>
                </div>
                
                <div class="bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg">
                    <div class="px-4 py-5 sm:px-6">
                        <h2 class="text-lg font-medium text-gray-900 dark:text-white">Container Configuration</h2>
                    </div>
                    <div class="border-t border-gray-200 dark:border-gray-700 px-4 py-5 sm:p-6">
                        <form action="/api/containers" method="POST" class="space-y-6" x-data="{{
                            ports: [{{}}],
                            volumes: [{{}}],
                            env: [{{}}],
                            addPort() {{ this.ports.push({{}}); }},
                            removePort(index) {{ this.ports.splice(index, 1); }},
                            addVolume() {{ this.volumes.push({{}}); }},
                            removeVolume(index) {{ this.volumes.splice(index, 1); }},
                            addEnv() {{ this.env.push({{}}); }},
                            removeEnv(index) {{ this.env.splice(index, 1); }}
                        }}">
                            <div>
                                <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Container Name</label>
                                <input type="text" name="name" id="name" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white" required>
                            </div>
                            
                            <div>
                                <label for="image" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Image</label>
                                <input type="text" name="image" id="image" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white" placeholder="e.g., nginx:latest" required>
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Port Mappings</label>
                                <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">Format: host_port:container_port (e.g., 8080:80)</p>
                                
                                <template x-for="(port, index) in ports" :key="index">
                                    <div class="flex mt-2 space-x-2">
                                        <input type="text" name="ports[]" x-model="port.value" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white" placeholder="8080:80">
                                        <button type="button" @click="removePort(index)" x-show="ports.length > 1" class="inline-flex items-center p-1 border border-transparent rounded-full shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                                            </svg>
                                        </button>
                                    </div>
                                </template>
                                
                                <button type="button" @click="addPort()" class="mt-2 inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded-md text-indigo-700 bg-indigo-100 hover:bg-indigo-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                    Add Port Mapping
                                </button>
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Volume Mappings</label>
                                <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">Format: host_path:container_path (e.g., /data:/app/data)</p>
                                
                                <template x-for="(volume, index) in volumes" :key="index">
                                    <div class="flex mt-2 space-x-2">
                                        <input type="text" name="volumes[]" x-model="volume.value" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white" placeholder="/data:/app/data">
                                        <button type="button" @click="removeVolume(index)" x-show="volumes.length > 1" class="inline-flex items-center p-1 border border-transparent rounded-full shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                                            </svg>
                                        </button>
                                    </div>
                                </template>
                                
                                <button type="button" @click="addVolume()" class="mt-2 inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded-md text-indigo-700 bg-indigo-100 hover:bg-indigo-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                    Add Volume Mapping
                                </button>
                            </div>
                            
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Environment Variables</label>
                                <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">Format: KEY=VALUE (e.g., POSTGRES_PASSWORD=secret)</p>
                                
                                <template x-for="(variable, index) in env" :key="index">
                                    <div class="flex mt-2 space-x-2">
                                        <input type="text" name="env[]" x-model="variable.value" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white" placeholder="KEY=VALUE">
                                        <button type="button" @click="removeEnv(index)" x-show="env.length > 1" class="inline-flex items-center p-1 border border-transparent rounded-full shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                                            </svg>
                                        </button>
                                    </div>
                                </template>
                                
                                <button type="button" @click="addEnv()" class="mt-2 inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded-md text-indigo-700 bg-indigo-100 hover:bg-indigo-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                    Add Environment Variable
                                </button>
                            </div>
                            
                            <div class="flex justify-end">
                                <button type="submit" class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                    Create Container
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </body>
        </html>
        "#
    )).into_response()
}
