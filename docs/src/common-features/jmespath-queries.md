# JMESPath Queries

JMESPath is a query language for JSON that allows you to extract and transform data from command output. redisctl supports JMESPath queries via the `-q` or `--query` flag.

## Basic Usage

```bash
redisctl [command] -o json -q "query_expression"
```

## Quick Start Examples

```bash
# Get just one field
redisctl enterprise cluster get -o json -q 'name'
# Output: "docker-cluster"

# Get multiple fields as object
redisctl enterprise database get 1 -o json -q '{name: name, port: port}'
# Output: {"name": "default-db", "port": 12000}

# Get field from all items in a list
redisctl enterprise database list -o json -q '[].name'
# Output: ["default-db", "cache-db", "persistent-db"]

# Filter list by condition
redisctl enterprise database list -o json -q "[?port > `12000`].name"
# Output: ["cache-db", "persistent-db"]

# Count items
redisctl enterprise database list -o json -q 'length(@)'
# Output: 3
```

## Common Query Patterns

### Select Specific Fields

```bash
# Get just database names and ports
redisctl cloud database list -o json -q "[].{name:name, port:port}"

# Output:
# [
#   {"name": "cache-prod", "port": 12000},
#   {"name": "sessions", "port": 12001}
# ]
```

### Filter Results

```bash
# Find active databases only
redisctl cloud database list -o json -q "[?status=='active']"

# Databases with specific memory size
redisctl enterprise database list -o json -q "[?memory_size > `1073741824`]"

# Multiple conditions
redisctl cloud subscription list -o json \
  -q "[?status=='active' && paymentMethodId=='12345']"
```

### Array Operations

```bash
# First 3 results
redisctl cloud database list -o json -q "[0:3]"

# Last result
redisctl cloud database list -o json -q "[-1]"

# Count results
redisctl cloud database list -o json -q "length(@)"
```

### Nested Data Access

```bash
# Access nested fields
redisctl cloud subscription get 123456 -o json \
  -q "databases[].{id:databaseId, name:name}"

# Flatten nested arrays
redisctl enterprise cluster get -o json \
  -q "nodes[].{node:name, shards:shards[].name}"
```

## Advanced Queries

### Sorting

```bash
# Sort by memory size descending
redisctl enterprise database list -o json \
  -q "reverse(sort_by(@, &memory_size))"

# Sort by name
redisctl cloud database list -o json \
  -q "sort_by(@, &name)"
```

### Aggregations

```bash
# Sum total memory across databases
redisctl enterprise database list -o json \
  -q "sum([].memory_size)"

# Get max port number
redisctl enterprise database list -o json \
  -q "max([].port)"
```

### Complex Transformations

```bash
# Group databases by status
redisctl cloud database list -o json \
  -q "group_by(@, &status)"

# Multi-level filtering and projection
redisctl cloud subscription list -o json \
  -q "[?databases[?status=='active']].{
    subscription: name,
    active_databases: databases[?status=='active'].name
  }"
```

## Enterprise-Specific Examples

### Database Management

```bash
# Get all database names and their persistence settings
redisctl enterprise database list -o json \
  -q '[].{name: name, persistence: data_persistence}'

# Find databases using AOF persistence
redisctl enterprise database list -o json \
  -q "[?data_persistence=='aof'].name"

# Get database endpoints for connection strings
redisctl enterprise database get 1 -o json \
  -q 'endpoints[0].{host: addr[0], port: port}'

# Monitor database creation status
redisctl enterprise database list -o json \
  -q "[?status!='active'].{name: name, status: status}"
```

### Node and Cluster Monitoring

```bash
# Get node addresses with their status
redisctl enterprise node list -o json \
  -q '[].{address: addr, status: status, shards: shard_count}'

# Extract specific node details
redisctl enterprise node get 1 -o json \
  -q '{address: addr, cores: cores, memory_gb: total_memory / `1073741824`}'

# Check cluster resource usage
redisctl enterprise cluster stats -o json \
  -q '{cpu: cpu_usage, memory: memory_usage, databases: total_databases}'

# Get cluster version and license status
redisctl enterprise cluster get -o json \
  -q '{name: name, version: software_version, licensed: !license_expired}'
```

### Module Management

```bash
# List all module names and versions
redisctl enterprise module list -o json \
  -q '[].{name: module_name, version: semantic_version}'

# Find specific module version
redisctl enterprise module list -o json \
  -q "[?module_name=='search'].semantic_version | [0]"

# Get modules configured for a database
redisctl enterprise database get 1 -o json \
  -q 'module_list[].{name: module_name, args: module_args}'
```

### License and Compliance

```bash
# Check license expiration
redisctl enterprise license get -o json \
  -q '{expired: expired, expires_on: expiration_date}'

# Count total shards across all databases
redisctl enterprise database list -o json \
  -q 'sum([].shards_count)'
```

### Alert Monitoring

```bash
# Count active alerts
redisctl api enterprise get /v1/cluster/alerts -o json \
  -q 'length(@)'

# Get alert details if any exist
redisctl api enterprise get /v1/cluster/alerts -o json \
  -q '[].{severity: severity, message: message}'
```

## Cloud-Specific Examples

### Find Resources by Tags

```bash
# Find subscriptions with specific tags
redisctl cloud subscription list -o json \
  -q "[?tags.environment=='production']"
```

### Monitor Resource Usage

```bash
# Get high memory usage databases
redisctl enterprise database stats all -o json \
  -q "[?used_memory > `858993459`].{
    name: name,
    usage_percent: (used_memory / memory_size) * `100`
  }"
```

### Extract Connection Info

```bash
# Get connection strings for all databases
redisctl cloud database list -o json \
  -q "[].{
    name: name,
    connection: join('', ['redis://', publicEndpoint, ':', to_string(port)])
  }"
```

### Audit Configuration

```bash
# Find databases without replication
redisctl enterprise database list -o json \
  -q "[?replication == `false`].name"

# Check backup settings
redisctl enterprise database list -o json \
  -q "[?backup_interval == `0`].{
    name: name,
    warning: 'No automatic backups configured'
  }"
```

## Query Testing Tips

### Test with Sample Data

```bash
# Save output to test queries
redisctl cloud database list -o json > databases.json

# Test queries offline
cat databases.json | jq '.' | jmespath.py "[?status=='active'].name"
```

### Debug Complex Queries

```bash
# Build queries incrementally
redisctl cloud subscription list -o json -q "@"           # All data
redisctl cloud subscription list -o json -q "[0]"         # First item
redisctl cloud subscription list -o json -q "[0].databases" # Databases of first
```

### Common Gotchas

1. **String literals need quotes**: `[?status=='active']` not `[?status==active]`
2. **Numbers use backticks**: `[?port > \`12000\`]` not `[?port > 12000]`
3. **Escape in shell**: Use single quotes around queries to avoid shell interpretation
4. **Null handling**: Use `[?field != null]` to filter out null values

## Performance Considerations

- JMESPath queries are applied client-side after receiving the full response
- For large result sets, consider using API pagination parameters first
- Complex queries may impact performance on very large JSON responses

## Reference

For complete JMESPath syntax, see:
- [JMESPath Specification](https://jmespath.org/)
- [JMESPath Tutorial](https://jmespath.org/tutorial.html)
- [JMESPath Examples](https://jmespath.org/examples.html)