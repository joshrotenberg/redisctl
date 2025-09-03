# Redis Enterprise Rust Library

The `redis-enterprise` crate provides a comprehensive, typed Rust client for Redis Enterprise clusters (on-prem or self-managed). It covers cluster management, databases (BDB), nodes, users/roles, modules, CRDB, stats, and logs.

- Crate: `redis-enterprise`
- Rustdoc: https://docs.rs/redis-enterprise/latest/redis_enterprise/

## Install

```toml
[dependencies]
redis-enterprise = "0.2"
tokio = { version = "1", features = ["full"] }
```

Or:

```bash
cargo add redis-enterprise
```

## Quick Example

```rust,no_run
use redis_enterprise::EnterpriseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = EnterpriseClient::builder()
        .base_url(std::env::var("REDIS_ENTERPRISE_URL")?)
        .username(std::env::var("REDIS_ENTERPRISE_USER")?)
        .password(std::env::var("REDIS_ENTERPRISE_PASSWORD")?)
        .build()?;

    let dbs = client.databases().list().await?;
    println!("Found {} databases", dbs.len());
    Ok(())
}
```

See the rustdoc for handlers, models, and advanced operations.

