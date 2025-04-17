//! Web handlers for service management UI.

use crate::app_state::AppState;
use crate::docker::services;
use crate::models::service::{Service, ServiceType, SSLConfig};
// use crate::templates::HtmlTemplate; // No longer needed
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
// use askama::Template; // No longer needed
use uuid::Uuid;
use std::collections::HashMap;

// These structs are no longer used since we're using direct HTML responses
// Keeping them commented out for reference
/*
pub struct ServiceListTemplate {
    pub services: Vec<Service>,
}

pub struct ServiceDetailTemplate {
    pub service: Service,
}

pub struct ServiceEditorTemplate {
    pub is_edit: bool,
    pub service: Option<Service>,
}
*/

/// Handler for the service list page.
pub async fn service_list_handler(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match services::list_services(&app_state.db).await {
        Ok(services) => {
            // Use a direct HTML response for now
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Services</title>
                    <link rel="stylesheet" href="/css/styles.css">
                </head>
                <body>
                    <div class="container mx-auto px-4 py-8">
                        <h1 class="text-2xl font-bold mb-6">Services</h1>
                        <p>{} services found</p>
                        <div class="mt-4">
                            <a href="/services/create" class="btn btn-primary">Create Service</a>
                        </div>
                    </div>
                </body>
                </html>
                "#,
                services.len()
            )).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list services: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to list services",
            )
                .into_response()
        }
    }
}

/// Handler for the service detail page.
pub async fn service_detail_handler(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> impl IntoResponse {
    match services::get_service(&app_state.db, &service_id).await {
        Ok(Some(service)) => {
            // Use a direct HTML response for now
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Service: {}</title>
                    <link rel="stylesheet" href="/css/styles.css">
                </head>
                <body>
                    <div class="container mx-auto px-4 py-8">
                        <h1 class="text-2xl font-bold mb-6">Service: {}</h1>
                        <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg p-6">
                            <p><strong>Domain:</strong> {}</p>
                            <p><strong>Type:</strong> {}</p>
                            <p><strong>Target:</strong> {}:{}</p>
                            <p><strong>Status:</strong> {}</p>
                        </div>
                        <div class="mt-4">
                            <a href="/services" class="btn btn-secondary">Back to Services</a>
                            <a href="/services/{}/edit" class="btn btn-primary">Edit</a>
                        </div>
                    </div>
                </body>
                </html>
                "#,
                service.name,
                service.name,
                service.domain,
                service.service_type,
                service.target,
                service.port,
                if service.enabled { "Enabled" } else { "Disabled" },
                service_id
            )).into_response()
        }
        Ok(None) => Redirect::to("/services").into_response(),
        Err(e) => {
            tracing::error!("Failed to get service {}: {}", service_id, e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get service",
            )
                .into_response()
        }
    }
}

/// Handler for the service create page.
pub async fn service_create_handler(
    State(_app_state): State<AppState>,
) -> impl IntoResponse {
    // Use a direct HTML response for now
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Create Service</title>
            <link rel="stylesheet" href="/css/styles.css">
        </head>
        <body>
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-2xl font-bold mb-6">Create New Service</h1>
                <p>Service creation form would go here.</p>
                <div class="mt-4">
                    <a href="/services" class="btn btn-primary">Back to Services</a>
                </div>
            </div>
        </body>
        </html>
        "#
    )).into_response()
}

/// Handler for the service edit page.
pub async fn service_edit_handler(
    State(app_state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> impl IntoResponse {
    match services::get_service(&app_state.db, &service_id).await {
        Ok(Some(service)) => {
            // Use a direct HTML response for now
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Edit Service: {}</title>
                    <link rel="stylesheet" href="/css/styles.css">
                </head>
                <body>
                    <div class="container mx-auto px-4 py-8">
                        <h1 class="text-2xl font-bold mb-6">Edit Service: {}</h1>
                        <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg p-6">
                            <p>Service edit form would go here.</p>
                            <p>Current values:</p>
                            <ul>
                                <li>Domain: {}</li>
                                <li>Type: {}</li>
                                <li>Target: {}:{}</li>
                                <li>Status: {}</li>
                            </ul>
                        </div>
                        <div class="mt-4">
                            <a href="/services/{}" class="btn btn-secondary">Back to Service</a>
                            <a href="/services" class="btn btn-primary">All Services</a>
                        </div>
                    </div>
                </body>
                </html>
                "#,
                service.name,
                service.name,
                service.domain,
                service.service_type,
                service.target,
                service.port,
                if service.enabled { "Enabled" } else { "Disabled" },
                service_id
            )).into_response()
        }
        Ok(None) => Redirect::to("/services").into_response(),
        Err(e) => {
            tracing::error!("Failed to get service {}: {}", service_id, e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get service",
            )
                .into_response()
        }
    }
}