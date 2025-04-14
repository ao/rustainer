//! Docker Compose integration for Rustainer.
//!
//! This module provides functionality for managing Docker Compose stacks.

use bollard::Docker;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};
use uuid::Uuid;

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
}

/// Request to update an existing Docker Compose stack.
#[derive(Debug, Deserialize)]
pub struct UpdateStackRequest {
    /// New content of the compose file.
    pub compose_content: String,
    /// Whether to restart the stack after update.
    pub restart: bool,
}

/// Directory where compose files are stored.
const COMPOSE_DIR: &str = "./data/compose";

/// List all Docker Compose stacks.
pub async fn list_stacks(docker: &Docker) -> Result<Vec<ComposeStack>> {
    // Ensure the compose directory exists
    ensure_compose_dir()?;

    let mut stacks = Vec::new();
    
    // Read all files in the compose directory
    let entries = fs::read_dir(COMPOSE_DIR)
        .context("Failed to read compose directory")?;
    
    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        // Skip non-yaml files
        if !is_yaml_file(&path) {
            continue;
        }
        
        // Parse the stack file
        if let Ok(stack) = read_stack_file(&path, docker).await {
            stacks.push(stack);
        }
    }
    
    Ok(stacks)
}

/// Get a Docker Compose stack by ID.
pub async fn get_stack(docker: &Docker, id: &str) -> Result<ComposeStack> {
    let path = stack_path(id)?;
    read_stack_file(&path, docker).await
}

/// Create a new Docker Compose stack.
pub async fn create_stack(docker: &Docker, request: CreateStackRequest) -> Result<ComposeStack> {
    // Ensure the compose directory exists
    ensure_compose_dir()?;
    
    // Generate a unique ID for the stack
    let id = Uuid::new_v4().to_string();
    let path = stack_path(&id)?;
    
    // Validate the compose file
    validate_compose_content(&request.compose_content)?;
    
    // Write the compose file
    fs::write(&path, &request.compose_content)
        .context("Failed to write compose file")?;
    
    // Create the stack object
    let now = Utc::now();
    let mut stack = ComposeStack {
        id: id.clone(),
        name: request.name,
        file_path: path.to_string_lossy().to_string(),
        status: StackStatus::Down,
        created_at: now,
        updated_at: now,
        services: Vec::new(),
    };
    
    // Start the stack if requested
    if request.start {
        stack = start_stack(docker, &id).await?;
    } else {
        // Just parse the services without starting
        stack.services = parse_services_from_content(&request.compose_content)?;
    }
    
    Ok(stack)
}

/// Update an existing Docker Compose stack.
pub async fn update_stack(docker: &Docker, id: &str, request: UpdateStackRequest) -> Result<ComposeStack> {
    let path = stack_path(id)?;
    
    // Ensure the stack exists
    if !path.exists() {
        anyhow::bail!("Stack not found");
    }
    
    // Validate the compose file
    validate_compose_content(&request.compose_content)?;
    
    // Write the updated compose file
    fs::write(&path, &request.compose_content)
        .context("Failed to write compose file")?;
    
    // Update the stack object
    let mut stack = read_stack_file(&path, docker).await?;
    stack.updated_at = Utc::now();
    
    // Restart the stack if requested
    if request.restart {
        stack = restart_stack(docker, id).await?;
    } else {
        // Just update the services without restarting
        stack.services = parse_services_from_content(&request.compose_content)?;
    }
    
    Ok(stack)
}

/// Delete a Docker Compose stack.
pub async fn delete_stack(docker: &Docker, id: &str) -> Result<()> {
    let path = stack_path(id)?;
    
    // Ensure the stack exists
    if !path.exists() {
        anyhow::bail!("Stack not found");
    }
    
    // Stop the stack first
    stop_stack(docker, id).await?;
    
    // Delete the compose file
    fs::remove_file(path).context("Failed to delete compose file")?;
    
    Ok(())
}

/// Start a Docker Compose stack.
pub async fn start_stack(docker: &Docker, id: &str) -> Result<ComposeStack> {
    let path = stack_path(id)?;
    
    // Ensure the stack exists
    if !path.exists() {
        anyhow::bail!("Stack not found");
    }
    
    // TODO: Implement actual Docker Compose up command
    // For now, we'll just update the status
    let mut stack = read_stack_file(&path, docker).await?;
    stack.status = StackStatus::Up;
    stack.updated_at = Utc::now();
    
    Ok(stack)
}

