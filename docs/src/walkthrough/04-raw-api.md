# 4. Layer 1: Raw API Access

**Direct REST calls to any endpoint**

## Why Raw API Layer?

Access endpoints not yet wrapped, test exact API behavior, or get maximum flexibility.

## Cloud Examples

```bash
# GET request
redisctl api cloud get /subscriptions

# GET with path parameters
redisctl api cloud get /subscriptions/12345/databases

# POST request with data
redisctl api cloud post /subscriptions \
  --data '{"name": "new-sub", "cloudProviders": [...]}'
```

## Enterprise Examples

```bash
# Cluster info
redisctl api enterprise get /v1/cluster

# List databases
redisctl api enterprise get /v1/bdbs

# Get cluster policy (as shown in docker-compose)
redisctl api enterprise get /v1/cluster/policy \
  -o json -q '{default_db: default_non_sharded_proxy_policy, rack_aware: rack_aware}'

# Get alerts count
redisctl api enterprise get /v1/cluster/alerts -o json -q 'length(@)'
```

## With JMESPath Queries

```bash
# Extract cluster name
redisctl api enterprise get /v1/cluster -q 'name'

# Filter active databases
redisctl api enterprise get /v1/bdbs \
  -q "[?status=='active'].{name:name,memory:memory_size}"
```

## With Different Output Formats

```bash
# JSON (default)
redisctl api cloud get /subscriptions -o json

# YAML
redisctl api cloud get /subscriptions -o yaml

# Table (auto-formatted)
redisctl api cloud get /subscriptions -o table
```

## Key Features

- **Any HTTP method**: GET, POST, PUT, PATCH, DELETE
- **Request body**: `--data` flag for JSON payloads
- **Output filtering**: JMESPath queries via `-q`
- **Format control**: JSON, YAML, or table output

---

**Previous:** [3. Installation & Setup](./03-setup.md)  
**Next:** [5. Human-Friendly Commands](./05-human-friendly.md)
