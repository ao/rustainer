use anyhow::Result;
use bollard::Docker;
use bollard::container::{
    Config, CreateContainerOptions, StartContainerOptions,
};
use bollard::models::{HostConfig, PortBinding};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{
    ContainerTemplate, CreateTemplateRequest, UpdateTemplateRequest,
    DeployTemplateRequest
};
use crate::models::template::{PortMapping, VolumeMapping, ResourceLimits};

/// In-memory storage for templates (in a real application, this would be a database)
static mut TEMPLATES: Option<HashMap<Uuid, ContainerTemplate>> = None;

/// Initialize the templates storage
fn init_templates() -> &'static mut HashMap<Uuid, ContainerTemplate> {
    unsafe {
        if TEMPLATES.is_none() {
            TEMPLATES = Some(HashMap::new());
        }
        TEMPLATES.as_mut().unwrap()
    }
}

/// Get a reference to the templates storage
fn get_templates() -> &'static mut HashMap<Uuid, ContainerTemplate> {
    unsafe {
        if TEMPLATES.is_none() {
            init_templates();
        }
        TEMPLATES.as_mut().unwrap()
    }
}

/// List all available container templates
pub async fn list_templates() -> Result<Vec<ContainerTemplate>> {
    let templates = get_templates();
    let result = templates.values().cloned().collect();
    Ok(result)
}

/// Get a specific template by ID
pub async fn get_template(id: &Uuid) -> Result<Option<ContainerTemplate>> {
    let templates = get_templates();
    Ok(templates.get(id).cloned())
}

/// Create a new container template
pub async fn create_template(request: CreateTemplateRequest) -> Result<ContainerTemplate> {
    let templates = get_templates();
    
    let id = Uuid::new_v4();
    let now = Utc::now();
    
    let template = ContainerTemplate {
        id,
        name: request.name,
        description: request.description,
        category: request.category,
        image: request.image,
        tag: request.tag,
        command: request.command,
        env: request.env,
        ports: request.ports,
        volumes: request.volumes,
        network_mode: request.network_mode,
        restart_policy: request.restart_policy,
        resources: request.resources,
        labels: request.labels,
        version: "1.0".to_string(),
        created_at: now,
        updated_at: now,
        created_by: None,
    };
    
    templates.insert(id, template.clone());
    
    Ok(template)
}

/// Update an existing template
pub async fn update_template(id: &Uuid, request: UpdateTemplateRequest) -> Result<Option<ContainerTemplate>> {
    let templates = get_templates();
    
    if let Some(template) = templates.get_mut(id) {
        if let Some(name) = request.name {
            template.name = name;
        }
        
        if let Some(description) = request.description {
            template.description = description;
        }
        
        if let Some(category) = request.category {
            template.category = category;
        }
        
        if let Some(image) = request.image {
            template.image = image;
        }
        
        if let Some(tag) = request.tag {
            template.tag = tag;
        }
        
        if let Some(command) = request.command {
            template.command = Some(command);
        }
        
        if let Some(env) = request.env {
            template.env = env;
        }
        
        if let Some(ports) = request.ports {
            template.ports = ports;
        }
        
        if let Some(volumes) = request.volumes {
            template.volumes = volumes;
        }
        
        if let Some(network_mode) = request.network_mode {
            template.network_mode = Some(network_mode);
        }
        
        if let Some(restart_policy) = request.restart_policy {
            template.restart_policy = Some(restart_policy);
        }
        
        if let Some(resources) = request.resources {
            template.resources = Some(resources);
        }
        
        if let Some(labels) = request.labels {
            template.labels = labels;
        }
        
        template.updated_at = Utc::now();
        template.version = format!("{:.1}", template.version.parse::<f32>().unwrap_or(1.0) + 0.1);
        
        Ok(Some(template.clone()))
    } else {
        Ok(None)
    }
}

/// Delete a template
pub async fn delete_template(id: &Uuid) -> Result<bool> {
    let templates = get_templates();
    Ok(templates.remove(id).is_some())
}

