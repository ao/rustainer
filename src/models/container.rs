use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Docker container
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Container {
    /// Container ID
    pub id: String,
    /// Container names (including leading slash)
    pub names: Vec<String>,
    /// Container image
    pub image: String,
    /// Container status (e.g., "running", "exited")
    pub status: String,
    /// Container state (e.g., "running", "exited")
    pub state: String,
    /// Container creation time
    pub created: DateTime<Utc>,
    /// Container ports
    pub ports: Vec<PortMapping>,
}

/// Represents a port mapping for a container
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortMapping {
    /// Container port
    pub container_port: u16,
    /// Host port
    pub host_port: Option<u16>,
    /// Protocol (e.g., "tcp", "udp")
    pub protocol: String,
}

/// Request to create a new container
#[derive(Debug, Deserialize)]
pub struct CreateContainerRequest {
    /// Image name
    pub image: String,
    /// Container name
    pub name: String,
    /// Port mappings (host_port:container_port)
    pub ports: Option<Vec<String>>,
    /// Environment variables
    pub env: Option<Vec<String>>,
    /// Volume mappings (host_path:container_path)
    pub volumes: Option<Vec<String>>,
}

/// Container resource statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerStats {
    /// Container ID
    pub id: String,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Network input in bytes
    pub network_input_bytes: u64,
    /// Network output in bytes
    pub network_output_bytes: u64,
    /// Block input in bytes
    pub block_input_bytes: u64,
    /// Block output in bytes
    pub block_output_bytes: u64,
    /// Process count
    pub process_count: u64,
}

/// Container logs
#[derive(Debug, Serialize)]
pub struct ContainerLogs {
    /// Container ID
    pub id: String,
    /// Log lines
    pub logs: Vec<String>,
}