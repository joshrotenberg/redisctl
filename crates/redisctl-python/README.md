# redisctl Python Bindings

Python bindings for Redis Cloud and Enterprise management APIs, built with PyO3.

## Installation

```bash
pip install redisctl
```

## Quick Start

### Redis Cloud

```python
from redisctl import CloudClient

# Create client with API credentials
client = CloudClient(
    api_key="your-api-key",
    api_secret="your-api-secret"
)

# Or use environment variables
# REDIS_CLOUD_API_KEY, REDIS_CLOUD_API_SECRET
client = CloudClient.from_env()

# List subscriptions (sync)
result = client.subscriptions_sync()
for sub in result.get('subscriptions', []):
    print(f"{sub['id']}: {sub['name']} ({sub['status']})")

# List databases in a subscription
dbs = client.databases_sync(subscription_id=12345)
```

### Redis Enterprise

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
# REDIS_ENTERPRISE_URL, REDIS_ENTERPRISE_USER, REDIS_ENTERPRISE_PASSWORD
client = EnterpriseClient.from_env()

# Get cluster info
cluster = client.cluster_info_sync()
print(f"Cluster: {cluster['name']}")

# List databases
for db in client.databases_sync():
    print(f"{db['uid']}: {db['name']} ({db['status']})")

# List nodes
for node in client.nodes_sync():
    print(f"Node {node['uid']}: {node['addr']}")
```

## Async Support

All methods have both sync and async versions:

```python
import asyncio
from redisctl import CloudClient

async def main():
    client = CloudClient.from_env()
    
    # Async methods (no _sync suffix)
    result = await client.subscriptions()
    print(result)

asyncio.run(main())
```

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

## Raw API Access

For API endpoints not covered by the typed methods:

```python
# GET request
result = client.get_sync("/v1/cluster/policy")

# POST request
client.post_sync("/v1/bdbs", {"name": "my-db", "memory_size": 104857600})

# PUT request
client.put_sync("/v1/bdbs/1", {"memory_size": 209715200})

# DELETE request
client.delete_sync("/v1/bdbs/1")
```

## Type Hints

Full type hints are provided via `.pyi` stub files for IDE support.

## License

MIT OR Apache-2.0
