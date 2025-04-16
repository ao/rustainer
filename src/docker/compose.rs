//! Docker Compose integration for Rustainer.
//!
//! This module provides functionality for managing Docker Compose stacks.

use bollard::{Docker, container::{ListContainersOptions, InspectContainerOptions}};
use serde_yaml;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use anyhow::{Result, Context, anyhow};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tokio::process::Command as TokioCommand;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use futures::StreamExt;

use crate::models::compose::{
    ComposeStack, ComposeService, StackStatus,
    CreateStackRequest, UpdateStackRequest, ScaleStackRequest
};

/// Directory where compose files are stored.
const COMPOSE_DIR: &str = "./data/compose";

/// Directory where compose environment files are stored.
const COMPOSE_ENV_DIR: &str = "./data/compose/env";

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
        environment: request.environment,
        version: None,
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
    
    // Read the stack file to get the current stack
    let mut stack = read_stack_file(&path, docker).await?;
    
    // Create a temporary script to run docker-compose up
    let script_path = create_compose_script("up", &path, &stack)?;
    
    // Run the script
    let output = TokioCommand::new(&script_path)
        .output()
        .await
        .context("Failed to execute docker-compose up command")?;
    
    // Check if the command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to start stack: {}", error);
    }
    
    // Update the stack status
    stack.status = StackStatus::Up;
    stack.updated_at = Utc::now();
    
    // Update the services with container IDs and status
    stack.services = get_stack_services(docker, &stack.name).await?;
    
    // Clean up the temporary script
    let _ = fs::remove_file(script_path);
    
    Ok(stack)
}

/// Stop a Docker Compose stack.
pub async fn stop_stack(docker: &Docker, id: &str) -> Result<ComposeStack> {
    let path = stack_path(id)?;
    
    // Ensure the stack exists
    if !path.exists() {
        anyhow::bail!("Stack not found");
    }
    
    // Read the stack file to get the current stack
    let mut stack = read_stack_file(&path, docker).await?;
    
    // Create a temporary script to run docker-compose down
    let script_path = create_compose_script("down", &path, &stack)?;
    
    // Run the script
    let output = TokioCommand::new(&script_path)
        .output()
        .await
        .context("Failed to execute docker-compose down command")?;
    
    // Check if the command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to stop stack: {}", error);
    }
    
    // Update the stack status
    stack.status = StackStatus::Down;
    stack.updated_at = Utc::now();
    
    // Update the services with empty container IDs and status
    for service in &mut stack.services {
        service.container_id = None;
        service.status = "stopped".to_string();
    }
    
    // Clean up the temporary script
    let _ = fs::remove_file(script_path);
    
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
    
    // Read the stack file to get the current stack
    let stack = read_stack_file(&path, docker).await?;
    let mut logs = HashMap::new();
    
    // Create a temporary script to run docker-compose logs
    let script_path = create_compose_script("logs", &path, &stack)?;
    
    // Run the script
    let output = TokioCommand::new(&script_path)
        .output()
        .await
        .context("Failed to execute docker-compose logs command")?;
    
    // Check if the command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to get stack logs: {}", error);
    }
    
    // Parse the logs
    let log_output = String::from_utf8_lossy(&output.stdout);
    
    // Split logs by service
    for service in &stack.services {
        let service_logs: Vec<String> = log_output
            .lines()
            .filter(|line| line.contains(&format!("{}_", service.name)))
            .map(|line| {
                // Remove the service prefix from the log line
                let parts: Vec<&str> = line.splitn(2, '|').collect();
                if parts.len() > 1 {
                    parts[1].trim().to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();
        
        logs.insert(service.name.clone(), service_logs);
    }
    
    // Clean up the temporary script
    let _ = fs::remove_file(script_path);
    
    Ok(logs)
}

/// Scale services in a Docker Compose stack.
pub async fn scale_stack(docker: &Docker, id: &str, request: ScaleStackRequest) -> Result<ComposeStack> {
    let path = stack_path(id)?;
    
    // Ensure the stack exists
    if !path.exists() {
        anyhow::bail!("Stack not found");
    }
    
    // Read the stack file to get the current stack
    let mut stack = read_stack_file(&path, docker).await?;
    
    // Create a temporary script to run docker-compose scale
    let mut script_content = format!(
        "#!/bin/bash\n\
         cd {} && \\\n\
         docker-compose -f {} -p {} scale",
        Path::new(COMPOSE_DIR).canonicalize()?.to_string_lossy(),
        path.file_name().unwrap().to_string_lossy(),
        stack.name
    );
    
    // Add each service to scale
    for (service, count) in &request.services {
        script_content.push_str(&format!(" {}={}", service, count));
    }
    
    // Write the script to a temporary file
    let script_path = Path::new(COMPOSE_DIR).join(format!("scale_{}.sh", id));
    let mut file = fs::File::create(&script_path)
        .context("Failed to create temporary script file")?;
    file.write_all(script_content.as_bytes())
        .context("Failed to write to temporary script file")?;
    
    // Make the script executable
    let mut perms = fs::metadata(&script_path)
        .context("Failed to get file metadata")?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)
        .context("Failed to set file permissions")?;
    
    // Run the script
    let output = TokioCommand::new(&script_path)
        .output()
        .await
        .context("Failed to execute docker-compose scale command")?;
    
    // Check if the command was successful
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to scale stack: {}", error);
    }
    
    // Update the stack status and services
    stack.updated_at = Utc::now();
    stack.services = get_stack_services(docker, &stack.name).await?;
    
    // Determine the overall status
    let running_services = stack.services.iter().filter(|s| s.status == "running").count();
    if running_services == 0 {
        stack.status = StackStatus::Down;
    } else if running_services == stack.services.len() {
        stack.status = StackStatus::Up;
    } else {
        stack.status = StackStatus::Partial;
    }
    
    // Clean up the temporary script
    let _ = fs::remove_file(script_path);
    
    Ok(stack)
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
        environment: None,
        version: None,
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
            
            // Extract ports
            let ports = if let Some(serde_yaml::Value::Sequence(ports)) = service.get("ports") {
                Some(ports.iter()
                    .filter_map(|p| p.as_str().map(|s| s.to_string()))
                    .collect())
            } else {
                None
            };
            
            // Extract volumes
            let volumes = if let Some(serde_yaml::Value::Sequence(volumes)) = service.get("volumes") {
                Some(volumes.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect())
            } else {
                None
            };
            
            // Extract networks
            let networks = if let Some(serde_yaml::Value::Mapping(networks)) = service.get("networks") {
                Some(networks.keys()
                    .filter_map(|k| k.as_str().map(|s| s.to_string()))
                    .collect())
            } else {
                None
            };
            
            // Extract environment variables
            let environment = if let Some(env) = service.get("environment") {
                match env {
                    serde_yaml::Value::Mapping(env_map) => {
                        let mut env_vars = HashMap::new();
                        for (k, v) in env_map {
                            if let (Some(key), Some(value)) = (k.as_str(), v.as_str()) {
                                env_vars.insert(key.to_string(), value.to_string());
                            }
                        }
                        Some(env_vars)
                    },
                    serde_yaml::Value::Sequence(env_seq) => {
                        let mut env_vars = HashMap::new();
                        for item in env_seq {
                            if let Some(env_str) = item.as_str() {
                                let parts: Vec<&str> = env_str.splitn(2, '=').collect();
                                if parts.len() == 2 {
                                    env_vars.insert(parts[0].to_string(), parts[1].to_string());
                                }
                            }
                        }
                        Some(env_vars)
                    },
                    _ => None,
                }
            } else {
                None
            };
            
            // Extract depends_on
            let depends_on = if let Some(deps) = service.get("depends_on") {
                match deps {
                    serde_yaml::Value::Sequence(deps_seq) => {
                        Some(deps_seq.iter()
                            .filter_map(|d| d.as_str().map(|s| s.to_string()))
                            .collect())
                    },
                    serde_yaml::Value::Mapping(deps_map) => {
                        Some(deps_map.keys()
                            .filter_map(|k| k.as_str().map(|s| s.to_string()))
                            .collect())
                    },
                    _ => None,
                }
            } else {
                None
            };
            
            result.push(ComposeService {
                name,
                image,
                status: "not running".to_string(),
                container_id: None,
                ports,
                volumes,
                networks,
                environment,
                depends_on,
            });
        }
    }
    
    Ok(result)
}

