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

This will build and start Rustainer, making it available at http://localhost:3000.

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
- Create reusable container templates with predefined configurations
- Deploy containers from templates with customizable options
- Organize templates by category for easy access
- Share templates across your organization

### User Interface
- Clean, responsive design that works on desktop and mobile
- Dark mode support for comfortable viewing in low-light environments
- Real-time updates for container status and statistics
- Intuitive navigation and clear visual indicators

## 🔒 Security Considerations
Rustainer connects directly to the Docker socket, which provides full control over your Docker environment. Be cautious when exposing Rustainer to untrusted networks or users.

## 🆕 Latest Updates

### April 2025 Update
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
