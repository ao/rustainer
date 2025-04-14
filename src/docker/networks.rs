use bollard::network::ListNetworksOptions;
use bollard::Docker;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Docker network
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    /// Network ID
    pub id: String,
    /// Network name
    pub name: String,
    /// Network driver
    pub driver: String,
    /// Network scope
    pub scope: String,
    /// Network creation time
    pub created: Option<String>,
    /// Network IPAM configuration
    pub ipam: Option<NetworkIpam>,
}

/// Represents a Docker network IPAM configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkIpam {
    /// IPAM driver
    pub driver: String,
    /// IPAM configuration
    pub config: Vec<NetworkIpamConfig>,
}

/// Represents a Docker network IPAM configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkIpamConfig {
    /// Subnet
    pub subnet: Option<String>,
    /// Gateway
    pub gateway: Option<String>,
    /// IP range
    pub ip_range: Option<String>,
}

/// List all networks
pub async fn list_networks(docker: &Docker) -> anyhow::Result<Vec<Network>> {
    let options = Some(ListNetworksOptions::<String> {
        ..Default::default()
    });

    let networks = docker.list_networks(options).await?;

    let result = networks
        .into_iter()
        .map(|n| {
            // Extract IPAM configuration
            let ipam = n.ipam.map(|ipam| {
                let config = ipam
                    .config
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| NetworkIpamConfig {
                        subnet: c.subnet,
                        gateway: c.gateway,
                        ip_range: c.ip_range,
                    })
                    .collect();

                NetworkIpam {
                    driver: ipam.driver.unwrap_or_default(),
                    config,
                }
            });

            Network {
                id: n.id.unwrap_or_default(),
                name: n.name.unwrap_or_default(),
                driver: n.driver.unwrap_or_default(),
                scope: n.scope.unwrap_or_default(),
                created: n.created,
                ipam,
            }
        })
        .collect();

    Ok(result)
}

/// Create a new network
pub async fn create_network(
    docker: &Docker,
    request: crate::models::CreateNetworkRequest,
) -> anyhow::Result<String> {
    // Create the network with minimal options
    let create_opts = bollard::network::CreateNetworkOptions {
        name: request.name,
        driver: request.driver,
        ..Default::default()
    };
    
    let response = docker.create_network(create_opts).await?;
    Ok(response.id.unwrap_or_default())
}

/// Get network details
pub async fn get_network(docker: &Docker, id: &str) -> anyhow::Result<crate::models::Network> {
    // Get network details
    let network = docker.inspect_network(id, None::<bollard::network::InspectNetworkOptions<String>>).await?;
    
    // Extract containers connected to the network
    let containers = network.containers.map(|containers| {
        containers.into_iter().map(|(id, container)| {
            crate::models::NetworkContainer {
                id: id.clone(),
                name: container.name.unwrap_or_default(),
                ipv4_address: container.ipv4_address,
                ipv6_address: container.ipv6_address,
                mac_address: container.mac_address,
            }
        }).collect()
    }).unwrap_or_default();
    
    // Extract IPAM configuration
    let ipam = network.ipam.map(|ipam| {
        crate::models::IpamConfig {
            driver: ipam.driver.unwrap_or_default(),
            options: ipam.options,
            config: ipam.config.unwrap_or_default().into_iter().map(|c| {
                crate::models::IpamPoolConfig {
                    subnet: c.subnet,
                    ip_range: c.ip_range,
                    gateway: c.gateway,
                    aux_addresses: c.auxiliary_addresses,
                }
            }).collect(),
        }
    });
    
    // Create network model
    let result = crate::models::Network {
        id: network.id.unwrap_or_default(),
        name: network.name.unwrap_or_default(),
        driver: network.driver.unwrap_or_default(),
        scope: network.scope.unwrap_or_default(),
        internal: network.internal.unwrap_or_default(),
        enable_ipv6: network.enable_ipv6.unwrap_or_default(),
        ipam,
        options: network.options.unwrap_or_default(),
        labels: network.labels.unwrap_or_default(),
        containers,
    };
    
    Ok(result)
}

/// Delete a network
pub async fn delete_network(docker: &Docker, id: &str) -> anyhow::Result<()> {
    docker.remove_network(id).await?;
    Ok(())
}

/// Connect a container to a network
pub async fn connect_container(
    docker: &Docker,
    network_id: &str,
    request: crate::models::ConnectContainerRequest,
) -> anyhow::Result<()> {
    // Connect container to network with minimal options
    let options = bollard::network::ConnectNetworkOptions {
        container: request.container_id,
        endpoint_config: bollard::models::EndpointSettings::default(),
    };
    
    docker.connect_network(network_id, options).await?;
    Ok(())
}

/// Disconnect a container from a network
pub async fn disconnect_container(
    docker: &Docker,
    network_id: &str,
    request: crate::models::DisconnectContainerRequest,
) -> anyhow::Result<()> {
    // Disconnect container from network
    let options = bollard::network::DisconnectNetworkOptions {
        container: request.container_id,
        force: request.force.unwrap_or(false),
    };
    
    docker.disconnect_network(network_id, options).await?;
    Ok(())
}

/// Prune unused networks
pub async fn prune_networks(docker: &Docker) -> anyhow::Result<Vec<String>> {
    let response = docker.prune_networks(None::<bollard::network::PruneNetworksOptions<String>>).await?;
    Ok(response.networks_deleted.unwrap_or_default())
}

/// Get network diagnostics
pub async fn get_network_diagnostics(
    docker: std::sync::Arc<Docker>,
    id: &str,
) -> anyhow::Result<crate::models::NetworkDiagnostics> {
    // Get network details
    let network = get_network(&docker, id).await?;
    
    // Create network status
    let status = crate::models::NetworkStatus {
        operational: true, // Assume operational if we can get details
        message: "Network is operational".to_string(),
        container_count: network.containers.len(),
        created_at: network.id.clone(), // Use ID as placeholder since we don't have creation time
    };
    
    // Create network metrics (placeholder values)
    let metrics = crate::models::NetworkMetrics {
        rx_bytes: 0,
        tx_bytes: 0,
        rx_packets: 0,
        tx_packets: 0,
        rx_errors: 0,
        tx_errors: 0,
        rx_dropped: 0,
        tx_dropped: 0,
    };
    
    // Create connectivity checks (placeholder)
    let connectivity = Vec::new();
    
    // Create network diagnostics
    let diagnostics = crate::models::NetworkDiagnostics {
        id: network.id,
        name: network.name,
        driver: network.driver,
        scope: network.scope,
        status,
        metrics,
        connectivity,
    };
    
    Ok(diagnostics)
}