/// Stop a Docker Compose stack.
pub async fn stop_stack(docker: &Docker, id: &str) -> Result<ComposeStack> {
    let path = stack_path(id)?;
    
    // Ensure the stack exists
    if !path.exists() {
        anyhow::bail!("Stack not found");
    }
    
    // TODO: Implement actual Docker Compose down command
    // For now, we'll just update the status
    let mut stack = read_stack_file(&path, docker).await?;
    stack.status = StackStatus::Down;
    stack.updated_at = Utc::now();
    
    Ok(stack)
}

/// Restart a Docker Compose stack.
pub async fn restart_stack(docker: &Docker, id: &str) -> Result<ComposeStack> {
    // Stop and then start the stack
    stop_stack(docker, id).await?;
    start_stack(docker, id).await
}

/// Get logs for a Docker Compose stack.
pub async fn get_stack_logs(docker: &Docker, id: &str) -> Result<HashMap<String, Vec<String>>> {
    let path = stack_path(id)?;
    
    // Ensure the stack exists
    if !path.exists() {
        anyhow::bail!("Stack not found");
    }
    
    // TODO: Implement actual Docker Compose logs command
    // For now, return empty logs
    let stack = read_stack_file(&path, docker).await?;
    let mut logs = HashMap::new();
    
    for service in &stack.services {
        logs.insert(service.name.clone(), vec!["Log output would appear here".to_string()]);
    }
    
    Ok(logs)
}

// Helper functions

/// Ensure the compose directory exists.
fn ensure_compose_dir() -> Result<()> {
    let path = Path::new(COMPOSE_DIR);
    if !path.exists() {
        fs::create_dir_all(path).context("Failed to create compose directory")?;
    }
    Ok(())
}

/// Check if a file is a YAML file.
fn is_yaml_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        ext == "yml" || ext == "yaml"
    } else {
        false
    }
}

/// Get the path for a stack file.
fn stack_path(id: &str) -> Result<PathBuf> {
    ensure_compose_dir()?;
    Ok(Path::new(COMPOSE_DIR).join(format!("{}.yml", id)))
}

/// Read a stack file and parse it.
async fn read_stack_file(path: &Path, docker: &Docker) -> Result<ComposeStack> {
    // Read the file content
    let content = fs::read_to_string(path)
        .context("Failed to read compose file")?;
    
    // Parse the file name to get the ID
    let id = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Get file metadata
    let metadata = fs::metadata(path)
        .context("Failed to read file metadata")?;
    
    // Parse the services
    let services = parse_services_from_content(&content)?;
    
    // TODO: Check the actual status of the services
    // For now, assume the stack is down
    let status = StackStatus::Down;
    
    // Create the stack object
    let stack = ComposeStack {
        id,
        name: path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
        file_path: path.to_string_lossy().to_string(),
        status,
        created_at: metadata.created()
            .map(|t| DateTime::<Utc>::from(t))
            .unwrap_or_else(|_| Utc::now()),
        updated_at: metadata.modified()
            .map(|t| DateTime::<Utc>::from(t))
            .unwrap_or_else(|_| Utc::now()),
        services,
    };
    
    Ok(stack)
}

/// Parse services from compose content.
fn parse_services_from_content(content: &str) -> Result<Vec<ComposeService>> {
    // Parse the YAML content
    let yaml: serde_yaml::Value = serde_yaml::from_str(content)
        .context("Failed to parse compose file")?;
    
    // Extract services
    let services = match yaml.get("services") {
        Some(services) => services,
        None => anyhow::bail!("No services found in compose file"),
    };
    
    // Parse each service
    let mut result = Vec::new();
    
    if let serde_yaml::Value::Mapping(services_map) = services {
        for (name, service) in services_map {
            let name = name.as_str().unwrap_or("unknown").to_string();
            
            let image = match service.get("image") {
                Some(serde_yaml::Value::String(image)) => image.clone(),
                _ => "unknown".to_string(),
            };
            
            result.push(ComposeService {
                name,
                image,
                status: "not running".to_string(),
                container_id: None,
            });
        }
    }
    
    Ok(result)
}

/// Validate compose content.
fn validate_compose_content(content: &str) -> Result<()> {
    // Parse the YAML content
    let yaml: serde_yaml::Value = serde_yaml::from_str(content)
        .context("Failed to parse compose file")?;
    
    // Check for services section
    if !yaml.get("services").is_some() {
        anyhow::bail!("No services found in compose file");
    }
    
    Ok(())
}