# Python Bindings

Use the `redisctl` Rust libraries from Python via PyO3 bindings.

!!! info "New in v0.8"
    Python bindings are available starting with redisctl v0.8.0.

## Installation

```bash
pip install redisctl
```

Or with development dependencies:

```bash
pip install redisctl[dev]
```

## Quick Start

=== "Redis Cloud"

    ```python
    from redisctl import CloudClient

    # Create client with API credentials
    client = CloudClient(
        api_key="your-api-key",
        api_secret="your-api-secret"
    )

    # Or use environment variables
    client = CloudClient.from_env()

    # List subscriptions
    result = client.subscriptions_sync()
    for sub in result.get('subscriptions', []):
        print(f"{sub['id']}: {sub['name']} ({sub['status']})")

    # List databases in a subscription
    dbs = client.databases_sync(subscription_id=12345)
    ```

=== "Redis Enterprise"

    ```python
    from redisctl import EnterpriseClient

    # Create client
    client = EnterpriseClient(
        base_url="https://cluster:9443",
        username="admin@redis.local",
        password="secret",
        insecure=True  # For self-signed certs
    )

    # Or use environment variables
    client = EnterpriseClient.from_env()

    # Get cluster info
    cluster = client.cluster_info_sync()
    print(f"Cluster: {cluster['name']}")

    # List databases
    for db in client.databases_sync():
        print(f"{db['uid']}: {db['name']} ({db['status']})")
    ```

## Async Support

All methods have both sync and async versions. Async methods have no suffix, while sync methods end with `_sync`:

```python
import asyncio
from redisctl import CloudClient

async def main():
    client = CloudClient.from_env()
    
    # Async methods (no _sync suffix)
    result = await client.subscriptions()
    print(result)
    
    # Get a specific subscription
    sub = await client.subscription(subscription_id=12345)
    
    # List databases
    dbs = await client.databases(subscription_id=12345)

asyncio.run(main())
```

## CloudClient API

### Constructor

```python
CloudClient(
    api_key: str,
    api_secret: str,
    base_url: str | None = None,      # Default: https://api.redislabs.com/v1
    timeout_secs: int | None = None   # Default: 30
)
```

### Factory Methods

```python
# Create from environment variables
client = CloudClient.from_env()
```

### Subscriptions

| Method | Description |
|--------|-------------|
| `subscriptions()` / `subscriptions_sync()` | List all subscriptions |
| `subscription(id)` / `subscription_sync(id)` | Get subscription by ID |

### Databases

| Method | Description |
|--------|-------------|
| `databases(subscription_id, offset?, limit?)` | List databases in subscription |
| `database(subscription_id, database_id)` | Get specific database |

### Raw API Access

| Method | Description |
|--------|-------------|
| `get(path)` / `get_sync(path)` | Execute GET request |
| `post(path, body)` / `post_sync(path, body)` | Execute POST request |
| `put(path, body)` / `put_sync(path, body)` | Execute PUT request |
| `delete(path)` / `delete_sync(path)` | Execute DELETE request |

## EnterpriseClient API

### Constructor

```python
EnterpriseClient(
    base_url: str,                     # e.g., "https://cluster:9443"
    username: str,
    password: str,
    insecure: bool = False,            # Allow self-signed certs
    timeout_secs: int | None = None    # Default: 30
)
```

### Factory Methods

```python
# Create from environment variables
client = EnterpriseClient.from_env()
```

### Cluster

| Method | Description |
|--------|-------------|
| `cluster_info()` / `cluster_info_sync()` | Get cluster information |
| `cluster_stats()` / `cluster_stats_sync()` | Get cluster statistics |
| `license()` / `license_sync()` | Get license information |

### Databases

| Method | Description |
|--------|-------------|
| `databases()` / `databases_sync()` | List all databases |
| `database(uid)` / `database_sync(uid)` | Get database by UID |
| `database_stats(uid)` / `database_stats_sync(uid)` | Get database statistics |

### Nodes

| Method | Description |
|--------|-------------|
| `nodes()` / `nodes_sync()` | List all nodes |
| `node(uid)` / `node_sync(uid)` | Get node by UID |
| `node_stats(uid)` / `node_stats_sync(uid)` | Get node statistics |

### Users

| Method | Description |
|--------|-------------|
| `users()` / `users_sync()` | List all users |
| `user(uid)` / `user_sync(uid)` | Get user by UID |

### Raw API Access

