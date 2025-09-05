# Using the Libraries

The `redis-cloud` and `redis-enterprise` crates can be used independently in your Rust projects.

## Installation

```toml
[dependencies]
redis-cloud = "0.2"
redis-enterprise = "0.2"
```

## Basic Usage

### Redis Cloud Client

```rust
use redis_cloud::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        "your-api-key",
        "your-api-secret",
    )?;

    // Get account info
    let account = client.get_raw("/account").await?;
    println!("{}", account);

    Ok(())
}
```

### Redis Enterprise Client

```rust
use redis_enterprise::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(
        "https://cluster:9443",
        "admin@cluster.local",
        "password",
        true, // insecure
    )?;

    // Get cluster info
    let cluster = client.get_raw("/v1/cluster").await?;
    println!("{}", cluster);

    Ok(())
}
```

More documentation coming soon.