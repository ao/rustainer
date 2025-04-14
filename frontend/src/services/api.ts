import axios from 'axios';
import {
  AuthResponse,
  Container,
  ContainerLogs,
  ContainerStats,
  CreateContainerRequest,
  LoginRequest,
  Network,
  User,
  Volume,
  ComposeStack,
  CreateStackRequest,
  UpdateStackRequest,
  StackLogs,
  CreateNetworkRequest,
  ConnectContainerRequest,
  DisconnectContainerRequest,
  NetworkDiagnostics,
  ContainerTemplate,
  CreateTemplateRequest,
  UpdateTemplateRequest,
  DeployTemplateRequest,
} from '../types';

// Create axios instance with base URL
const api = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add a request interceptor to add the token to requests
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

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
  
  // Get a network by ID
  getNetwork: async (id: string): Promise<Network> => {
    const response = await api.get<Network>(`/networks/${id}`);
    return response.data;
  },
  
  // Create a new network
  createNetwork: async (request: CreateNetworkRequest): Promise<string> => {
    const response = await api.post<string>('/networks', request);
    return response.data;
  },
  
  // Delete a network
  deleteNetwork: async (id: string): Promise<void> => {
    await api.delete(`/networks/${id}`);
  },
  
  // Connect a container to a network
  connectContainer: async (networkId: string, request: ConnectContainerRequest): Promise<void> => {
    await api.post(`/networks/${networkId}/connect`, request);
  },
  
  // Disconnect a container from a network
  disconnectContainer: async (networkId: string, request: DisconnectContainerRequest): Promise<void> => {
    await api.post(`/networks/${networkId}/disconnect`, request);
  },
  
  // Prune unused networks
  pruneNetworks: async (): Promise<string[]> => {
    const response = await api.post<string[]>('/networks/prune');
    return response.data;
  },
  
  // Get network diagnostics
  getNetworkDiagnostics: async (id: string): Promise<NetworkDiagnostics> => {
    const response = await api.get<NetworkDiagnostics>(`/networks/${id}/diagnostics`);
    return response.data;
  },
};

// Docker Compose API
export const composeApi = {
  // Get all stacks
  getStacks: async (): Promise<ComposeStack[]> => {
    const response = await api.get<ComposeStack[]>('/compose');
    return response.data;
  },

  // Get a stack by ID
  getStack: async (id: string): Promise<ComposeStack> => {
    const response = await api.get<ComposeStack>(`/compose/${id}`);
    return response.data;
  },

  // Create a new stack
  createStack: async (request: CreateStackRequest): Promise<ComposeStack> => {
    const response = await api.post<ComposeStack>('/compose', request);
    return response.data;
  },

  // Update a stack
  updateStack: async (id: string, request: UpdateStackRequest): Promise<ComposeStack> => {
    const response = await api.post<ComposeStack>(`/compose/${id}`, request);
    return response.data;
  },

  // Delete a stack
  deleteStack: async (id: string): Promise<void> => {
    await api.delete(`/compose/${id}`);
  },

  // Start a stack
  startStack: async (id: string): Promise<ComposeStack> => {
    const response = await api.post<ComposeStack>(`/compose/${id}/start`);
    return response.data;
  },

  // Stop a stack
  stopStack: async (id: string): Promise<ComposeStack> => {
    const response = await api.post<ComposeStack>(`/compose/${id}/stop`);
    return response.data;
  },

  // Restart a stack
  restartStack: async (id: string): Promise<ComposeStack> => {
    const response = await api.post<ComposeStack>(`/compose/${id}/restart`);
    return response.data;
  },

  // Get stack logs
  getStackLogs: async (id: string): Promise<StackLogs> => {
    const response = await api.get<StackLogs>(`/compose/${id}/logs`);
    return response.data;
  },
};

// Template API
export const templateApi = {
  // Get all templates
  getTemplates: async (): Promise<ContainerTemplate[]> => {
    const response = await api.get<ContainerTemplate[]>('/templates');
    return response.data;
  },

  // Get a template by ID
  getTemplate: async (id: string): Promise<ContainerTemplate> => {
    const response = await api.get<ContainerTemplate>(`/templates/${id}`);
    return response.data;
  },

  // Create a new template
  createTemplate: async (request: CreateTemplateRequest): Promise<ContainerTemplate> => {
    const response = await api.post<ContainerTemplate>('/templates', request);
    return response.data;
  },

  // Update a template
  updateTemplate: async (id: string, request: UpdateTemplateRequest): Promise<ContainerTemplate> => {
    const response = await api.post<ContainerTemplate>(`/templates/${id}`, request);
    return response.data;
  },

  // Delete a template
  deleteTemplate: async (id: string): Promise<void> => {
    await api.delete(`/templates/${id}`);
  },

  // Deploy a container from a template
  deployTemplate: async (request: DeployTemplateRequest): Promise<string> => {
    const response = await api.post<string>('/templates/deploy', request);
    return response.data;
  },
};

// Auth API
export const authApi = {
  // Login
  login: async (credentials: LoginRequest): Promise<AuthResponse> => {
    const response = await api.post<AuthResponse>('/auth/login', credentials);
    // Store the token in localStorage
    localStorage.setItem('token', response.data.token);
    return response.data;
  },

  // Get current user
  getCurrentUser: async (): Promise<User> => {
    const response = await api.get<User>('/auth/me');
    return response.data;
  },

  // Logout
  logout: (): void => {
    localStorage.removeItem('token');
  },

  // Check if user is authenticated
  isAuthenticated: (): boolean => {
    return localStorage.getItem('token') !== null;
  },
};

export default {
  authApi,
  containerApi,
  volumeApi,
  networkApi,
  composeApi,
};