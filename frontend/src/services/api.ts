import axios from 'axios';
import {
  Container,
  CreateContainerRequest,
  Volume,
  Network,
  CreateNetworkRequest,
  ConnectContainerRequest,
  DisconnectContainerRequest,
  NetworkDiagnostics,
  ComposeStack,
  CreateStackRequest,
  UpdateStackRequest,
  StackLogs,
  ContainerTemplate,
  CreateTemplateRequest,
  UpdateTemplateRequest,
  DeployTemplateRequest,
  LoginRequest,
  AuthResponse,
  User,
  CreateUserRequest,
  UpdateUserRequest,
  Service,
  CreateServiceRequest,
  UpdateServiceRequest,
} from '../types';

// Create axios instance
const axiosInstance = axios.create({
  baseURL: process.env.REACT_APP_API_URL || '',
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add token to requests
axiosInstance.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Container API
export const containerApi = {
  // List all containers
  listContainers: async (): Promise<Container[]> => {
    const response = await axiosInstance.get<Container[]>('/api/containers');
    return response.data;
  },

  // Get container details
  getContainer: async (id: string): Promise<Container> => {
    const response = await axiosInstance.get<Container>(`/api/containers/${id}`);
    return response.data;
  },

  // Start a container
  startContainer: async (id: string): Promise<void> => {
    await axiosInstance.post(`/api/containers/${id}/start`);
  },

  // Stop a container
  stopContainer: async (id: string): Promise<void> => {
    await axiosInstance.post(`/api/containers/${id}/stop`);
  },

  // Restart a container
  restartContainer: async (id: string): Promise<void> => {
    await axiosInstance.post(`/api/containers/${id}/restart`);
  },

  // Get container logs
  getContainerLogs: async (id: string): Promise<string[]> => {
    const response = await axiosInstance.get<string[]>(`/api/containers/${id}/logs`);
    return response.data;
  },

  // Create a container
  createContainer: async (container: CreateContainerRequest): Promise<Container> => {
    const response = await axiosInstance.post<Container>('/api/containers/create', container);
    return response.data;
  },

  // Get container stats
  getContainerStats: async (id: string): Promise<any> => {
    const response = await axiosInstance.get(`/api/containers/${id}/stats`);
    return response.data;
  },
};

// Volume API
export const volumeApi = {
  // List all volumes
  listVolumes: async (): Promise<Volume[]> => {
    const response = await axiosInstance.get<Volume[]>('/api/volumes');
    return response.data;
  },
};

// Network API
export const networkApi = {
  // List all networks
  listNetworks: async (): Promise<Network[]> => {
    const response = await axiosInstance.get<Network[]>('/api/networks');
    return response.data;
  },

  // Get network details
  getNetwork: async (id: string): Promise<Network> => {
    const response = await axiosInstance.get<Network>(`/api/networks/${id}`);
    return response.data;
  },

  // Create a network
  createNetwork: async (network: CreateNetworkRequest): Promise<Network> => {
    const response = await axiosInstance.post<Network>('/api/networks', network);
    return response.data;
  },

  // Delete a network
  deleteNetwork: async (id: string): Promise<void> => {
    await axiosInstance.delete(`/api/networks/${id}`);
  },

  // Connect a container to a network
  connectContainer: async (id: string, request: ConnectContainerRequest): Promise<void> => {
    await axiosInstance.post(`/api/networks/${id}/connect`, request);
  },

  // Disconnect a container from a network
  disconnectContainer: async (id: string, request: DisconnectContainerRequest): Promise<void> => {
    await axiosInstance.post(`/api/networks/${id}/disconnect`, request);
  },

  // Prune unused networks
  pruneNetworks: async (): Promise<void> => {
    await axiosInstance.post('/api/networks/prune');
  },

  // Get network diagnostics
  getNetworkDiagnostics: async (id: string): Promise<NetworkDiagnostics> => {
    const response = await axiosInstance.get<NetworkDiagnostics>(`/api/networks/${id}/diagnostics`);
    return response.data;
  },
};

// Docker Compose API
export const composeApi = {
  // List all stacks
  listStacks: async (): Promise<ComposeStack[]> => {
    const response = await axiosInstance.get<ComposeStack[]>('/api/compose');
    return response.data;
  },

  // Get stack details
  getStack: async (id: string): Promise<ComposeStack> => {
    const response = await axiosInstance.get<ComposeStack>(`/api/compose/${id}`);
    return response.data;
  },

  // Create a stack
  createStack: async (stack: CreateStackRequest): Promise<ComposeStack> => {
    const response = await axiosInstance.post<ComposeStack>('/api/compose', stack);
    return response.data;
  },

  // Update a stack
  updateStack: async (id: string, stack: UpdateStackRequest): Promise<ComposeStack> => {
    const response = await axiosInstance.post<ComposeStack>(`/api/compose/${id}`, stack);
    return response.data;
  },

  // Delete a stack
  deleteStack: async (id: string): Promise<void> => {
    await axiosInstance.delete(`/api/compose/${id}`);
  },

  // Start a stack
  startStack: async (id: string): Promise<void> => {
    await axiosInstance.post(`/api/compose/${id}/start`);
  },

  // Stop a stack
  stopStack: async (id: string): Promise<void> => {
    await axiosInstance.post(`/api/compose/${id}/stop`);
  },

  // Restart a stack
  restartStack: async (id: string): Promise<void> => {
    await axiosInstance.post(`/api/compose/${id}/restart`);
  },

  // Get stack logs
  getStackLogs: async (id: string): Promise<StackLogs> => {
    const response = await axiosInstance.get<StackLogs>(`/api/compose/${id}/logs`);
    return response.data;
  },
};

// Template API
export const templateApi = {
  // List all templates
  listTemplates: async (): Promise<ContainerTemplate[]> => {
    const response = await axiosInstance.get<ContainerTemplate[]>('/api/templates');
    return response.data;
  },

  // Get template details
  getTemplate: async (id: string): Promise<ContainerTemplate> => {
    const response = await axiosInstance.get<ContainerTemplate>(`/api/templates/${id}`);
    return response.data;
  },

  // Create a template
  createTemplate: async (template: CreateTemplateRequest): Promise<ContainerTemplate> => {
    const response = await axiosInstance.post<ContainerTemplate>('/api/templates', template);
    return response.data;
  },

  // Update a template
  updateTemplate: async (id: string, template: UpdateTemplateRequest): Promise<ContainerTemplate> => {
    const response = await axiosInstance.post<ContainerTemplate>(`/api/templates/${id}`, template);
    return response.data;
  },

  // Delete a template
  deleteTemplate: async (id: string): Promise<void> => {
    await axiosInstance.delete(`/api/templates/${id}`);
  },

  // Deploy a template
  deployTemplate: async (request: DeployTemplateRequest): Promise<Container> => {
    const response = await axiosInstance.post<Container>('/api/templates/deploy', request);
    return response.data;
  },
};

// Auth API
export const authApi = {
  // Login
  login: async (credentials: LoginRequest): Promise<AuthResponse> => {
    const response = await axiosInstance.post<AuthResponse>('/api/auth/login', credentials);
    // Store the token in localStorage
    localStorage.setItem('token', response.data.token);
    return response.data;
  },

  // Get current user
  getCurrentUser: async (): Promise<User> => {
    const response = await axiosInstance.get<User>('/api/auth/me');
    return response.data;
  },

  // Get all users
  getUsers: async (): Promise<User[]> => {
    const response = await axiosInstance.get<User[]>('/api/auth/users');
    return response.data;
  },

  // Get a user by ID
  getUser: async (id: string): Promise<User> => {
    const response = await axiosInstance.get<User>(`/api/auth/users/${id}`);
    return response.data;
  },

  // Create a new user
  createUser: async (user: CreateUserRequest): Promise<User> => {
    const response = await axiosInstance.post<User>('/api/auth/users', user);
    return response.data;
  },

  // Update a user
  updateUser: async (id: string, user: UpdateUserRequest): Promise<User> => {
    const response = await axiosInstance.post<User>(`/api/auth/users/${id}`, user);
    return response.data;
  },

  // Delete a user
  deleteUser: async (id: string): Promise<void> => {
    await axiosInstance.delete(`/api/auth/users/${id}`);
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

// Service API
export const serviceApi = {
  // List all services
  listServices: async (): Promise<Service[]> => {
    const response = await axiosInstance.get<Service[]>('/api/services');
    return response.data;
  },

  // Get a service by ID
  getService: async (id: string): Promise<Service> => {
    const response = await axiosInstance.get<Service>(`/api/services/${id}`);
    return response.data;
  },

  // Create a new service
  createService: async (service: CreateServiceRequest): Promise<Service> => {
    const response = await axiosInstance.post<Service>('/api/services', service);
    return response.data;
  },

  // Update a service
  updateService: async (id: string, service: UpdateServiceRequest): Promise<Service> => {
    const response = await axiosInstance.post<Service>(`/api/services/${id}`, service);
    return response.data;
  },

  // Delete a service
  deleteService: async (id: string): Promise<void> => {
    await axiosInstance.delete(`/api/services/${id}`);
  },

  // Enable a service
  enableService: async (id: string): Promise<Service> => {
    const response = await axiosInstance.post<Service>(`/api/services/${id}/enable`, {});
    return response.data;
  },

  // Disable a service
  disableService: async (id: string): Promise<Service> => {
    const response = await axiosInstance.post<Service>(`/api/services/${id}/disable`, {});
    return response.data;
  },
};

// Export all APIs
export const api = {
  auth: authApi,
  containers: containerApi,
  volumes: volumeApi,
  networks: networkApi,
  compose: composeApi,
  templates: templateApi,
  services: serviceApi,
};

export default api;