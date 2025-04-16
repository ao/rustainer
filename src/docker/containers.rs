use std::collections::HashMap;
use std::sync::Arc;

use bollard::container::{
    Config, CreateContainerOptions, ListContainersOptions, LogOutput, LogsOptions, RemoveContainerOptions,
    RestartContainerOptions, StartContainerOptions, StatsOptions, StopContainerOptions,
};
use bollard::models::{ContainerStateStatusEnum, HostConfig, PortBinding};
use bollard::Docker;
use chrono::{DateTime, TimeZone, Utc};
use futures::stream::StreamExt;

use crate::models::{Container, ContainerLogs, ContainerStats, CreateContainerRequest};
use crate::models::container::PortMapping;

/// List all containers
pub async fn list_containers(docker: &Docker) -> anyhow::Result<Vec<Container>> {
    let options = Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    });

    let containers = docker.list_containers(options).await?;

    let result = containers
        .into_iter()
        .map(|c| {
            // Extract container names (remove leading slash)
            let names = c
                .names
                .unwrap_or_default()
                .into_iter()
                .map(|name| name.trim_start_matches('/').to_string())
                .collect();

            // Extract port mappings
            let ports = c
                .ports
                .unwrap_or_default()
                .into_iter()
                .filter_map(|port| {
                    if let Some(protocol) = port.typ {
                        Some(PortMapping {
                            container_port: port.private_port as u16,
                            host_port: port.public_port.map(|p| p as u16),
                            protocol: protocol.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect();

            // Convert created timestamp to DateTime
            let created = Utc.timestamp_opt(c.created.unwrap_or(0), 0).unwrap();

            Container {
                id: c.id.unwrap_or_default(),
                names,
                image: c.image.unwrap_or_default(),
                status: c.status.unwrap_or_default(),
                state: c.state.map_or_else(String::new, |s| s.to_string()),
                created,
                ports,
            }
        })
        .collect();

    Ok(result)
}

/// Start a container
pub async fn start_container(docker: &Docker, id: &str) -> anyhow::Result<()> {
    docker
        .start_container(id, None::<StartContainerOptions<String>>)
        .await?;
    Ok(())
}

/// Stop a container
pub async fn stop_container(docker: &Docker, id: &str) -> anyhow::Result<()> {
    docker
        .stop_container(id, None::<StopContainerOptions>)
        .await?;
    Ok(())
}

/// Restart a container
pub async fn restart_container(docker: &Docker, id: &str) -> anyhow::Result<()> {
    docker
        .restart_container(id, None::<RestartContainerOptions>)
        .await?;
    Ok(())
}

/// Get container logs
pub async fn get_container_logs(docker: &Docker, id: &str) -> anyhow::Result<ContainerLogs> {
    let options = Some(LogsOptions::<String> {
        stdout: true,
        stderr: true,
        tail: "100".to_string(),
        ..Default::default()
    });

    let mut logs_stream = docker.logs(id, options);
    let mut log_lines = Vec::new();

    while let Some(log_result) = logs_stream.next().await {
        match log_result {
            Ok(log_output) => {
                let line = match log_output {
                    LogOutput::StdOut { message } => String::from_utf8_lossy(&message).to_string(),
                    LogOutput::StdErr { message } => String::from_utf8_lossy(&message).to_string(),
                    _ => continue,
                };
                log_lines.push(line);
            }
            Err(e) => {
                tracing::error!("Error getting logs: {}", e);
            }
        }
    }

    Ok(ContainerLogs {
        id: id.to_string(),
        logs: log_lines,
    })
}

/// Create a new container
pub async fn create_container(
    docker: &Docker,
    request: CreateContainerRequest,
) -> anyhow::Result<Container> {
    // Parse port bindings
    let mut port_bindings = HashMap::new();
    if let Some(ports) = &request.ports {
        for port_mapping in ports {
            let parts: Vec<&str> = port_mapping.split(':').collect();
            if parts.len() == 2 {
                let host_port = parts[0].parse::<u16>()?;
                let container_port = parts[1].parse::<u16>()?;
                let key = format!("{}/tcp", container_port);
                
                port_bindings.insert(
                    key,
                    Some(vec![PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some(host_port.to_string()),
                    }]),
                );
            }
        }
    }

    // Parse volume bindings
    let mut binds = Vec::new();
    if let Some(volumes) = &request.volumes {
        for volume_mapping in volumes {
            binds.push(volume_mapping.clone());
        }
    }
    // Create container
    let options = CreateContainerOptions {
        name: request.name,
        platform: None,
    };

    let config = Config {
        image: Some(request.image),
        env: request.env,
        host_config: Some(HostConfig {
            port_bindings: Some(port_bindings),
            binds: Some(binds),
            ..Default::default()
        }),
        ..Default::default()
    };

    let response = docker.create_container(Some(options), config).await?;

    // Start the container
    docker
        .start_container(&response.id, None::<StartContainerOptions<String>>)
        .await?;

    // Get container details
    let containers = list_containers(docker).await?;
    let container = containers
        .into_iter()
        .find(|c| c.id == response.id)
        .ok_or_else(|| anyhow::anyhow!("Container not found after creation"))?;

    Ok(container)
}

/// Get container stats
pub async fn get_container_stats(docker: &Docker, id: &str) -> anyhow::Result<ContainerStats> {
    let options = Some(StatsOptions {
        stream: false,
        ..Default::default()
    });

    let mut stats_stream = docker.stats(id, options);
    let stats = match stats_stream.next().await {
        Some(Ok(stats)) => stats,
        Some(Err(e)) => return Err(anyhow::anyhow!("Failed to get stats: {}", e)),
        None => return Err(anyhow::anyhow!("No stats available")),
    };

    // Calculate CPU usage percentage
    let cpu_delta = stats.cpu_stats.as_ref()
        .and_then(|cpu| cpu.cpu_usage.as_ref())
        .and_then(|usage| usage.total_usage)
        .unwrap_or(0) as f64 -
        stats.precpu_stats.as_ref()
        .and_then(|cpu| cpu.cpu_usage.as_ref())
        .and_then(|usage| usage.total_usage)
        .unwrap_or(0) as f64;
    
    let system_delta = stats.cpu_stats.as_ref()
        .and_then(|cpu| cpu.system_cpu_usage)
        .unwrap_or(0) as f64 -
        stats.precpu_stats.as_ref()
        .and_then(|cpu| cpu.system_cpu_usage)
        .unwrap_or(0) as f64;
    
    let cpu_count = stats.cpu_stats.as_ref()
        .and_then(|cpu| cpu.online_cpus)
        .unwrap_or(1) as f64;

    let cpu_usage_percent = if system_delta > 0.0 && cpu_delta > 0.0 {
        (cpu_delta / system_delta) * cpu_count * 100.0
    } else {
        0.0
    };

    // Calculate memory usage percentage
    let memory_usage_bytes = stats.memory_stats.as_ref()
        .and_then(|mem| mem.usage)
        .unwrap_or(0);
    
    let memory_limit = stats.memory_stats.as_ref()
        .and_then(|mem| mem.limit)
        .unwrap_or(1);
    
    let memory_usage_percent = (memory_usage_bytes as f64 / memory_limit as f64) * 100.0;

    // Calculate network I/O
    // Calculate network I/O
    let mut network_input_bytes = 0;
    let mut network_output_bytes = 0;
    // Skip network stats for now as the structure is complex
    // We'll implement this properly in a future update

    // Calculate block I/O
    let mut block_input_bytes = 0;
    let mut block_output_bytes = 0;
    if let Some(blkio) = stats.blkio_stats.as_ref() {
        if let Some(io_stats) = &blkio.io_service_bytes_recursive {
            for io_stat in io_stats {
                if let Some(op) = &io_stat.op {
                    if let Some(value) = io_stat.value {
                        match op.as_str() {
                            "Read" => block_input_bytes += value,
                            "Write" => block_output_bytes += value,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // Get process count
    let process_count = stats.pids_stats.as_ref()
        .and_then(|pids| pids.current)
        .unwrap_or(0);

    Ok(ContainerStats {
        id: id.to_string(),
        cpu_usage_percent,
        memory_usage_bytes,
        memory_usage_percent,
        network_input_bytes,
        network_output_bytes,
        block_input_bytes,
        block_output_bytes,
        process_count,
    })
}