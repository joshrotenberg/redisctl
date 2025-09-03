# redis-enterprise

A comprehensive Rust client library for the Redis Enterprise REST API.

## Features

- Complete coverage of Redis Enterprise REST API endpoints
- Async/await support with tokio
- Strong typing for API requests and responses
- Comprehensive error handling
- Support for all Redis Enterprise features including:
  - Cluster management and bootstrap
  - Database (BDB) operations
  - Node management and statistics
  - User and role management
  - Redis modules
  - Active-Active (CRDB) databases
  - Monitoring and alerts

## Installation

```toml
[dependencies]
redis-enterprise = "0.1.0"
```

## Quick Start

```rust
use redis_enterprise::EnterpriseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client using builder pattern
    let client = EnterpriseClient::builder()
        .url("https://cluster.example.com:9443")
        .username("admin@example.com")
        .password("your-password")
        .insecure(false) // Set to true for self-signed certificates
        .build()?;
    
    // Get cluster information
    let cluster = client.cluster().info().await?;
    println!("Cluster: {:?}", cluster);
    
    // List databases (BDBs)
    let databases = client.database().list().await?;
    println!("Databases: {:?}", databases);
    
    // Get node statistics
    let nodes = client.node().list().await?;
    println!("Nodes: {:?}", nodes);
    
    Ok(())
}
```

## Examples

The `examples/` directory contains runnable examples demonstrating common use cases:

- [`basic_enterprise.rs`](examples/basic_enterprise.rs) - Getting started with cluster connection
- [`database_management.rs`](examples/database_management.rs) - Managing databases and viewing statistics

Run examples with:
```bash
# Set your cluster credentials
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@redis.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"  # For self-signed certificates

# Run an example
cargo run --example basic
```

## API Coverage

This library provides 100% coverage of the Redis Enterprise REST API, including:

- **Cluster Operations** - Bootstrap, configuration, topology
- **Database Management** - CRUD operations, actions, statistics
- **Node Management** - Add/remove nodes, statistics, actions
- **Security** - Users, roles, ACLs, LDAP integration
- **Modules** - Upload and manage Redis modules
- **Monitoring** - Stats, alerts, logs, diagnostics
- **Active-Active** - CRDB management and tasks
- **Administration** - License, certificates, services

## Documentation

For detailed API documentation, see the [Redis Enterprise REST API Reference](https://docs.redis.com/latest/rs/references/rest-api/).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.