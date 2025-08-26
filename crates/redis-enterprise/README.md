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

## Usage

```rust
use redis_enterprise::{EnterpriseClient, EnterpriseClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EnterpriseClientConfig {
        base_url: "https://cluster.example.com:9443".to_string(),
        username: "admin@example.com".to_string(),
        password: "your-password".to_string(),
        insecure: false, // Set to true for self-signed certificates
    };

    let client = EnterpriseClient::new(config)?;
    
    // Get cluster information
    let cluster = client.get_cluster_info().await?;
    println!("Cluster: {:?}", cluster);
    
    // List databases
    let databases = client.list_databases().await?;
    println!("Databases: {:?}", databases);
    
    // Get node statistics
    let stats = client.get_node_stats("1").await?;
    println!("Node stats: {:?}", stats);
    
    Ok(())
}
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