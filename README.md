# Rustainer

A lightweight container management UI built in Rust, inspired by Portainer and Dockge.

## Overview

Rustainer is a simple yet powerful container management tool that provides:

1. A proxy server on port 80 that routes traffic to Docker containers based on domain names
2. An admin UI on port 3000 for managing containers, applications, and configurations
3. Easy application deployment with domain-based routing

## Features

- **Container Management**: View, create, start, stop, and restart Docker containers
- **Application Management**: Deploy applications with domain-based routing
- **Domain Routing**: Route traffic to containers based on domain names
- **Admin UI**: Simple and intuitive web interface for management
- **Authentication**: Secure access with JWT-based authentication
- **Docker Integration**: Direct integration with Docker API

## Architecture

Rustainer consists of two main components:

1. **Proxy Server (Port 80)**: Routes incoming HTTP requests to the appropriate container based on the domain name in the request
2. **Admin UI (Port 3000)**: Web interface for managing containers, applications, and configurations

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Frontend**: Server-side rendered HTML with Askama templates
- **Styling**: Custom CSS with dark/light theme support
- **Docker API**: Bollard Rust client
- **Database**: SQLite with SQLx
- **Authentication**: JWT tokens

## Getting Started

### Prerequisites

- Rust (latest stable)
- Docker Engine
- SQLite

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/ao/rustainer.git
   cd rustainer
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   ./target/release/rustainer
   ```

4. Access the web interface at `http://localhost:3000`

### Default Login

- Username: `admin`
- Password: `admin`

## Development

1. Clone the repository
2. Install dependencies:
   ```
   cargo build
   ```

3. Run in development mode:
   ```
   cargo run
   ```

## Project Structure

```
rustainer/
├── src/
│   ├── api/           # API handlers for REST endpoints
│   ├── auth/          # Authentication and authorization
│   ├── config/        # Configuration management
│   ├── db/            # Database operations and migrations
│   ├── docker/        # Docker API interactions
│   ├── models/        # Data models
│   ├── proxy/         # Proxy server implementation
│   ├── static/        # Static assets (CSS, JS)
│   ├── templates/     # HTML templates
│   ├── web/           # Web handlers for UI pages
│   └── main.rs        # Application entry point
├── data/              # Application data (database)
```

## License

MIT

## Acknowledgements

- Inspired by [Portainer](https://www.portainer.io/) and [Dockge](https://github.com/louislam/dockge)
- Built with [Axum](https://github.com/tokio-rs/axum)
- Docker API access via [Bollard](https://github.com/fussybeaver/bollard)