/// Deploy a container from a template
pub async fn deploy_from_template(
    docker: Arc<Docker>,
    request: DeployTemplateRequest,
) -> Result<String> {
    let templates = get_templates();
    
    let template = match templates.get(&request.template_id) {
        Some(t) => t,
        None => return Err(anyhow::anyhow!("Template not found")),
    };
    
    // Prepare environment variables
    let mut env = template.env.clone();
    if let Some(env_override) = request.env_override {
        for (key, value) in env_override {
            env.insert(key, value);
        }
    }
    let env_vec: Vec<String> = env.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
    
    // Prepare port bindings
    let ports_to_use = request.port_override.as_ref().unwrap_or(&template.ports);
    let mut port_bindings = HashMap::new();
    for port in ports_to_use {
        let container_port = format!("{}/{}", port.container_port, port.protocol);
        let host_binding = vec![PortBinding {
            host_ip: Some(String::from("0.0.0.0")),
            host_port: match port.host_port {
                Some(p) => Some(p.to_string()),
                None => None,
            },
        }];
        port_bindings.insert(container_port, host_binding);
    }
    
    // Prepare volume bindings
    let volumes_to_use = request.volume_override.as_ref().unwrap_or(&template.volumes);
    let mut binds = Vec::new();
    for volume in volumes_to_use {
        let mut bind = format!("{}:{}", volume.host_path, volume.container_path);
        if volume.read_only {
            bind.push_str(":ro");
        }
        binds.push(bind);
    }
    
    // Prepare resource limits
    // Convert port_bindings to the format expected by bollard
    let port_bindings_bollard: HashMap<String, Option<Vec<PortBinding>>> = port_bindings
        .into_iter()
        .map(|(k, v)| (k, Some(v)))
        .collect();

    let mut host_config = HostConfig {
        port_bindings: Some(port_bindings_bollard),
        binds: Some(binds),
        network_mode: template.network_mode.clone(),
        restart_policy: template.restart_policy.as_ref().map(|policy| {
            let name = match policy.as_str() {
                "no" => bollard::models::RestartPolicyNameEnum::NO,
                "always" => bollard::models::RestartPolicyNameEnum::ALWAYS,
                "unless-stopped" => bollard::models::RestartPolicyNameEnum::UNLESS_STOPPED,
                "on-failure" => bollard::models::RestartPolicyNameEnum::ON_FAILURE,
                _ => bollard::models::RestartPolicyNameEnum::NO,
            };
            bollard::models::RestartPolicy {
                name: Some(name),
                maximum_retry_count: Some(0),
            }
        }),
        ..Default::default()
    };
    
    if let Some(resources) = &template.resources {
        if let Some(cpu) = resources.cpu {
            host_config.nano_cpus = Some((cpu * 1_000_000_000.0) as i64);
        }
        
        if let Some(memory) = resources.memory {
            host_config.memory = Some(memory as i64);
        }
        
        if let Some(memory_swap) = resources.memory_swap {
            host_config.memory_swap = Some(memory_swap as i64);
        }
    }
    
    // Create container config
    let config = Config {
        image: Some(format!("{}:{}", template.image, template.tag)),
        cmd: template.command.as_ref().map(|cmd| cmd.split_whitespace().map(String::from).collect()),
        env: Some(env_vec),
        host_config: Some(host_config),
        labels: Some(template.labels.clone()),
        ..Default::default()
    };
    
    // Create the container
    let options = CreateContainerOptions {
        name: request.name,
        ..Default::default()
    };
    
    let response = docker.create_container(Some(options), config).await?;
    
    // Start the container
    docker.start_container(&response.id, None::<StartContainerOptions<String>>).await?;
    
    Ok(response.id)
}

