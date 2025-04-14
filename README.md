# 🚢 Rustainer — A Lightweight Portainer Alternative

## 💡 What is This?
Rustainer is a simple, fast, and minimal container management UI built entirely in Rust. It connects to the local Docker Engine via the Docker socket and offers a REST API and a web UI to:
- List, start, stop, and restart containers
- View container logs and detailed statistics
- Create new containers with customizable configurations
- Manage volumes with detailed information
- Advanced network management with IPAM configuration
- Deploy and manage Docker Compose stacks
- Create and use container templates for quick deployment
- Service proxy for domain-based routing to containers and static sites
- Dark mode support for comfortable viewing

This is a **lightweight, local-first alternative to Portainer**, built for developers and homelabbers who want speed, control, and simplicity — no Electron, no JS-heavy UI unless needed.

## 🚀 Getting Started

### Prerequisites
- Docker installed and running
- Rust (if building from source)
- Node.js and npm (if developing the frontend)

### Quick Start with Docker Compose
The easiest way to run Rustainer is using Docker Compose:

```bash
docker-compose up -d
```
This will build and start Rustainer, making it available at:
- http://localhost:801 for the admin UI (HTTP)
- https://localhost:4431 for the admin UI (HTTPS)
- http://localhost:80 for the service proxy (HTTP)
- https://localhost:443 for the service proxy (HTTPS)

Note that you may need to run Docker with elevated privileges to bind to ports 80 and 443.
This will build and start Rustainer, making it available at http://localhost:801 for the admin UI and port 80 for the service proxy.

### Building from Source

#### Backend
```bash
# Build the Rust backend
cargo build --release
```

#### Frontend
```bash
# Navigate to the frontend directory
cd frontend

# Install dependencies
npm install

# Build the frontend
npm run build
```

#### Running the Application
```bash
# Run the backend (which will serve the frontend)
./target/release/rustainer
```

## 🧰 Development

### Backend Development
```bash
# Run the backend in development mode
cargo run
```

### Frontend Development
```bash
# Navigate to the frontend directory
cd frontend

# Install dependencies
npm install

# Start the development server
npm start
```

## 📚 API Endpoints

### Containers
- `GET /api/containers` - List all containers
- `POST /api/containers/:id/start` - Start a container
- `POST /api/containers/:id/stop` - Stop a container
- `POST /api/containers/:id/restart` - Restart a container
- `GET /api/containers/:id/logs` - Get container logs
- `POST /api/containers/create` - Create a new container
- `GET /api/containers/:id/stats` - Get container stats

### Volumes
- `GET /api/volumes` - List all volumes

### Networks
- `GET /api/networks` - List all networks
- `GET /api/networks/:id` - Get network details
- `POST /api/networks` - Create a new network
- `DELETE /api/networks/:id` - Delete a network
- `POST /api/networks/:id/connect` - Connect a container to a network
- `POST /api/networks/:id/disconnect` - Disconnect a container from a network
- `POST /api/networks/prune` - Prune unused networks
- `GET /api/networks/:id/diagnostics` - Get network diagnostics

### Docker Compose
- `GET /api/compose` - List all compose stacks
- `POST /api/compose` - Create a new compose stack
- `GET /api/compose/:id` - Get compose stack details
- `POST /api/compose/:id` - Update a compose stack
- `DELETE /api/compose/:id` - Delete a compose stack
- `POST /api/compose/:id/start` - Start a compose stack
- `POST /api/compose/:id/stop` - Stop a compose stack
- `POST /api/compose/:id/restart` - Restart a compose stack
- `GET /api/compose/:id/logs` - Get compose stack logs

### Templates
- `GET /api/templates` - List all templates
- `POST /api/templates` - Create a new template
- `GET /api/templates/:id` - Get template details
- `POST /api/templates/:id` - Update a template
- `DELETE /api/templates/:id` - Delete a template
- `POST /api/templates/deploy` - Deploy a container from a template

