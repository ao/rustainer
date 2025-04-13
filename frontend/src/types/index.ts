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
}

export interface NetworkIpam {
  driver: string;
  config: NetworkIpamConfig[];
}

export interface NetworkIpamConfig {
  subnet?: string;
  gateway?: string;
  ip_range?: string;
}