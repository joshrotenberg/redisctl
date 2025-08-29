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

## Quick Start

```rust
use redis_cloud::CloudClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client using builder pattern
    let client = CloudClient::builder()
        .api_key("your-api-key")
        .api_secret("your-api-secret")
        .build()?;
    
    // Get account information
    let account = client.account().get().await?;
    println!("Account: {:?}", account);
    
    // List all subscriptions
    let subscriptions = client.subscription().list().await?;
    println!("Subscriptions: {:?}", subscriptions);
    
    // List databases in a subscription
    let databases = client.database().list("subscription-id").await?;
    println!("Databases: {:?}", databases);
    
    Ok(())
}
```

## Examples

The `examples/` directory contains runnable examples demonstrating common use cases:

- [`basic.rs`](examples/basic.rs) - Getting started with the API client
- [`database_management.rs`](examples/database_management.rs) - Managing databases

Run examples with:
```bash
# Set your API credentials
export REDIS_CLOUD_API_KEY="your-api-key"
export REDIS_CLOUD_API_SECRET="your-api-secret"

# Run an example
cargo run --example basic
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