# redisctl - Python Bindings

Python bindings for Redis Cloud and Enterprise management APIs, built with [PyO3](https://pyo3.rs/).

## Installation

```bash
pip install redisctl
```

Or build from source:

```bash
# Install maturin
pip install maturin

# Build and install
cd crates/redisctl-python
maturin develop
```

## Quick Start

### Redis Cloud

```python
from redisctl import CloudClient
import asyncio

# Create client
client = CloudClient(
    api_key="your-api-key",
    api_secret="your-api-secret"
)

# Async usage (recommended for concurrent operations)
async def main():
    # List subscriptions
    subs = await client.subscriptions()
    for sub in subs:
        print(f"Subscription: {sub['id']} - {sub['name']}")
    
    # List databases in a subscription
    dbs = await client.databases(subscription_id=12345)
    for db in dbs:
        print(f"Database: {db['databaseId']} - {db['name']}")

asyncio.run(main())

# Sync usage (simpler for scripts)
subs = client.subscriptions_sync()
print(f"Found {len(subs)} subscriptions")
```

### Redis Enterprise

```python
from redisctl import EnterpriseClient
import asyncio

# Create client
client = EnterpriseClient(
    base_url="https://cluster.example.com:9443",
    username="admin@redis.local",
    password="your-password",
    insecure=True  # For self-signed certificates
)

# Or from environment variables
# client = EnterpriseClient.from_env()

# Async usage
async def main():
    # Get cluster info
    info = await client.cluster_info()
    print(f"Cluster: {info['name']}")
    
    # List databases
    dbs = await client.databases()
    for db in dbs:
        print(f"Database: {db['uid']} - {db['name']}")
    
    # List nodes
    nodes = await client.nodes()
    for node in nodes:
        print(f"Node: {node['uid']} - {node['addr']}")

asyncio.run(main())

# Sync usage
dbs = client.databases_sync()
print(f"Found {len(dbs)} databases")
```

## API Reference

### CloudClient

| Method | Async | Sync | Description |
|--------|-------|------|-------------|
| `subscriptions()` | ✓ | `subscriptions_sync()` | List all subscriptions |
| `subscription(id)` | ✓ | `subscription_sync(id)` | Get subscription by ID |
| `databases(sub_id)` | ✓ | `databases_sync(sub_id)` | List databases in subscription |
| `database(sub_id, db_id)` | ✓ | `database_sync(sub_id, db_id)` | Get database by ID |
| `get(path)` | ✓ | `get_sync(path)` | Raw GET request |
| `post(path, body)` | ✓ | `post_sync(path, body)` | Raw POST request |
| `put(path, body)` | ✓ | `put_sync(path, body)` | Raw PUT request |
| `delete(path)` | ✓ | `delete_sync(path)` | Raw DELETE request |

### EnterpriseClient

| Method | Async | Sync | Description |
|--------|-------|------|-------------|
| `cluster_info()` | ✓ | `cluster_info_sync()` | Get cluster information |
| `cluster_stats()` | ✓ | `cluster_stats_sync()` | Get cluster statistics |
| `license()` | ✓ | `license_sync()` | Get license info |
| `databases()` | ✓ | `databases_sync()` | List all databases |
| `database(uid)` | ✓ | `database_sync(uid)` | Get database by UID |
| `database_stats(uid)` | ✓ | `database_stats_sync(uid)` | Get database statistics |
| `nodes()` | ✓ | `nodes_sync()` | List all nodes |
| `node(uid)` | ✓ | `node_sync(uid)` | Get node by UID |
| `node_stats(uid)` | ✓ | `node_stats_sync(uid)` | Get node statistics |
| `users()` | ✓ | `users_sync()` | List all users |
| `user(uid)` | ✓ | `user_sync(uid)` | Get user by UID |
| `get(path)` | ✓ | `get_sync(path)` | Raw GET request |
| `post(path, body)` | ✓ | `post_sync(path, body)` | Raw POST request |
| `put(path, body)` | ✓ | `put_sync(path, body)` | Raw PUT request |
| `delete(path)` | ✓ | `delete_sync(path)` | Raw DELETE request |

## Raw API Access

Both clients provide raw API methods for endpoints not covered by typed methods:

```python
# Cloud API
result = await client.get("/subscriptions/12345/databases")
result = await client.post("/subscriptions", {"name": "new-sub", ...})

# Enterprise API
result = await client.get("/v1/bdbs")
result = await client.post("/v1/bdbs", {"name": "new-db", "memory_size": 1073741824})
```

## Environment Variables

### Redis Cloud
- `REDIS_CLOUD_API_KEY`: API key
- `REDIS_CLOUD_API_SECRET`: API secret

### Redis Enterprise
- `REDIS_ENTERPRISE_URL`: Cluster URL (default: `https://localhost:9443`)
- `REDIS_ENTERPRISE_USER`: Username (default: `admin@redis.local`)
- `REDIS_ENTERPRISE_PASSWORD`: Password (required)
- `REDIS_ENTERPRISE_INSECURE`: Set to `true` for self-signed certs

## Error Handling

```python
from redisctl import CloudClient, RedisCtlError

client = CloudClient(api_key="...", api_secret="...")

try:
    sub = await client.subscription(99999)
except ValueError as e:
    print(f"Not found: {e}")
except RedisCtlError as e:
    print(f"API error: {e}")
except ConnectionError as e:
    print(f"Connection failed: {e}")
```

## Type Hints

Type stubs are provided for IDE support. After installation, your IDE should provide autocomplete and type checking.

## License

MIT OR Apache-2.0