/// Validate compose content.
pub fn validate_compose_content(content: &str) -> Result<()> {
    // Parse the YAML content
    let yaml: serde_yaml::Value = serde_yaml::from_str(content)
        .context("Failed to parse compose file")?;
    
    // Check for services section
    if !yaml.get("services").is_some() {
        anyhow::bail!("No services found in compose file");
    }
    
    // Check compose file version
    if let Some(version) = yaml.get("version") {
        if let Some(version_str) = version.as_str() {
            // Check if the version is supported
            if !["2", "2.0", "2.1", "2.2", "2.3", "2.4", "3", "3.0", "3.1", "3.2", "3.3", "3.4", "3.5", "3.6", "3.7", "3.8", "3.9"].contains(&version_str) {
                anyhow::bail!("Unsupported Docker Compose file version: {}", version_str);
            }
        }
    }
    
    Ok(())
}

/// Create a temporary script to run a Docker Compose command.
fn create_compose_script(command: &str, compose_path: &Path, stack: &ComposeStack) -> Result<PathBuf> {
    // Ensure the compose directory exists
    ensure_compose_dir()?;
    
    // Create the script content
    let mut script_content = format!(
        "#!/bin/bash\n\
         cd {} && \\\n\
         docker-compose -f {} -p {}",
        Path::new(COMPOSE_DIR).canonicalize()?.to_string_lossy(),
        compose_path.file_name().unwrap().to_string_lossy(),
        stack.name
    );
    
    // Add environment variables if present
    if let Some(env) = &stack.environment {
        // Create an environment file
        ensure_compose_env_dir()?;
        let env_file_path = Path::new(COMPOSE_ENV_DIR).join(format!("{}.env", stack.id));
        let mut env_content = String::new();
        
        for (key, value) in env {
            env_content.push_str(&format!("{}={}\n", key, value));
        }
        
        fs::write(&env_file_path, env_content)
            .context("Failed to write environment file")?;
        
        // Add the env file to the command
        script_content.push_str(&format!(" --env-file {}", env_file_path.to_string_lossy()));
    }
    
    // Add the command
    script_content.push_str(&format!(" {}", command));
    
    // Add command-specific options
    match command {
        "up" => script_content.push_str(" -d"),
        "logs" => script_content.push_str(" --no-color"),
        _ => {}
    }
    
    // Write the script to a temporary file
    let script_path = Path::new(COMPOSE_DIR).join(format!("{}_{}.sh", command, stack.id));
    let mut file = fs::File::create(&script_path)
        .context("Failed to create temporary script file")?;
    file.write_all(script_content.as_bytes())
        .context("Failed to write to temporary script file")?;
    
    // Make the script executable
    let mut perms = fs::metadata(&script_path)
        .context("Failed to get file metadata")?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)
        .context("Failed to set file permissions")?;
    
    Ok(script_path)
}

