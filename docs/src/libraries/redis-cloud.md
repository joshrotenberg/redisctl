# Redis Cloud Rust Library

The `redis-cloud` crate provides a typed Rust client for the Redis Cloud REST API. It supports subscriptions, databases, ACLs, SSO, billing, logs, metrics, CRDB, and networking endpoints.

- Crate: `redis-cloud`
- Rustdoc: https://docs.rs/redis-cloud/latest/redis_cloud/

## Install

```toml
[dependencies]
redis-cloud = "0.1"
tokio = { version = "1", features = ["full"] }
```

Or:

```bash
cargo add redis-cloud
```

## Quick Example

```rust,no_run
use redis_cloud::{CloudClient, CloudSubscriptionHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = CloudClient::builder()
        .api_key(std::env::var("REDIS_CLOUD_API_KEY")?)
        .api_secret(std::env::var("REDIS_CLOUD_API_SECRET")?)
        .build()?;

    let subs = CloudSubscriptionHandler::new(client).list().await?;
    println!("Found {} subscriptions", subs.len());
    Ok(())
}
```

See the rustdoc for all handlers, models, and configuration options.

