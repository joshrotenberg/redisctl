# Libraries

Use redisctl components in your own Rust projects.

## Available Crates

| Crate | Description | docs.rs |
|-------|-------------|---------|
| `redis-cloud` | Redis Cloud API client | [docs](https://docs.rs/redis-cloud) |
| `redis-enterprise` | Redis Enterprise API client | [docs](https://docs.rs/redis-enterprise) |
| `redisctl-config` | Profile and credential management | [docs](https://docs.rs/redisctl-config) |

## redis-cloud

Redis Cloud API client with full type coverage.

### Installation

```toml
[dependencies]
redis-cloud = "0.7"
tokio = { version = "1", features = ["full"] }
```

### Example

```rust
use redis_cloud::RedisCloudClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RedisCloudClient::new(
        std::env::var("REDIS_CLOUD_API_KEY")?,
        std::env::var("REDIS_CLOUD_SECRET_KEY")?,
    );

    // List subscriptions
    let subscriptions = client.subscriptions().list().await?;
    for sub in subscriptions {
        println!("{}: {}", sub.id, sub.name);
    }

    // Get databases
    let databases = client.databases().list(subscription_id).await?;

    Ok(())
}
```

## redis-enterprise

Redis Enterprise REST API client.

### Installation

```toml
[dependencies]
redis-enterprise = "0.7"
tokio = { version = "1", features = ["full"] }
```

### Example

```rust
use redis_enterprise::RedisEnterpriseClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RedisEnterpriseClient::builder()
        .url("https://cluster.example.com:9443")
        .username("admin@cluster.local")
        .password("password")
        .insecure(true)  // For self-signed certs
        .build()?;

    // Get cluster info
    let cluster = client.cluster().get().await?;
    println!("Cluster: {} ({})", cluster.name, cluster.version);

    // List databases
    let databases = client.databases().list().await?;
    for db in databases {
        println!("  {}: {}", db.uid, db.name);
    }

    Ok(())
}
```

## redisctl-config

Profile and credential management.

### Installation

```toml
[dependencies]
redisctl-config = "0.1"
```

### Example

```rust
use redisctl_config::{Config, Profile};

fn main() -> anyhow::Result<()> {
    // Load config
    let config = Config::load()?;

    // Get default profile
    let profile = config.default_profile()?;

    // Access credentials
    if let Some(key) = profile.cloud_api_key() {
        println!("Cloud API key: {}...", &key[..8]);
    }

    Ok(())
}
```

## Use Cases

### Custom Backup Tool

```rust
use redis_enterprise::RedisEnterpriseClient;
use std::fs::File;

async fn backup_config(client: &RedisEnterpriseClient) -> anyhow::Result<()> {
    let cluster = client.cluster().get().await?;
    let databases = client.databases().list().await?;

    let backup = serde_json::json!({
        "cluster": cluster,
        "databases": databases,
        "timestamp": chrono::Utc::now(),
    });

    let file = File::create("backup.json")?;
    serde_json::to_writer_pretty(file, &backup)?;

    Ok(())
}
```

### Monitoring Integration

```rust
use redis_enterprise::RedisEnterpriseClient;

async fn collect_metrics(client: &RedisEnterpriseClient) -> anyhow::Result<()> {
    let nodes = client.nodes().list().await?;

    for node in nodes {
        prometheus::gauge!("redis_node_shards", node.shard_count as f64,
            "node" => node.uid.to_string());
    }

    Ok(())
}
```

## API Coverage

### redis-cloud

- Subscriptions (CRUD)
- Databases (CRUD)
- Tasks (list, get, wait)
- VPC Peering
- Users and ACLs
- And more...

### redis-enterprise

- Cluster (get, update, stats)
- Databases (CRUD, stats)
- Nodes (list, get, stats)
- Users and Roles
- LDAP configuration
- Support packages
- And more...

## Links

- [GitHub](https://github.com/redis-developer/redisctl)
- [docs.rs/redis-cloud](https://docs.rs/redis-cloud)
- [docs.rs/redis-enterprise](https://docs.rs/redis-enterprise)
- [crates.io/crates/redisctl](https://crates.io/crates/redisctl)
