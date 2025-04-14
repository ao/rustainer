use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Docker network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    /// Network ID
    pub id: String,
    /// Network name
    pub name: String,
    /// Network driver (e.g., "bridge", "overlay", "macvlan")
    pub driver: String,
    /// Network scope (e.g., "local", "swarm", "global")
    pub scope: String,
    /// Whether the network is internal
    pub internal: bool,
    /// Whether IPv6 is enabled
    pub enable_ipv6: bool,
    /// IPAM configuration
    pub ipam: Option<IpamConfig>,
    /// Network options
    pub options: HashMap<String, String>,
    /// Network labels
    pub labels: HashMap<String, String>,
    /// Containers connected to this network
    pub containers: Vec<NetworkContainer>,
}

/// Represents IPAM (IP Address Management) configuration for a network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamConfig {
    /// IPAM driver (e.g., "default")
    pub driver: String,
    /// IPAM options
    pub options: Option<HashMap<String, String>>,
    /// IPAM configuration
    pub config: Vec<IpamPoolConfig>,
}

/// Represents an IPAM pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPoolConfig {
    /// Subnet (e.g., "172.17.0.0/16")
    pub subnet: Option<String>,
    /// IP range (e.g., "172.17.0.0/24")
    pub ip_range: Option<String>,
    /// Gateway (e.g., "172.17.0.1")
    pub gateway: Option<String>,
    /// Auxiliary addresses
    pub aux_addresses: Option<HashMap<String, String>>,
}

/// Represents a container connected to a network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkContainer {
    /// Container ID
    pub id: String,
    /// Container name
    pub name: String,
    /// Container IPv4 address
    pub ipv4_address: Option<String>,
    /// Container IPv6 address
    pub ipv6_address: Option<String>,
    /// Container MAC address
    pub mac_address: Option<String>,
}

/// Request to create a new network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNetworkRequest {
    /// Network name
    pub name: String,
    /// Network driver (e.g., "bridge", "overlay", "macvlan")
    pub driver: String,
    /// Whether the network is internal
    pub internal: Option<bool>,
    /// Whether IPv6 is enabled
    pub enable_ipv6: Option<bool>,
    /// IPAM configuration
    pub ipam: Option<IpamConfigRequest>,
    /// Network options
    pub options: Option<HashMap<String, String>>,
    /// Network labels
    pub labels: Option<HashMap<String, String>>,
}

/// IPAM configuration for network creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamConfigRequest {
    /// IPAM driver (e.g., "default")
    pub driver: Option<String>,
    /// IPAM options
    pub options: Option<HashMap<String, String>>,
    /// IPAM configuration
    pub config: Vec<IpamPoolConfigRequest>,
}

/// IPAM pool configuration for network creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamPoolConfigRequest {
    /// Subnet (e.g., "172.17.0.0/16")
    pub subnet: Option<String>,
    /// IP range (e.g., "172.17.0.0/24")
    pub ip_range: Option<String>,
    /// Gateway (e.g., "172.17.0.1")
    pub gateway: Option<String>,
    /// Auxiliary addresses
    pub aux_addresses: Option<HashMap<String, String>>,
}

/// Request to connect a container to a network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectContainerRequest {
    /// Container ID
    pub container_id: String,
    /// IPv4 address to assign to the container
    pub ipv4_address: Option<String>,
    /// IPv6 address to assign to the container
    pub ipv6_address: Option<String>,
    /// Links to other containers
    pub links: Option<Vec<String>>,
    /// Aliases for the container in this network
    pub aliases: Option<Vec<String>>,
}

/// Request to disconnect a container from a network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisconnectContainerRequest {
    /// Container ID
    pub container_id: String,
    /// Force disconnection
    pub force: Option<bool>,
}

/// Network diagnostics information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiagnostics {
    /// Network ID
    pub id: String,
    /// Network name
    pub name: String,
    /// Network driver
    pub driver: String,
    /// Network scope
    pub scope: String,
    /// Network status
    pub status: NetworkStatus,
    /// Network metrics
    pub metrics: NetworkMetrics,
    /// Network connectivity checks
    pub connectivity: Vec<ConnectivityCheck>,
}

/// Network status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// Whether the network is operational
    pub operational: bool,
    /// Status message
    pub message: String,
    /// Number of containers connected to the network
    pub container_count: usize,
    /// Creation time
    pub created_at: String,
}

/// Network metrics information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Total bytes received
    pub rx_bytes: u64,
    /// Total bytes transmitted
    pub tx_bytes: u64,
    /// Total packets received
    pub rx_packets: u64,
    /// Total packets transmitted
    pub tx_packets: u64,
    /// Total errors received
    pub rx_errors: u64,
    /// Total errors transmitted
    pub tx_errors: u64,
    /// Total dropped packets received
    pub rx_dropped: u64,
    /// Total dropped packets transmitted
    pub tx_dropped: u64,
}

/// Network connectivity check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityCheck {
    /// Source container ID
    pub source_id: String,
    /// Source container name
    pub source_name: String,
    /// Destination container ID
    pub destination_id: String,
    /// Destination container name
    pub destination_name: String,
    /// Whether the connectivity check was successful
    pub success: bool,
    /// Latency in milliseconds
    pub latency_ms: Option<f64>,
    /// Error message if the check failed
    pub error: Option<String>,
    /// Timestamp of the check
    pub timestamp: String,
}