//! Application state for sharing between handlers.

use crate::auth::JwtConfig;
use crate::models::service::{CreateServiceRequest, Service, UpdateServiceRequest};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

/// Combined application state.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool.
    pub db: SqlitePool,
    /// JWT configuration.
    pub jwt_config: Arc<JwtConfig>,
}

impl AppState {
    /// Create a new application state.
    pub fn new(db: SqlitePool, jwt_config: Arc<JwtConfig>) -> Self {
        Self { db, jwt_config }
    }

    /// Get all services.
    pub async fn get_services(&self) -> Result<Vec<Service>, sqlx::Error> {
        // For now, return mock services
        let services = vec![
            Service::new(
                "Website".to_string(),
                "example.com".to_string(),
                crate::models::service::ServiceType::Container,
                "web-container".to_string(),
                80,
            ),
            Service::new(
                "API".to_string(),
                "api.example.com".to_string(),
                crate::models::service::ServiceType::Container,
                "api-container".to_string(),
                80,
            ),
            Service::new(
                "Static Site".to_string(),
                "static.example.com".to_string(),
                crate::models::service::ServiceType::StaticSite,
                "/var/www/html".to_string(),
                80,
            ),
        ];

        Ok(services)
    }

    /// Get services by domain.
    pub async fn get_services_by_domain(&self, domain: &str) -> Result<Vec<Service>, sqlx::Error> {
        let services = self.get_services().await?;
        Ok(services.into_iter().filter(|s| s.matches_domain(domain)).collect())
    }

    /// Get a service by ID.
    pub async fn get_service(&self, id: Uuid) -> Result<Option<Service>, sqlx::Error> {
        let services = self.get_services().await?;
        Ok(services.into_iter().find(|s| s.id == id))
    }

    /// Create a new service.
    pub async fn create_service(&self, request: CreateServiceRequest) -> Result<Service, sqlx::Error> {
        let service = Service::new(
            request.name,
            request.domain,
            request.service_type,
            request.target,
            request.port,
        );

        // Apply optional fields
        let mut service = service;
        if let Some(ssl) = request.ssl {
            service.ssl = ssl;
        }
        if let Some(headers) = request.headers {
            service.headers = headers;
        }

        // In a real implementation, we would save the service to the database here

        Ok(service)
    }

    /// Update a service.
    pub async fn update_service(&self, id: Uuid, request: UpdateServiceRequest) -> Result<Option<Service>, sqlx::Error> {
        let mut service = match self.get_service(id).await? {
            Some(service) => service,
            None => return Ok(None),
        };

        // Apply updates from the request
        if let Some(name) = request.name {
            service.name = name;
        }
        if let Some(domain) = request.domain {
            service.domain = domain;
        }
        if let Some(service_type) = request.service_type {
            service.service_type = service_type;
        }
        if let Some(target) = request.target {
            service.target = target;
        }
        if let Some(port) = request.port {
            service.port = port;
        }
        if let Some(ssl) = request.ssl {
            service.ssl = ssl;
        }
        if let Some(headers) = request.headers {
            service.headers = headers;
        }
        if let Some(enabled) = request.enabled {
            service.enabled = enabled;
        }

        service.updated_at = chrono::Utc::now();

        // In a real implementation, we would update the service in the database here

        Ok(Some(service))
    }

    /// Delete a service.
    pub async fn delete_service(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        // In a real implementation, we would delete the service from the database here
        Ok(true)
    }

    /// Enable a service.
    pub async fn enable_service(&self, id: Uuid) -> Result<Option<Service>, sqlx::Error> {
        self.update_service(id, UpdateServiceRequest {
            name: None,
            domain: None,
            service_type: None,
            target: None,
            port: None,
            ssl: None,
            headers: None,
            enabled: Some(true),
        }).await
    }

    /// Disable a service.
    pub async fn disable_service(&self, id: Uuid) -> Result<Option<Service>, sqlx::Error> {
        self.update_service(id, UpdateServiceRequest {
            name: None,
            domain: None,
            service_type: None,
            target: None,
            port: None,
            ssl: None,
            headers: None,
            enabled: Some(false),
        }).await
    }
}