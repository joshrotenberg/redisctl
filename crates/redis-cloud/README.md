# redis-cloud

A comprehensive Rust client library for the Redis Cloud REST API.

## Features

- Complete coverage of Redis Cloud REST API endpoints
- Async/await support with tokio
- Strong typing for API requests and responses
- Comprehensive error handling
- Support for all Redis Cloud features including:
  - Subscriptions and databases
  - User and ACL management
  - Backup and restore operations
  - VPC peering and networking
  - Metrics and monitoring
  - Billing and payment management

## Installation

```toml
[dependencies]
redis-cloud = "0.1.0"
```

## Usage

```rust
use redis_cloud::{CloudClient, CloudClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CloudClientConfig {
        api_key: "your-api-key".to_string(),
        secret_key: "your-secret-key".to_string(),
        base_url: None, // Uses default https://api.redislabs.com/v1
    };

    let client = CloudClient::new(config)?;
    
    // List all subscriptions
    let subscriptions = client.list_subscriptions(None).await?;
    println!("Subscriptions: {:?}", subscriptions);
    
    // Get account information
    let account = client.get_account().await?;
    println!("Account: {:?}", account);
    
    Ok(())
}
```

## API Coverage

This library provides comprehensive coverage of the Redis Cloud REST API, including:

- **Account Management** - Account info, users, payment methods
- **Subscriptions** - CRUD operations, pricing, CIDR management
- **Databases** - Full database lifecycle, backups, imports, metrics
- **ACL Management** - Users, roles, Redis rules
- **Networking** - VPC peering, Transit Gateway, Private Service Connect
- **Monitoring** - Metrics, logs, alerts
- **Billing** - Invoices, payment methods, usage

## Documentation

For detailed API documentation, see the [Redis Cloud API Reference](https://api.redislabs.com/v1/swagger-ui/index.html).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.