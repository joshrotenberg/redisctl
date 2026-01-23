# Developer Guide

Build with redisctl and contribute to the project.

## Using redisctl Libraries

redisctl is built as reusable libraries available for both Rust and Python:

### Rust Crates

| Crate | Description | docs.rs |
|-------|-------------|---------|
| `redis-cloud` | Redis Cloud API client | [docs](https://docs.rs/redis-cloud) |
| `redis-enterprise` | Redis Enterprise API client | [docs](https://docs.rs/redis-enterprise) |
| `redisctl-config` | Profile and credential management | [docs](https://docs.rs/redisctl-config) |

### Python Package

| Package | Description | PyPI |
|---------|-------------|------|
| `redisctl` | Python bindings for Redis Cloud & Enterprise | [pypi](https://pypi.org/project/redisctl/) |

### Example: Using redis-cloud

```rust
use redis_cloud::RedisCloudClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = RedisCloudClient::new(
        "your-api-key",
        "your-secret-key",
    );

    let subscriptions = client.subscriptions().list().await?;
    for sub in subscriptions {
        println!("{}: {}", sub.id, sub.name);
    }

    Ok(())
}
```

[:octicons-arrow-right-24: Rust Libraries Guide](libraries.md)

### Example: Using Python

```python
from redisctl import CloudClient

client = CloudClient.from_env()

subscriptions = client.subscriptions_sync()
for sub in subscriptions.get('subscriptions', []):
    print(f"{sub['id']}: {sub['name']}")
```

[:octicons-arrow-right-24: Python Bindings Guide](python.md)

## Architecture

Understand how redisctl is structured:

- Four-layer design (Profiles, Raw API, Human Commands, Workflows)
- Workspace organization
- Error handling patterns
- Output formatting

[:octicons-arrow-right-24: Architecture](architecture.md)

## Contributing

We welcome contributions:

- Bug reports and feature requests
- Documentation improvements
- Code contributions

[:octicons-arrow-right-24: Contributing Guide](contributing.md)

## Links

- [GitHub Repository](https://github.com/redis-developer/redisctl)
- [Issue Tracker](https://github.com/redis-developer/redisctl/issues)
- [Releases](https://github.com/redis-developer/redisctl/releases)
