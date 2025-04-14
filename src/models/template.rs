use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a container template for quick deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerTemplate {
    /// Unique identifier for the template
    pub id: Uuid,
    
    /// Name of the template
    pub name: String,
    
    /// Description of the template
    pub description: String,
    
    /// Category of the template (e.g., "Database", "Web Server", etc.)
    pub category: String,
    
    /// Docker image to use for the container
    pub image: String,
    
    /// Image tag to use
    pub tag: String,
    
    /// Command to run in the container
    pub command: Option<String>,
    
    /// Environment variables for the container
    pub env: HashMap<String, String>,
    
    /// Port mappings (host:container)
    pub ports: Vec<PortMapping>,
    
    /// Volume mappings (host:container)
    pub volumes: Vec<VolumeMapping>,
    
    /// Network settings
    pub network_mode: Option<String>,
    
    /// Restart policy
    pub restart_policy: Option<String>,
    
    /// Resource limits
    pub resources: Option<ResourceLimits>,
    
    /// Labels for the container
    pub labels: HashMap<String, String>,
    
    /// Template version
    pub version: String,
    
    /// Template creation time
    pub created_at: DateTime<Utc>,
    
    /// Template last update time
    pub updated_at: DateTime<Utc>,
    
    /// User who created the template
    pub created_by: Option<String>,
}

/// Represents a port mapping for a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    /// Host port
    pub host_port: Option<u16>,
    
    /// Container port
    pub container_port: u16,
    
    /// Protocol (tcp, udp)
    pub protocol: String,
}

/// Represents a volume mapping for a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMapping {
    /// Host path
    pub host_path: String,
    
    /// Container path
    pub container_path: String,
    
    /// Read-only flag
    pub read_only: bool,
}

/// Represents resource limits for a container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// CPU limit (in cores or shares)
    pub cpu: Option<f64>,
    
    /// Memory limit (in bytes)
    pub memory: Option<u64>,
    
    /// Memory swap limit (in bytes)
    pub memory_swap: Option<u64>,
}

/// Request to create a new template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    /// Name of the template
    pub name: String,
    
    /// Description of the template
    pub description: String,
    
    /// Category of the template
    pub category: String,
    
    /// Docker image to use
    pub image: String,
    
    /// Image tag to use
    pub tag: String,
    
    /// Command to run
    pub command: Option<String>,
    
    /// Environment variables
    pub env: HashMap<String, String>,
    
    /// Port mappings
    pub ports: Vec<PortMapping>,
    
    /// Volume mappings
    pub volumes: Vec<VolumeMapping>,
    
    /// Network settings
    pub network_mode: Option<String>,
    
    /// Restart policy
    pub restart_policy: Option<String>,
    
    /// Resource limits
    pub resources: Option<ResourceLimits>,
    
    /// Labels
    pub labels: HashMap<String, String>,
}

/// Request to update an existing template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTemplateRequest {
    /// Name of the template
    pub name: Option<String>,
    
    /// Description of the template
    pub description: Option<String>,
    
    /// Category of the template
    pub category: Option<String>,
    
    /// Docker image to use
    pub image: Option<String>,
    
    /// Image tag to use
    pub tag: Option<String>,
    
    /// Command to run
    pub command: Option<String>,
    
    /// Environment variables
    pub env: Option<HashMap<String, String>>,
    
    /// Port mappings
    pub ports: Option<Vec<PortMapping>>,
    
    /// Volume mappings
    pub volumes: Option<Vec<VolumeMapping>>,
    
    /// Network settings
    pub network_mode: Option<String>,
    
    /// Restart policy
    pub restart_policy: Option<String>,
    
    /// Resource limits
    pub resources: Option<ResourceLimits>,
    
    /// Labels
    pub labels: Option<HashMap<String, String>>,
}

/// Request to deploy a container from a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployTemplateRequest {
    /// Template ID to deploy
    pub template_id: Uuid,
    
    /// Name for the deployed container
    pub name: String,
    
    /// Environment variable overrides
    pub env_override: Option<HashMap<String, String>>,
    
    /// Port mapping overrides
    pub port_override: Option<Vec<PortMapping>>,
    
    /// Volume mapping overrides
    pub volume_override: Option<Vec<VolumeMapping>>,
}