# 4. Layer 1: Raw API Access

**Direct REST calls to any endpoint**

## Why Raw API Layer?

When you need:
- Access to endpoints not yet wrapped
- Exact API behavior for testing
- Maximum flexibility

## Cloud Examples

```bash
# GET request
redisctl api cloud get /subscriptions

# GET with path parameters
redisctl api cloud get /subscriptions/12345/databases

# POST request with data
redisctl api cloud post /subscriptions \
  --data '{"name": "new-sub", "cloudProviders": [...]}'

# DELETE request
redisctl api cloud delete /subscriptions/12345
```

## Enterprise Examples

```bash
# Cluster info
redisctl api enterprise get /v1/cluster

# List databases
redisctl api enterprise get /v1/bdbs

# Get specific database
redisctl api enterprise get /v1/bdbs/1

# Create database
redisctl api enterprise post /v1/bdbs \
  --data '{"name": "mydb", "memory_size": 1073741824}'
```

## With JMESPath Queries

```bash
# Extract just the name field
redisctl api enterprise get /v1/cluster -q 'name'

# Filter and reshape
redisctl api enterprise get /v1/bdbs \
  -q '[?status==`active`].{name:name,memory:memory_size}'
```

## With Different Output Formats

```bash
# JSON (default)
redisctl api enterprise get /v1/bdbs

# YAML
redisctl api enterprise get /v1/bdbs -o yaml

# Table (limited for raw API)
redisctl api enterprise get /v1/cluster -o table
```

## HTTP Methods Supported

- `GET` - Retrieve resources
- `POST` - Create resources
- `PUT` - Update resources (full replacement)
- `PATCH` - Update resources (partial)
- `DELETE` - Remove resources

## When to Use

✅ Testing new API endpoints  
✅ Debugging API behavior  
✅ Endpoints not in human-friendly layer yet  
✅ Need exact API response

---

**← Previous:** [3. Installation & Setup](./03-setup.md)  
**Next →** [5. Human-Friendly Layer](./05-human-friendly.md)

**Layer Stack:** **Raw API** → Human-Friendly → Workflows
