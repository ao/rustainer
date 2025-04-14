// Authentication types
export interface User {
  id: string;
  username: string;
  role: 'admin' | 'operator' | 'viewer';
  email?: string;
  created_at: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface CreateUserRequest {
  username: string;
  password: string;
  role: 'admin' | 'operator' | 'viewer';
  email?: string;
}

export interface UpdateUserRequest {
  username?: string;
  password?: string;
  role?: 'admin' | 'operator' | 'viewer';
  email?: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

// Container types
// Container types
export interface Container {
  id: string;
  names: string[];
  image: string;
  status: string;
  state: string;
  created: string;
  ports: PortMapping[];
}

export interface PortMapping {
  container_port: number;
  host_port?: number;
  protocol: string;
}

export interface CreateContainerRequest {
  image: string;
  name: string;
  ports?: string[];
  env?: string[];
  volumes?: string[];
}

export interface ContainerStats {
  id: string;
  cpu_usage_percent: number;
  memory_usage_bytes: number;
  memory_usage_percent: number;
  network_input_bytes: number;
  network_output_bytes: number;
  block_input_bytes: number;
  block_output_bytes: number;
  process_count: number;
}

export interface ContainerLogs {
  id: string;
  logs: string[];
}

// Volume types
export interface Volume {
  name: string;
  driver: string;
  mountpoint: string;
  created_at?: string;
  labels?: Record<string, string>;
}

// Network types
export interface Network {
  id: string;
  name: string;
  driver: string;
  scope: string;
  created?: string;
  ipam?: NetworkIpam;
  internal: boolean;
  enable_ipv6: boolean;
  options: Record<string, string>;
  labels: Record<string, string>;
  containers: NetworkContainer[];
}

export interface NetworkIpam {
  driver: string;
  config: NetworkIpamConfig[];
}

export interface NetworkIpamConfig {
  subnet?: string;
  gateway?: string;
  ip_range?: string;
  aux_addresses?: Record<string, string>;
}

export interface NetworkContainer {
  id: string;
  name: string;
  ipv4_address?: string;
  ipv6_address?: string;
  mac_address?: string;
}

export interface CreateNetworkRequest {
  name: string;
  driver: string;
  internal?: boolean;
  enable_ipv6?: boolean;
  ipam?: NetworkIpamRequest;
  options?: Record<string, string>;
  labels?: Record<string, string>;
}

export interface NetworkIpamRequest {
  driver?: string;
  options?: Record<string, string>;
  config: NetworkIpamConfigRequest[];
}

export interface NetworkIpamConfigRequest {
  subnet?: string;
  ip_range?: string;
  gateway?: string;
  aux_addresses?: Record<string, string>;
}

export interface ConnectContainerRequest {
  container_id: string;
  ipv4_address?: string;
  ipv6_address?: string;
  links?: string[];
  aliases?: string[];
}

export interface DisconnectContainerRequest {
  container_id: string;
  force?: boolean;
}

export interface NetworkDiagnostics {
  id: string;
  name: string;
  driver: string;
  scope: string;
  status: NetworkStatus;
  metrics: NetworkMetrics;
  connectivity: ConnectivityCheck[];
}

export interface NetworkStatus {
  operational: boolean;
  message: string;
  container_count: number;
  created_at: string;
}

export interface NetworkMetrics {
  rx_bytes: number;
  tx_bytes: number;
  rx_packets: number;
  tx_packets: number;
  rx_errors: number;
  tx_errors: number;
  rx_dropped: number;
  tx_dropped: number;
}

export interface ConnectivityCheck {
  source_id: string;
  source_name: string;
  destination_id: string;
  destination_name: string;
  success: boolean;
  latency_ms?: number;
  error?: string;
  timestamp: string;
}

// Docker Compose types
export interface ComposeStack {
  id: string;
  name: string;
  file_path: string;
  status: 'up' | 'down' | 'partial' | 'error';
  created_at: string;
  updated_at: string;
  services: ComposeService[];
}

// Template types
export interface PortMapping {
  host_port?: number;
  container_port: number;
  protocol: string;
}

export interface VolumeMapping {
  host_path: string;
  container_path: string;
  read_only: boolean;
}

export interface ResourceLimits {
  cpu?: number;
  memory?: number;
  memory_swap?: number;
}

export interface ContainerTemplate {
  id: string;
  name: string;
  description: string;
  category: string;
  image: string;
  tag: string;
  command?: string;
  env: Record<string, string>;
  ports: PortMapping[];
  volumes: VolumeMapping[];
  network_mode?: string;
  restart_policy?: string;
  resources?: ResourceLimits;
  labels: Record<string, string>;
  version: string;
  created_at: string;
  updated_at: string;
  created_by?: string;
}

export interface CreateTemplateRequest {
  name: string;
  description: string;
  category: string;
  image: string;
  tag: string;
  command?: string;
  env: Record<string, string>;
  ports: PortMapping[];
  volumes: VolumeMapping[];
  network_mode?: string;
  restart_policy?: string;
  resources?: ResourceLimits;
  labels: Record<string, string>;
}

export interface UpdateTemplateRequest {
  name?: string;
  description?: string;
  category?: string;
  image?: string;
  tag?: string;
  command?: string;
  env?: Record<string, string>;
  ports?: PortMapping[];
  volumes?: VolumeMapping[];
  network_mode?: string;
  restart_policy?: string;
  resources?: ResourceLimits;
  labels?: Record<string, string>;
}

export interface DeployTemplateRequest {
  template_id: string;
  name: string;
  env_override?: Record<string, string>;
  port_override?: PortMapping[];
  volume_override?: VolumeMapping[];
}

export interface ComposeService {
  name: string;
  image: string;
  status: string;
  container_id?: string;
}

export interface CreateStackRequest {
  name: string;
  compose_content: string;
  start: boolean;
}

export interface UpdateStackRequest {
  compose_content: string;
  restart: boolean;
}

export interface StackLogs {
  [serviceName: string]: string[];
}

// Service types
export interface SSLConfig {
  enabled: boolean;
  cert_path?: string;
  key_path?: string;
  auto_generate: boolean;
}

export interface Service {
  id: string;
  name: string;
  domain: string;
  service_type: 'Container' | 'StaticSite' | 'CustomURL';
  target: string;
  port: number;
  ssl: SSLConfig;
  headers: Record<string, string>;
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateServiceRequest {
  name: string;
  domain: string;
  service_type: 'Container' | 'StaticSite' | 'CustomURL';
  target: string;
  port: number;
  ssl?: SSLConfig;
  headers?: Record<string, string>;
}

export interface UpdateServiceRequest {
  name?: string;
  domain?: string;
  service_type?: 'Container' | 'StaticSite' | 'CustomURL';
  target?: string;
  port?: number;
  ssl?: SSLConfig;
  headers?: Record<string, string>;
  enabled?: boolean;
}