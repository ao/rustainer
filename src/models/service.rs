//! Service model for domain-based routing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

/// Service type for routing requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    /// Route to a container.
    Container,
    /// Route to a static website.
    StaticSite,
    /// Route to a custom URL.
    CustomURL,
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceType::Container => write!(f, "Container"),
            ServiceType::StaticSite => write!(f, "StaticSite"),
            ServiceType::CustomURL => write!(f, "CustomURL"),
        }
    }
}

// Implement PartialEq<&str> for ServiceType
impl PartialEq<&str> for ServiceType {
    fn eq(&self, other: &&str) -> bool {
        match self {
            ServiceType::Container => *other == "Container",
            ServiceType::StaticSite => *other == "StaticSite",
            ServiceType::CustomURL => *other == "CustomURL",
        }
    }
}

/// SSL/TLS configuration for a service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSLConfig {
    /// Whether SSL is enabled.
    pub enabled: bool,
    /// Path to the certificate file.
    pub cert_path: Option<String>,
    /// Path to the key file.
    pub key_path: Option<String>,
    /// Whether to auto-generate a certificate using Let's Encrypt.
    pub auto_generate: bool,
}

/// Service model for domain-based routing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Unique identifier for the service.
    pub id: Uuid,
    /// Name of the service.
    pub name: String,
    /// Domain name for the service (e.g., example.com).
    pub domain: String,
    /// Type of service.
    pub service_type: ServiceType,
    /// Target for the service (container ID, path, or URL).
    pub target: String,
    /// Port to expose the service on.
    pub port: u16,
    /// SSL/TLS configuration.
    pub ssl: SSLConfig,
    /// Additional headers to add to requests.
    pub headers: HashMap<String, String>,
    /// Whether the service is enabled.
    pub enabled: bool,
    /// When the service was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the service was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Service {
    /// Create a new service.
    pub fn new(
        name: String,
        domain: String,
        service_type: ServiceType,
        target: String,
        port: u16,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            domain,
            service_type,
            target,
            port,
            ssl: SSLConfig {
                enabled: false,
                cert_path: None,
                key_path: None,
                auto_generate: false,
            },
            headers: HashMap::new(),
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the service matches a given domain.
    pub fn matches_domain(&self, domain: &str) -> bool {
        self.domain == domain
    }
}

/// Service creation request.
#[derive(Debug, Deserialize)]
pub struct CreateServiceRequest {
    /// Name of the service.
    pub name: String,
    /// Domain name for the service.
    pub domain: String,
    /// Type of service.
    pub service_type: ServiceType,
    /// Target for the service.
    pub target: String,
    /// Port to expose the service on.
    pub port: u16,
    /// SSL/TLS configuration.
    pub ssl: Option<SSLConfig>,
    /// Additional headers to add to requests.
    pub headers: Option<HashMap<String, String>>,
}

/// Service update request.
#[derive(Debug, Deserialize)]
pub struct UpdateServiceRequest {
    /// Name of the service.
    pub name: Option<String>,
    /// Domain name for the service.
    pub domain: Option<String>,
    /// Type of service.
    pub service_type: Option<ServiceType>,
    /// Target for the service.
    pub target: Option<String>,
    /// Port to expose the service on.
    pub port: Option<u16>,
    /// SSL/TLS configuration.
    pub ssl: Option<SSLConfig>,
    /// Additional headers to add to requests.
    pub headers: Option<HashMap<String, String>>,
    /// Whether the service is enabled.
    pub enabled: Option<bool>,
}

/// Service response.
#[derive(Debug, Serialize)]
pub struct ServiceResponse {
    /// Unique identifier for the service.
    pub id: Uuid,
    /// Name of the service.
    pub name: String,
    /// Domain name for the service.
    pub domain: String,
    /// Type of service.
    pub service_type: ServiceType,
    /// Target for the service.
    pub target: String,
    /// Port to expose the service on.
    pub port: u16,
    /// SSL/TLS configuration.
    pub ssl: SSLConfig,
    /// Additional headers to add to requests.
    pub headers: HashMap<String, String>,
    /// Whether the service is enabled.
    pub enabled: bool,
    /// When the service was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the service was last updated.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Service> for ServiceResponse {
    fn from(service: Service) -> Self {
        Self {
            id: service.id,
            name: service.name,
            domain: service.domain,
            service_type: service.service_type,
            target: service.target,
            port: service.port,
            ssl: service.ssl,
            headers: service.headers,
            enabled: service.enabled,
            created_at: service.created_at,
            updated_at: service.updated_at,
        }
    }
}