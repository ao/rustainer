# Rustainer

A lightweight container management UI built in Rust, inspired by Portainer.

## Features

- Container management (view, create, start, stop, restart)
- Volume management
- Network management
- Docker Compose stack management
- User authentication
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

## License

MIT

## Acknowledgements

- Inspired by [Portainer](https://www.portainer.io/)
- Built with [Axum](https://github.com/tokio-rs/axum)
- Docker API access via [Bollard](https://github.com/fussybeaver/bollard)
