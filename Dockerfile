# Stage 1: Build the Rust backend
FROM rust:latest as backend-builder

WORKDIR /app

# Copy Rust project files
COPY Cargo.toml .
COPY src ./src

# Build the Rust application
RUN cargo build --release

# Stage 2: Build the React frontend
FROM node:18-alpine as frontend-builder

WORKDIR /app

# Copy frontend package.json
COPY frontend/package.json ./
# Install dependencies
RUN npm install

# Copy the rest of the frontend files
COPY frontend ./
# Build the frontend
RUN npm run build

# Stage 3: Create the final image
FROM rust:slim

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the Rust binary from the build stage
COPY --from=backend-builder /app/target/release/rustainer /app/rustainer

# Copy the frontend build from the frontend build stage
COPY --from=frontend-builder /app/build /app/frontend

# Expose the port
EXPOSE 3000

# Set the entrypoint
CMD ["/app/rustainer"]