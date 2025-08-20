# Docker Development Environment

The Redis Enterprise CLI includes a comprehensive Docker setup for development and testing. This environment provides a real Redis Enterprise cluster without requiring manual setup.

## Overview

Our Docker environment includes:
- **Redis Enterprise cluster** with ARM Mac compatibility
- **Automated cluster initialization** using our CLI workflows
- **Multiple service profiles** for different testing scenarios
- **Development tooling** with live code mounting
- **Performance testing** and debugging capabilities

## Quick Start

```bash
# Start Redis Enterprise and initialize cluster
make docker-up

# Access interactive CLI
make docker-cli

# Run tests against the cluster
make docker-test

# Clean up
make docker-down
```

## Service Profiles

### Default Profile

Starts Redis Enterprise with basic setup:

```bash
docker compose up -d
```

**Includes:**
- Redis Enterprise server
- Cluster initialization
- Basic test database

### CLI Profile

Interactive container for manual testing:

```bash
docker compose --profile cli up cli
# or
make docker-cli
```

**Features:**
- Pre-configured credentials
- Interactive shell with CLI installed
- Project source mounted for reference

### Examples Profile

Demonstrates workflow commands:

```bash
docker compose --profile examples up
# or
make docker-examples
```

**Creates:**
- Cache database (optimized for caching)
- Persistent database (with AOF persistence)
- Displays results in table format

### Monitor Profile

Continuous monitoring of cluster status:

```bash
docker compose --profile monitor up
# or
make docker-monitor
```

**Shows:**
- Cluster information every 30 seconds
- Database list with current status
- Useful for watching changes during development

## Advanced Development Features

### Testing Profile

Comprehensive test suite against live cluster:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile test up
# or
make docker-test
```

**Tests:**
- All cluster commands
- Database CRUD operations
- Node and user management
- Direct API access
- Output format validation

### All Database Types

Creates every supported database type:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile all-dbs up
# or
make docker-all-dbs
```

**Database Types:**
- **cache**: High-performance caching with LRU eviction
- **persistent**: Durable storage with AOF persistence  
- **search**: RediSearch for full-text search
- **timeseries**: RedisTimeSeries for time-series data
- **json**: RedisJSON for JSON document storage
- **graph**: RedisGraph for graph database operations

### Performance Testing

Load testing against the cluster:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile perf up
# or
make docker-perf
```

**Tests:**
- 100+ rapid database list operations
- 100+ rapid database get operations
- Performance timing measurements
- Stress testing API responsiveness

### Debug Mode

Development container with verbose logging:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile debug up
# or
make docker-debug
```

**Features:**
- `RUST_LOG=trace` for maximum verbosity
- Full backtrace enabled
- Source code mounted for live development
- Interactive shell for debugging

### Cleanup

Remove all test databases:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile cleanup up
# or
make docker-cleanup
```

## Environment Variables

Control logging and behavior:

```bash
# Set log level
RUST_LOG=debug docker compose up

# Component-specific logging
RUST_LOG="redis_enterprise=trace,redis_enterprise_cli=debug" docker compose up

# Use different Redis Enterprise image
ENTERPRISE_IMAGE=redislabs/redis:latest docker compose up
```

## Development Workflow

### Typical Development Session

```bash
# 1. Start development environment
make dev

# 2. Make changes to CLI code
# 3. Test changes
make docker-test

# 4. Try new features
make docker-cli

# 5. Debug issues if needed
make docker-debug

# 6. Clean up
make dev-clean
```

### Testing New Features

```bash
# Start basic environment
docker compose up -d

# Test your new command
docker compose exec redis-enterprise-cli-interactive redis-enterprise your-new-command

# Or rebuild and test
docker compose build redis-enterprise-cli
docker compose up --force-recreate enterprise-init
```

### Debugging Connection Issues

```bash
# Check Redis Enterprise health
docker compose ps
docker compose logs enterprise

# Test CLI connectivity
docker compose exec cli redis-enterprise --insecure cluster info

# Debug with verbose logging
RUST_LOG=debug docker compose exec cli redis-enterprise --insecure --verbose cluster info
```

## Service Architecture

### Main Services

- **enterprise**: Redis Enterprise server (ARM-compatible)
- **enterprise-init**: Cluster initialization using workflows  
- **enterprise-db-create**: Creates initial test database
- **cli**: Interactive CLI container

### Development Services

- **enterprise-db-examples**: Workflow demonstrations
- **monitor**: Continuous cluster monitoring
- **test-runner**: Automated test execution
- **create-all-db-types**: Database type showcase
- **perf-test**: Performance validation
- **debug**: Development debugging container

### Networking

All services use the `redis-net` bridge network:
- Redis Enterprise API: `https://enterprise:9443`
- Web UI: `https://enterprise:8443`  
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

**ARM Mac Issues:**
```bash
# Ensure using ARM-compatible image
ENTERPRISE_IMAGE=kurtfm/rs-arm:latest docker compose up
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

- Use `make dev` for complete environment setup
- Use profiles to run only needed services
- Mount source code for live development
- Use verbose logging for debugging

### Testing

- Always test against real Redis Enterprise
- Use different profiles for different test scenarios  
- Clean up test data between runs
- Verify all output formats work correctly

### Performance

- Use performance profile to validate changes
- Monitor resource usage during development
- Test with realistic data sizes
- Validate API response times