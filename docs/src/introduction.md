# Redis Enterprise Rust CLI & API Client

Feature-complete command-line interface and Rust library for managing Redis Enterprise clusters.

## Project Status

**✅ Feature Complete**: This project provides full implementation of all supported Redis Enterprise endpoints with comprehensive testing and documentation.

## Primary Components

### 1. Command-Line Interface (CLI)
The primary way to interact with Redis Enterprise clusters, featuring:
- **All Operations Exposed**: Every API endpoint accessible via intuitive commands
- **Multiple Output Formats**: JSON, YAML, and Table output
- **JMESPath Queries**: Powerful filtering and transformation of output
- **Configuration Profiles**: Save and switch between multiple clusters
- **Workflow Commands**: High-level operations for common tasks
- **Scripting Ready**: Exit codes, JSON output, and environment variables

```bash
# Quick example
redis-enterprise database create --name mydb --memory 1GB
redis-enterprise database list --output table
redis-enterprise cluster info --query 'name'
```

### 2. Rust Library
For programmatic access and building custom tools:
- **Type-Safe API**: Full Rust type system integration
- **Builder Pattern**: Fluent API for complex configurations
- **Async/Await**: Built on Tokio for high performance
- **100+ Integration Tests**: Comprehensive test coverage

```rust
use redis_enterprise::EnterpriseClient;

let client = EnterpriseClient::builder()
    .base_url("https://cluster.example.com:9443")
    .username("admin@example.com")
    .password("your-password")
    .build()?;

let databases = client.databases().list().await?;
```

## Features

- **Complete Implementation**: All major Redis Enterprise endpoints fully implemented
- **8 Major Endpoints**: Cluster, Database, Node, User, Bootstrap, Module, Role, License
- **45+ REST Operations**: Full CRUD operations for all resources
- **100+ CLI Commands**: Intuitive commands for every operation
- **Configuration Management**: Save credentials and switch between clusters
- **Docker Test Environment**: Complete testing setup with multiple profiles

## Quick Links

### Getting Started
- [Installation Guide](./getting-started/installation.md)
- [Quick Start](./getting-started/quickstart.md)
- [Docker Environment](./getting-started/docker.md)

### CLI Documentation (Primary Interface)
- [CLI Commands Reference](./cli/commands.md)
- [Examples](./cli/examples.md)
- [Configuration](./cli/configuration.md)
- [Output Formats](./cli/output-formats.md)

### Workflows
- [Cluster Bootstrap](./workflows/cluster-bootstrap.md)
- [Database Creation](./workflows/database-creation.md)

### API Documentation
- [API Client Reference](./api/endpoints-overview.md)
- [REST API Reference](./rest-api/cluster.md)

## Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/joshrotenberg/redis-enterprise-rs/issues)
- **Redis Documentation**: [Official Redis Enterprise docs](https://redis.io/docs/latest/operate/rs/)
- **API Reference**: [REST API documentation](https://redis.io/docs/latest/operate/rs/references/rest-api/)