| Method | Description |
|--------|-------------|
| `get(path)` / `get_sync(path)` | Execute GET request |
| `post(path, body)` / `post_sync(path, body)` | Execute POST request |
| `put(path, body)` / `put_sync(path, body)` | Execute PUT request |
| `delete(path)` / `delete_sync(path)` | Execute DELETE request |

## Environment Variables

### Redis Cloud

| Variable | Description |
|----------|-------------|
| `REDIS_CLOUD_API_KEY` | API key (also: `REDIS_CLOUD_ACCOUNT_KEY`) |
| `REDIS_CLOUD_API_SECRET` | API secret (also: `REDIS_CLOUD_SECRET_KEY`, `REDIS_CLOUD_USER_KEY`) |
| `REDIS_CLOUD_BASE_URL` | Base URL (optional, also: `REDIS_CLOUD_API_URL`) |

### Redis Enterprise

| Variable | Description |
|----------|-------------|
| `REDIS_ENTERPRISE_URL` | Cluster URL (e.g., `https://cluster:9443`) |
| `REDIS_ENTERPRISE_USER` | Username |
| `REDIS_ENTERPRISE_PASSWORD` | Password |
| `REDIS_ENTERPRISE_INSECURE` | Set to `true` to skip TLS verification |

## Error Handling

All errors are raised as `RedisCtlError` exceptions:

```python
from redisctl import CloudClient, RedisCtlError

client = CloudClient.from_env()

try:
    sub = client.subscription_sync(subscription_id=99999)
except RedisCtlError as e:
    print(f"API error: {e}")
```

## Examples

### List All Databases Across Subscriptions

```python
from redisctl import CloudClient

client = CloudClient.from_env()

# Get all subscriptions
subs = client.subscriptions_sync()

for sub in subs.get('subscriptions', []):
    sub_id = sub['id']
    print(f"\nSubscription {sub_id}: {sub['name']}")
    
    # Get databases for this subscription
    dbs = client.databases_sync(subscription_id=sub_id)
    for db in dbs.get('subscription', [{}])[0].get('databases', []):
        print(f"  - {db['name']} ({db['status']})")
```

### Monitor Cluster Health

```python
from redisctl import EnterpriseClient

client = EnterpriseClient.from_env()

# Get cluster info
cluster = client.cluster_info_sync()
print(f"Cluster: {cluster['name']}")
print(f"Version: {cluster.get('software_version', 'unknown')}")

# Check nodes
nodes = client.nodes_sync()
for node in nodes:
    status = "OK" if node.get('status') == 'active' else "WARN"
    print(f"Node {node['uid']}: {status}")

# Check databases
dbs = client.databases_sync()
for db in dbs:
    status = "OK" if db.get('status') == 'active' else "WARN"
    print(f"Database {db['name']}: {status}")
```

### Export Configuration

```python
import json
from redisctl import EnterpriseClient

client = EnterpriseClient.from_env()

# Gather cluster configuration
config = {
    "cluster": client.cluster_info_sync(),
    "databases": client.databases_sync(),
    "nodes": client.nodes_sync(),
    "users": client.users_sync(),
}

# Save to file
with open("cluster-config.json", "w") as f:
    json.dump(config, f, indent=2)

print("Configuration exported to cluster-config.json")
```

### Raw API for Unsupported Endpoints

```python
from redisctl import EnterpriseClient

client = EnterpriseClient.from_env()

# Get cluster policy (not a typed method)
policy = client.get_sync("/v1/cluster/policy")
print(f"Persistent storage: {policy.get('persistent_node_storage')}")

# Update cluster settings
client.put_sync("/v1/cluster", {
    "name": "production-cluster"
})
```

## Platform Support

The Python bindings are available as pre-built wheels for:

- **macOS**: Intel (x86_64) and Apple Silicon (arm64)
- **Linux**: x86_64 and aarch64 (glibc)
- **Windows**: x86_64

Python versions: 3.9, 3.10, 3.11, 3.12, 3.13

## Building from Source

If pre-built wheels aren't available for your platform:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install maturin
pip install maturin

# Clone and build
git clone https://github.com/redis-developer/redisctl.git
cd redisctl/crates/redisctl-python
maturin build --release
pip install target/wheels/redisctl-*.whl
```

## Links

- [PyPI Package](https://pypi.org/project/redisctl/)
- [GitHub Repository](https://github.com/redis-developer/redisctl)
- [Issue Tracker](https://github.com/redis-developer/redisctl/issues)
