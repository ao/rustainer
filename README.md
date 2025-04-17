# Rustainer

A lightweight container management UI built in Rust, inspired by Portainer and Dockge.

## Current Status

Rustainer is under active development. The following features are currently implemented:

- Basic server setup with authentication
- Docker Compose stack management (partial implementation)
- Service routing system (domain-based routing to containers)
- Proxy server on port 80 for routing traffic
- User authentication with JWT
- Basic dark/light theme support

See the [ROADMAP.md](ROADMAP.md) file for detailed implementation status and future plans.

## Features (Planned)

- Container management (view, create, start, stop, restart)
- Volume management
- Network management
- Docker Compose stack management
- Application creation with service/ingress
- Domain-based routing to containers (port 80)
- Management UI (port 3000)
- User authentication with role-based access control
- Dark/light theme support
- Responsive design

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
   git clone https://github.com/yourusername/rustainer.git
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
│   ├── db/            # Database operations and migrations
│   ├── docker/        # Docker API interactions
│   ├── models/        # Data models
│   ├── static/        # Static assets (CSS, JS)
│   ├── templates/     # HTML templates
│   ├── web/           # Web handlers for UI pages
│   ├── websocket/     # WebSocket handlers for real-time updates
│   ├── app_state.rs   # Application state
│   ├── config.rs      # Configuration
│   └── main.rs        # Application entry point
├── data/              # Application data (database, compose files)
├── IMPLEMENTATION_PLAN.md  # Detailed implementation plan
└── ROADMAP.md         # Feature roadmap and status
```

## Implementation Plan

See the [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) file for a detailed implementation plan.

## License

MIT

## Acknowledgements

- Inspired by [Portainer](https://www.portainer.io/) and [Dockge](https://github.com/louislam/dockge)
- Built with [Axum](https://github.com/tokio-rs/axum)
- Docker API access via [Bollard](https://github.com/fussybeaver/bollard)
