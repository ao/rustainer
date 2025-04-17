# Rustainer Roadmap and Feature Plan

This document outlines the current status, planned features, and implementation roadmap for Rustainer, a lightweight container management UI built in Rust, inspired by Portainer and Dockge.

## Current Status

Rustainer currently implements these core features:
- Basic server setup with authentication
- Docker Compose stack management (partial implementation)
- Service routing system (domain-based routing to containers)
- Proxy server on port 80 for routing traffic
- User authentication with JWT
- Basic dark/light theme support

The following features are partially implemented or missing:
- Container management UI is incomplete
- Volume management is missing
- Network management is missing
- User management (RBAC) is incomplete
- Templates are incomplete or missing
- Many UI pages are using placeholder HTML

## Architecture Overview

Rustainer is built with:
- **Backend**: Rust with Axum web framework
- **Frontend**: Server-side rendered HTML with Askama templates
- **Styling**: Custom CSS with dark/light theme support
- **Docker API**: Bollard Rust client
- **Database**: SQLite with SQLx
- **Authentication**: JWT tokens

## Feature Roadmap

The development roadmap is organized into phases, with each phase building on the previous one.

### Phase 1: Core Container Management (In Progress)
- Basic container operations (CRUD, start/stop) - Partially Implemented
- Container logs and stats - Planned
- Volume and network basics - Planned
- Authentication system - Partially Implemented

### Phase 2: Docker Compose Integration (In Progress)
- Compose file parsing and validation - Partially Implemented
- Stack deployment and management - Partially Implemented
- Visual compose editor - Partially Implemented
- Stack templates - Planned

### Phase 2.5: Application & Service Routing (In Progress)
- Application creation (container with service/ingress) - Partially Implemented
- Domain-based routing to applications - Implemented
- Automatic service discovery - Planned
- Port 80 listener for routing traffic to containers based on domain - Implemented
- Management UI remains on port 3000 - Implemented
- DNS integration for domain resolution - Planned
- Traffic metrics and monitoring - Planned

### Phase 3: Advanced Features (Planned)
- Multi-environment support
- RBAC enhancements
- Registry integration
- Advanced networking features

### Phase 4: UI/UX Refinement (Planned)
- Modern, responsive UI
- Dark/light mode
- Real-time updates via WebSockets
- Dashboard customization

### Phase 5: Enterprise Features (Future)
- Backup and restore
- High availability
- Monitoring and alerting
- Audit logging

## Detailed Feature Plans

### 1. Authentication and Role-Based Access Control (RBAC)

**Status**: Partially Implemented

**Current Implementation**:
- Basic JWT authentication
- Login/logout functionality
- Admin user creation

**Remaining Work**:
- Implement role-based permissions (Admin, Operator, Viewer)
- Create user management interface for administrators
- Add user profile management

### 2. Container Management

**Status**: Partially Implemented

**Current Implementation**:
- Basic container routes defined
- Placeholder UI for container management

**Remaining Work**:
- Implement container list view with filtering and search
- Create container detail view showing logs, stats, and configuration
- Add container creation form with image selection, port mapping, volume mounting
- Implement container actions (start, stop, restart, delete)

### 3. Volume Management

**Status**: Not Implemented

**Implementation Plan**:
- Create volume list view
- Implement volume creation, deletion, and inspection
- Add volume mounting to container creation/edit forms

### 4. Network Management

**Status**: Not Implemented

**Implementation Plan**:
- Create network list view
- Implement network creation with subnet configuration
- Add network connection to container creation/edit forms

### 5. Docker Compose Integration

**Status**: Partially Implemented

**Current Implementation**:
- Created models for Docker Compose projects
- Basic YAML parsing and validation
- Created storage for compose configurations
- Basic compose up/down operations
- Simple compose project management interface

**Remaining Work**:
- Complete the compose file editor with syntax highlighting
- Enhance compose file validation
- Improve support for environment variables
- Add support for scaling services
- Create stack templates system

### 6. Application Creation and Service Routing

**Status**: Partially Implemented

**Current Implementation**:
- Created models for applications (container + service/ingress)
- Implemented domain-based routing system
- Set up port 80 listener for routing traffic to containers
- Basic service management interface
- Basic SSL/TLS support

**Remaining Work**:
- Complete service creation interface
- Enhance domain-based routing configuration
- Improve SSL/TLS support
- Add DNS integration for domain resolution
- Implement automatic service discovery
- Add traffic metrics and monitoring
- Add support for custom network configurations

### 7. UI/UX Refinement

**Status**: Partially Implemented

**Current Implementation**:
- Basic dark/light theme support
- Simple responsive design

**Remaining Work**:
- Complete responsive design for all pages
- Implement proper dark/light theme support
- Add real-time updates via WebSockets
- Create comprehensive dashboard with system overview

## Architecture Evolution

### Current Architecture
- Rust backend with Axum
- Server-side rendered HTML with direct HTML responses
- Minimal JavaScript with HTMX for dynamic updates
- Alpine.js for client-side behavior

### Previous Architecture
- Rust backend with Axum
- React frontend with TypeScript
- Axios for API communication

### Technical Differentiators
- WebAssembly support for client-side Rust code
- Efficient resource usage with lower memory footprint
- API-first design with well-documented endpoints
- Plugin system for extensibility

## Implementation Timeline

1. **Phase 1**: Q2 2025 (4 weeks)
2. **Phase 2**: Q2 2025 (3 weeks)
3. **Phase 2.5**: Q2-Q3 2025 (4 weeks)
4. **Phase 3**: Q3 2025 (5 weeks)
5. **Phase 4**: Q4 2025 (2 weeks)
6. **Phase 5**: Q1 2026 (4 weeks)

## Development Approach

1. **Feature Development**:
   - Implement backend functionality first
   - Create API endpoints
   - Develop frontend components
   - Integrate and test

2. **Testing Strategy**:
   - Unit tests for core functionality
   - Integration tests for API endpoints
   - End-to-end tests for critical flows
   - Visual regression testing for UI

3. **Deployment Strategy**:
   - Single binary deployment
   - Docker container for easy installation
   - Docker Compose setup for development

## Contributing

Contributions are welcome! Please see the README.md file for information on how to get started with development.