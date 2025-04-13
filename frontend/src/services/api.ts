import axios from 'axios';
import {
  Container,
  ContainerLogs,
  ContainerStats,
  CreateContainerRequest,
  Network,
  Volume,
} from '../types';

// Create axios instance with base URL
const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

// Container API
export const containerApi = {
  // Get all containers
  getContainers: async (): Promise<Container[]> => {
    const response = await api.get<Container[]>('/containers');
    return response.data;
  },

  // Start a container
  startContainer: async (id: string): Promise<void> => {
    await api.post(`/containers/${id}/start`);
  },

  // Stop a container
  stopContainer: async (id: string): Promise<void> => {
    await api.post(`/containers/${id}/stop`);
  },

  // Restart a container
  restartContainer: async (id: string): Promise<void> => {
    await api.post(`/containers/${id}/restart`);
  },

  // Get container logs
  getContainerLogs: async (id: string): Promise<ContainerLogs> => {
    const response = await api.get<ContainerLogs>(`/containers/${id}/logs`);
    return response.data;
  },

  // Create a container
  createContainer: async (request: CreateContainerRequest): Promise<Container> => {
    const response = await api.post<Container>('/containers/create', request);
    return response.data;
  },

  // Get container stats
  getContainerStats: async (id: string): Promise<ContainerStats> => {
    const response = await api.get<ContainerStats>(`/containers/${id}/stats`);
    return response.data;
  },
};

// Volume API
export const volumeApi = {
  // Get all volumes
  getVolumes: async (): Promise<Volume[]> => {
    const response = await api.get<Volume[]>('/volumes');
    return response.data;
  },
};

// Network API
export const networkApi = {
  // Get all networks
  getNetworks: async (): Promise<Network[]> => {
    const response = await api.get<Network[]>('/networks');
    return response.data;
  },
};

export default {
  containerApi,
  volumeApi,
  networkApi,
};