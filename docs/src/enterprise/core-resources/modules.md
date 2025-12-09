# Module Management

Manage Redis modules available on your Redis Enterprise cluster.

## Commands Overview

```bash
redisctl enterprise module --help
```

## List Available Modules

```bash
# List all modules installed on the cluster
redisctl enterprise module list

# Or use the alias
redisctl enterprise module ls

# Output as JSON
redisctl enterprise module list -o json
```

## Get Module Details

```bash
# Get details for a specific module
redisctl enterprise module get <module_uid>

# Get specific fields
redisctl enterprise module get <module_uid> -o json | jq '{name, version, capabilities}'
```

## Upload Module

Upload a new module to the cluster:

```bash
# Upload a module file
redisctl enterprise module upload --file /path/to/module.so

# Upload with metadata
redisctl enterprise module upload --file /path/to/module.so --data '{
  "module_name": "custom-module"
}'
```

## Delete Module

```bash
# Delete a module from the cluster
redisctl enterprise module delete <module_uid>

# Or use the alias
redisctl enterprise module rm <module_uid>
```

## Configure Module for Database

```bash
# Configure module settings for a specific database
redisctl enterprise module config-bdb <module_uid> <db_id> --data '{
  "config": "value"
}'
```

## Common Modules

Redis Enterprise supports these modules:

| Module | Description |
|--------|-------------|
| `search` | RediSearch - Full-text search and secondary indexing |
| `json` | RedisJSON - Native JSON data type |
| `timeseries` | RedisTimeSeries - Time series data structure |
| `bloom` | RedisBloom - Probabilistic data structures |
| `graph` | RedisGraph - Graph database (deprecated) |
| `ai` | RedisAI - Machine learning model serving |

## JSON Output Examples

```bash
# List all module names and versions
redisctl enterprise module list -o json | jq '.[] | {name: .module_name, version}'

# Find a specific module
redisctl enterprise module list -o json | jq '.[] | select(.module_name == "search")'
```

## Enabling Modules on Databases

To enable modules when creating a database:

```bash
redisctl enterprise database create --data '{
  "name": "my-search-db",
  "memory_size": 1073741824,
  "module_list": [
    {"module_name": "search"},
    {"module_name": "json"}
  ]
}'
```

To add modules to an existing database:

```bash
redisctl enterprise database update-modules <db_id> --data '{
  "module_list": [
    {"module_name": "search"},
    {"module_name": "json"}
  ]
}'
```
