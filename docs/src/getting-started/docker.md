# Docker Development Environment

The Redis Enterprise CLI includes a comprehensive Docker setup for development and testing. This environment provides a real Redis Enterprise cluster without requiring manual setup.

## Overview

Our Docker environment includes:
- **Redis Enterprise cluster** for local development
- **Automated cluster initialization** using our CLI workflows
- **Multiple service profiles** for different testing scenarios
- **Development tooling** with live code mounting
- **Performance testing** and debugging capabilities

## Quick Start

```bash
# Start Redis Enterprise cluster
docker compose up -d

# Access the cluster
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_INSECURE="true"
redisctl enterprise cluster info

# Clean up
docker compose down -v
```

## Using Docker Compose

The Docker Compose setup provides a Redis Enterprise cluster with automatic initialization:

```bash
# Start Redis Enterprise with auto-initialization
docker compose up -d

# Check cluster status
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@redis.local"
export REDIS_ENTERPRISE_PASSWORD="Redis123!"
export REDIS_ENTERPRISE_INSECURE="true"
redisctl enterprise cluster info

# View databases
redisctl enterprise database list

# Clean up when done
docker compose down -v
```

**Includes:**
- Redis Enterprise server
- Automatic cluster initialization via workflow
- Ready-to-use configuration

## Testing with Docker

### Running Tests Against the Cluster

Once your cluster is running, you can test various commands:

```bash
# Test cluster commands
redisctl enterprise cluster info
redisctl enterprise node list
redisctl enterprise database list

# Create a test database
redisctl enterprise database create --data '{
  "name": "test-db",
  "memory_size": 1073741824,
  "port": 12000
}'

# Test with different output formats
redisctl enterprise database list -o yaml
redisctl enterprise database list -o table

# Use verbose logging for debugging
RUST_LOG=debug redisctl enterprise cluster info
```

### Interactive Testing

For interactive testing, you can use a temporary container:

```bash
# Run interactive shell with redisctl
docker run --rm -it \
  --network redisctl_redisctl-network \
  -e REDIS_ENTERPRISE_URL="https://redis-enterprise:9443" \
  -e REDIS_ENTERPRISE_INSECURE="true" \
  -e REDIS_ENTERPRISE_USER="admin@redis.local" \
  -e REDIS_ENTERPRISE_PASSWORD="Redis123!" \
  ghcr.io/redis-developer/redisctl:latest \
  /bin/sh

# Inside the container, run commands
redisctl enterprise cluster info
redisctl enterprise database list
```

## Environment Variables

Configure the Docker environment via `.env` file (if needed):

```bash
# Copy example environment file (optional)
cp .env.example .env

# Edit .env to customize:
# - REDIS_ENTERPRISE_IMAGE: Docker image to use
# - REDIS_ENTERPRISE_PLATFORM: Platform architecture
```

Control logging and behavior:

```bash
# Set log level
RUST_LOG=debug docker compose up

# Component-specific logging
RUST_LOG="redis_enterprise=trace,redisctl=debug" docker compose up
```

## Development Workflow

### Typical Development Session

```bash
# 1. Start development environment
docker compose up -d

# 2. Build and test your changes locally
cargo build --release
./target/release/redisctl enterprise cluster info

# 3. Test with Docker image
docker build -t redisctl:dev .
docker run --rm \
  --network redisctl_redisctl-network \
  -e REDIS_ENTERPRISE_URL="https://redis-enterprise:9443" \
  -e REDIS_ENTERPRISE_INSECURE="true" \
  -e REDIS_ENTERPRISE_USER="admin@redis.local" \
  -e REDIS_ENTERPRISE_PASSWORD="Redis123!" \
  redisctl:dev enterprise cluster info

# 4. Clean up
docker compose down -v
```

### Testing New Features

```bash
# Start basic environment
docker compose up -d

# Test your new command locally
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@redis.local"
export REDIS_ENTERPRISE_PASSWORD="Redis123!"
export REDIS_ENTERPRISE_INSECURE="true"

# Run your new command
cargo run -- enterprise your-new-command

# Or test with the release build
cargo build --release
./target/release/redisctl enterprise your-new-command
```

### Debugging Connection Issues

```bash
# Check Redis Enterprise health
docker compose ps
docker compose logs redis-enterprise

# Test connectivity directly
curl -k https://localhost:9443/v1/bootstrap

# Test with verbose logging
RUST_LOG=debug redisctl enterprise cluster info

# Check network connectivity from container
docker run --rm \
  --network redisctl_redisctl-network \
  alpine/curl \
  curl -k https://redis-enterprise:9443/v1/bootstrap
```

## Service Architecture

### Main Services

- **redis-enterprise**: Redis Enterprise server
- **redis-enterprise-init**: Automatic cluster initialization using the workflow command

### Networking

All services use the `redisctl-network` bridge network:
- Redis Enterprise API: `https://redis-enterprise:9443` (external: `https://localhost:9443`)
- Web UI: `https://redis-enterprise:8443` (external: `https://localhost:8443`)  
- Database ports: `12000-12010`

### Volumes

- **enterprise-data**: Persistent Redis Enterprise data
- **Source mounting**: Development containers access project files

## Troubleshooting

### Common Issues

**Port Conflicts:**
```bash
# Check if ports are in use
lsof -i :9443
lsof -i :8443

# Stop conflicting services
docker compose down
```

**Platform Compatibility Issues:**
```bash
# If you encounter platform issues, check Docker settings
docker version

# Ensure Docker Desktop is configured for your platform
# Try pulling the image manually
docker pull redis/redis-stack-server:latest
```

**Permission Issues:**
```bash
# Reset Docker volumes
docker compose down -v
docker compose up -d
```

**Build Issues:**
```bash
# Force rebuild
docker compose build --no-cache
docker compose up --force-recreate
```

### Debugging Commands

```bash
# Check service status  
docker compose ps

# View logs
docker compose logs -f enterprise
docker compose logs -f enterprise-init

# Execute commands in running container
docker compose exec cli sh
docker compose exec enterprise bash

# Check network connectivity
docker compose exec cli ping enterprise
docker compose exec cli curl -k https://enterprise:9443/v1/bootstrap
```

## Best Practices

### Development

- Use `docker compose up -d` for complete environment setup
- Build locally with `cargo build --release` for development
- Use verbose logging (`RUST_LOG=debug`) for debugging
- Always clean up with `docker compose down -v`

### Testing

- Always test against real Redis Enterprise
- Test all output formats (JSON, YAML, table)
- Clean up test data between runs
- Verify error handling with invalid inputs

### Performance

- Use performance profile to validate changes
- Monitor resource usage during development
- Test with realistic data sizes
- Validate API response times