### Services
- `GET /api/services` - List all services
- `POST /api/services` - Create a new service
- `GET /api/services/:id` - Get service details
- `POST /api/services/:id` - Update a service
- `DELETE /api/services/:id` - Delete a service
- `POST /api/services/:id/enable` - Enable a service
- `POST /api/services/:id/disable` - Disable a service

## ✨ Features

### Container Management
- View detailed container information including status, ports, and environment variables
- Start, stop, and restart containers with a single click
- View real-time container logs with automatic updates
- Monitor container resource usage with detailed statistics
- Create new containers with customizable configurations

### Volume Management
- List all volumes with detailed information
- View volume mountpoints and usage

### Advanced Network Management
- Create custom networks with specific drivers and subnet configurations
- Connect and disconnect containers from networks with custom IP configurations
- View detailed network information including IPAM configuration and connected containers
- Prune unused networks to clean up the Docker environment
- View network diagnostics and metrics

### Docker Compose Integration
- Deploy multi-container applications using Docker Compose
- Manage compose stacks with start, stop, and restart operations
- View logs for all services in a compose stack
- Edit compose files directly in the UI

### Container Templates
## 🏗️ Architecture

### Service Proxy Architecture
Rustainer implements a service proxy that allows you to route traffic to different services based on domain names. The architecture consists of:

1. **Service Definitions**: Each service is defined with a domain, target (container, static site, or custom URL), and configuration options.

2. **Proxy Router**: The main entry point for all HTTP requests, which:
   - Extracts the domain from the Host header
   - Looks up the corresponding service
   - Routes the request to the appropriate handler based on service type

3. **Service Types**:
   - **Container**: Routes requests to a Docker container
   - **StaticSite**: Serves static files from a directory
   - **CustomURL**: Forwards requests to an external URL

4. **SSL/TLS Support**: Optional SSL/TLS termination with Let's Encrypt integration for automatic certificate generation

The service proxy runs on port 80 (and 443 for HTTPS), while the admin UI runs on port 801 (and 4431 for HTTPS), allowing you to manage your services without interfering with the proxy functionality.

- Create reusable container templates with predefined configurations
## 🔄 Using the Service Proxy

### Creating a Service

To create a new service:

1. Navigate to the Services page in the admin UI
2. Click "Create Service"
3. Fill in the service details:
   - **Name**: A descriptive name for the service
   - **Domain**: The domain name to route (e.g., example.com)
   - **Type**: Container, StaticSite, or CustomURL
   - **Target**: The container name, file path, or URL to route to
   - **Port**: The port to expose the service on
   - **SSL**: Configure SSL/TLS settings if needed

### Managing Services

From the Services page, you can:
- **Enable/Disable** services with a single click
- **Edit** service configurations
- **Delete** services that are no longer needed

### Accessing Services

Once a service is created and enabled, you can access it by:
1. Ensuring your DNS points to the Rustainer host for the domain
2. Accessing the domain in your browser (e.g., http://example.com)

The service proxy will automatically route the request to the appropriate target based on the domain.

- Deploy containers from templates with customizable options
- Organize templates by category for easy access
- Share templates across your organization
### Service Proxy
- Domain-based routing to containers, static sites, and custom URLs
- SSL/TLS support with auto-generation via Let's Encrypt
- Custom header management for proxied requests
- Enable/disable services with a single click
- Centralized management of all your web services

### User Interface
- Clean, responsive design that works on desktop and mobile
- Dark mode support for comfortable viewing in low-light environments
- Real-time updates for container status and statistics
- Intuitive navigation and clear visual indicators

## 🔒 Security Considerations
Rustainer connects directly to the Docker socket, which provides full control over your Docker environment. Be cautious when exposing Rustainer to untrusted networks or users.

## 🆕 Latest Updates

### April 2025 Update
- Added service proxy for domain-based routing to containers and static sites
- Added advanced network management features with IPAM configuration
- Implemented container templates for quick deployment
- Added Docker Compose integration for multi-container applications
- Improved UI with dark mode support and responsive design
- Fixed styling issues in the resource usage section
- Enhanced API with comprehensive endpoints for all features

## 🤝 Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License
This project is licensed under the MIT License - see the LICENSE file for details.
