//! Models for Docker Compose stacks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Docker Compose stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeStack {
    /// Unique identifier for the stack.
    pub id: String,
    /// Name of the stack.
    pub name: String,
    /// Path to the compose file.
    pub file_path: String,
    /// Status of the stack (up, down, partial).
    pub status: StackStatus,
    /// When the stack was created.
    pub created_at: DateTime<Utc>,
    /// When the stack was last updated.
    pub updated_at: DateTime<Utc>,
    /// Services in the stack.
    pub services: Vec<ComposeService>,
    /// Environment variables for the stack.
    pub environment: Option<HashMap<String, String>>,
    /// Version of the compose file.
    pub version: Option<String>,
}

/// Status of a Docker Compose stack.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StackStatus {
    /// All services are running.
    Up,
    /// All services are stopped.
    Down,
    /// Some services are running, some are stopped.
    Partial,
    /// The stack has errors.
    Error,
}

impl std::fmt::Display for StackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackStatus::Up => write!(f, "Up"),
            StackStatus::Down => write!(f, "Down"),
            StackStatus::Partial => write!(f, "Partial"),
            StackStatus::Error => write!(f, "Error"),
        }
    }
}

/// Represents a service in a Docker Compose stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeService {
    /// Name of the service.
    pub name: String,
    /// Image used by the service.
    pub image: String,
    /// Status of the service.
    pub status: String,
    /// Container ID if the service is running.
    pub container_id: Option<String>,
    /// Ports exposed by the service.
    pub ports: Option<Vec<String>>,
    /// Volumes used by the service.
    pub volumes: Option<Vec<String>>,
    /// Networks the service is connected to.
    pub networks: Option<Vec<String>>,
    /// Environment variables for the service.
    pub environment: Option<HashMap<String, String>>,
    /// Dependencies of the service.
    pub depends_on: Option<Vec<String>>,
}

/// Request to create a new Docker Compose stack.
#[derive(Debug, Deserialize)]
pub struct CreateStackRequest {
    /// Name of the stack.
    pub name: String,
    /// Content of the compose file.
    pub compose_content: String,
    /// Whether to start the stack after creation.
    pub start: bool,
    /// Environment variables for the stack.
    pub environment: Option<HashMap<String, String>>,
}

/// Request to update an existing Docker Compose stack.
#[derive(Debug, Deserialize)]
pub struct UpdateStackRequest {
    /// New content of the compose file.
    pub compose_content: String,
    /// Whether to restart the stack after update.
    pub restart: bool,
    /// Environment variables for the stack.
    pub environment: Option<HashMap<String, String>>,
}

/// Response for stack operations.
#[derive(Debug, Serialize)]
pub struct StackResponse {
    /// The stack that was operated on.
    pub stack: ComposeStack,
    /// Any messages related to the operation.
    pub messages: Vec<String>,
}

/// Request to scale services in a stack.
#[derive(Debug, Deserialize)]
pub struct ScaleStackRequest {
    /// Map of service names to desired replica counts.
    pub services: HashMap<String, u32>,
}