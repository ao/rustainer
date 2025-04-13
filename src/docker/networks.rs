use bollard::network::ListNetworksOptions;
use bollard::Docker;
use serde::{Deserialize, Serialize};

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