/// Initialize default templates
pub async fn init_default_templates() -> Result<()> {
    let templates = get_templates();
    
    // Only initialize if empty
    if !templates.is_empty() {
        return Ok(());
    }
    
    // Add some default templates
    
    // Nginx template
    let nginx_id = Uuid::new_v4();
    let nginx_template = ContainerTemplate {
        id: nginx_id,
        name: "Nginx Web Server".to_string(),
        description: "A high-performance web server and reverse proxy".to_string(),
        category: "Web Server".to_string(),
        image: "nginx".to_string(),
        tag: "latest".to_string(),
        command: None,
        env: HashMap::new(),
        ports: vec![
            PortMapping {
                host_port: Some(8080),
                container_port: 80,
                protocol: "tcp".to_string(),
            }
        ],
        volumes: vec![
            VolumeMapping {
                host_path: "./nginx/html".to_string(),
                container_path: "/usr/share/nginx/html".to_string(),
                read_only: false,
            },
            VolumeMapping {
                host_path: "./nginx/conf".to_string(),
                container_path: "/etc/nginx/conf.d".to_string(),
                read_only: false,
            }
        ],
        network_mode: None,
        restart_policy: Some("unless-stopped".to_string()),
        resources: None,
        labels: {
            let mut labels = HashMap::new();
            labels.insert("com.rustainer.template".to_string(), "true".to_string());
            labels
        },
        version: "1.0".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: Some("system".to_string()),
    };
    templates.insert(nginx_id, nginx_template);
    
    // PostgreSQL template
    let postgres_id = Uuid::new_v4();
    let postgres_template = ContainerTemplate {
        id: postgres_id,
        name: "PostgreSQL Database".to_string(),
        description: "A powerful, open source object-relational database system".to_string(),
        category: "Database".to_string(),
        image: "postgres".to_string(),
        tag: "13".to_string(),
        command: None,
        env: {
            let mut env = HashMap::new();
            env.insert("POSTGRES_PASSWORD".to_string(), "postgres".to_string());
            env.insert("POSTGRES_USER".to_string(), "postgres".to_string());
            env.insert("POSTGRES_DB".to_string(), "postgres".to_string());
            env
        },
        ports: vec![
            PortMapping {
                host_port: Some(5432),
                container_port: 5432,
                protocol: "tcp".to_string(),
            }
        ],
        volumes: vec![
            VolumeMapping {
                host_path: "./postgres/data".to_string(),
                container_path: "/var/lib/postgresql/data".to_string(),
                read_only: false,
            }
        ],
        network_mode: None,
        restart_policy: Some("unless-stopped".to_string()),
        resources: Some(ResourceLimits {
            cpu: Some(1.0),
            memory: Some(1024 * 1024 * 1024), // 1GB
            memory_swap: None,
        }),
        labels: {
            let mut labels = HashMap::new();
            labels.insert("com.rustainer.template".to_string(), "true".to_string());
            labels
        },
        version: "1.0".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: Some("system".to_string()),
    };
    templates.insert(postgres_id, postgres_template);
    
    // Redis template
    let redis_id = Uuid::new_v4();
    let redis_template = ContainerTemplate {
        id: redis_id,
        name: "Redis Cache".to_string(),
        description: "An in-memory data structure store, used as a database, cache, and message broker".to_string(),
        category: "Cache".to_string(),
        image: "redis".to_string(),
        tag: "alpine".to_string(),
        command: None,
        env: HashMap::new(),
        ports: vec![
            PortMapping {
                host_port: Some(6379),
                container_port: 6379,
                protocol: "tcp".to_string(),
            }
        ],
        volumes: vec![
            VolumeMapping {
                host_path: "./redis/data".to_string(),
                container_path: "/data".to_string(),
                read_only: false,
            }
        ],
        network_mode: None,
        restart_policy: Some("unless-stopped".to_string()),
        resources: None,
        labels: {
            let mut labels = HashMap::new();
            labels.insert("com.rustainer.template".to_string(), "true".to_string());
            labels
        },
        version: "1.0".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: Some("system".to_string()),
    };
    templates.insert(redis_id, redis_template);
    
    Ok(())
}