/// Ensure the compose environment directory exists.
fn ensure_compose_env_dir() -> Result<()> {
    let path = Path::new(COMPOSE_ENV_DIR);
    if !path.exists() {
        fs::create_dir_all(path).context("Failed to create compose environment directory")?;
    }
    Ok(())
}

/// Get the services for a stack with their current status.
async fn get_stack_services(docker: &Docker, stack_name: &str) -> Result<Vec<ComposeService>> {
    // List all containers
    let options = ListContainersOptions::<String> {
        all: true,
        filters: {
            let mut filters = HashMap::new();
            filters.insert("label".to_string(), vec![format!("com.docker.compose.project={}", stack_name)]);
            filters
        },
        ..Default::default()
    };
    
    let containers = docker.list_containers(Some(options))
        .await
        .context("Failed to list containers")?;
    
    // Group containers by service
    let mut services = HashMap::new();
    
    for container in containers {
        // Extract service name from labels
        let service_name = container.labels
            .as_ref()
            .and_then(|labels| labels.get("com.docker.compose.service").cloned())
            .unwrap_or_else(|| "unknown".to_string());
        
        // Get container status
        let status = match container.state {
            Some(state) => state.to_string(),
            None => "unknown".to_string()
        };
        
        // Get container ID
        let container_id = container.id.clone();
        
        // Get container details
        let container_details = if let Some(id) = &container_id {
            docker.inspect_container(id, None::<InspectContainerOptions>)
                .await
                .ok()
        } else {
            None
        };
        
        // Extract image
        let image = container.image.unwrap_or_else(|| "unknown".to_string());
        
        // Extract ports
        let ports = if let Some(ports) = container.ports {
            Some(ports.iter()
                .filter_map(|p| {
                    // Extract port information
                    let private_port = p.private_port;
                    
                    if let Some(public_port) = p.public_port {
                        if let Some(ip_str) = p.ip.as_ref() {
                            Some(format!("{}:{}->{}", ip_str, public_port, private_port))
                        } else {
                            Some(format!("{}:{}", public_port, private_port))
                        }
                    } else {
                        Some(format!("{}", private_port))
                    }
                })
                .collect())
        } else {
            None
        };
        
        // Extract networks
        let networks = if let Some(details) = &container_details {
            if let Some(network_settings) = &details.network_settings {
                if let Some(networks) = &network_settings.networks {
                    Some(networks.keys().cloned().collect())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        // Extract volumes
        let volumes = if let Some(details) = &container_details {
            if let Some(mounts) = &details.mounts {
                Some(mounts.iter()
                    .map(|m| {
                        format!("{}:{}", m.source.clone().unwrap_or_default(), m.destination.clone().unwrap_or_default())
                    })
                    .collect())
            } else {
                None
            }
        } else {
            None
        };
        
        // Extract environment variables
        let environment = if let Some(details) = &container_details {
            if let Some(config) = &details.config {
                if let Some(env) = &config.env {
                    let mut env_vars = HashMap::new();
                    for env_str in env {
                        let parts: Vec<&str> = env_str.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            env_vars.insert(parts[0].to_string(), parts[1].to_string());
                        }
                    }
                    Some(env_vars)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        
        // Create or update service
        services.entry(service_name.clone())
            .and_modify(|service: &mut ComposeService| {
                service.status = status.clone();
                service.container_id = container_id.clone();
                if let Some(p) = ports.clone() {
                    service.ports = Some(p);
                }
                if let Some(n) = networks.clone() {
                    service.networks = Some(n);
                }
                if let Some(v) = volumes.clone() {
                    service.volumes = Some(v);
                }
                if let Some(e) = environment.clone() {
                    service.environment = Some(e);
                }
            })
            .or_insert(ComposeService {
                name: service_name,
                image,
                status,
                container_id,
                ports,
                volumes,
                networks,
                environment,
                depends_on: None,
            });
    }
    
    // Convert to vector
    let mut result: Vec<ComposeService> = services.into_values().collect();
    
    // Sort by service name
    result.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(result)
}