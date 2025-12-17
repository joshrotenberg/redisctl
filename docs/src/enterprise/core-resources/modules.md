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
# Get details for a specific module by UID
redisctl enterprise module get <module_uid>

# Get module by name (case-insensitive)
redisctl enterprise module get --name search
redisctl enterprise module get --name ReJSON

# Get specific fields
redisctl enterprise module get --name search -o json -q '{name: module_name, version: semantic_version}'
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

## JMESPath Query Examples

```bash
# List all module names and versions
redisctl enterprise module list -q '[].{name: module_name, version: version}'

# Find a specific module
redisctl enterprise module list -q "[?module_name=='search']"
```

## Enabling Modules on Databases

To enable modules when creating a database:

```bash
# Using the --module flag (recommended)
redisctl enterprise database create --name my-search-db --memory 1073741824 \
  --module search --module ReJSON

# Using JSON data
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

## Custom Module Development (RE8+)

Redis Enterprise 8.x uses a new native module packaging format. The following tools help with custom module development and packaging.

### Validate Module Metadata

Validate a `module.json` file against the Redis Enterprise schema before packaging:

```bash
# Basic validation
redisctl enterprise module validate ./module.json

# Example output:
# Validating: ./module.json
#
#   v module_name: jmespath
#   v version: 300
#   v semantic_version: 0.3.0
#   v min_redis_version: 7.0.0
#   v compatible_redis_version: 8.0.0
#   v commands: 33 commands defined
#   v capabilities: 7 capabilities
#
# v Module metadata is valid for Redis Enterprise 8.x

# Strict mode (require all recommended fields)
redisctl enterprise module validate ./module.json --strict
```

**Validation checks:**
- Required fields: `module_name`
- Recommended fields: `version`, `semantic_version`, `min_redis_version`
- Important for RE8: `compatible_redis_version` (required for upgrade tests)
- Commands and capabilities definitions

### Inspect Module Package

Inspect a packaged module zip file to verify its structure and contents:

```bash
# Basic inspection
redisctl enterprise module inspect ./redis-jmespath.Linux-x86_64.0.3.0.zip

# Example output:
# Package: redis-jmespath.Linux-x86_64.0.3.0.zip
#
# Files:
#   module.json (6.2 KB)
#   jmespath.so (6.5 MB)
#
# Metadata:
#   Name: jmespath
#   Display: JMESPath
#   Version: 0.3.0 (300)
#   Min Redis: 7.0.0
#   Compatible: 8.0.0
#   Commands: 33
#   Capabilities: types, replica_of, backup_restore, ...
#
# v Package structure is valid for RE8 user_defined_modules

# Show all commands
redisctl enterprise module inspect ./module.zip --full
```

**Structure validation:**
- Files must be at zip root (no subdirectories)
- Must contain `module.json`
- Must contain `.so` module binary

### Package Module

Create an RE8-compatible module zip package:

```bash
# Basic packaging
redisctl enterprise module package \
  --module ./libredis_jmespath.so \
  --metadata ./module.json \
  --out ./dist/redis-jmespath.Linux-x86_64.0.3.0.zip

# Package with validation
redisctl enterprise module package \
  --module ./module.so \
  --metadata ./module.json \
  --out ./package.zip \
  --validate
```

### module.json Format

The `module.json` file describes your module for Redis Enterprise:

```json
{
  "module_name": "jmespath",
  "display_name": "JMESPath",
  "version": 300,
  "semantic_version": "0.3.0",
  "min_redis_version": "7.0.0",
  "compatible_redis_version": "8.0.0",
  "author": "Your Name",
  "description": "JMESPath query support for Redis",
  "license": "MIT",
  "command_line_args": "",
  "capabilities": [
    "types",
    "replica_of",
    "clustering",
    "backup_restore"
  ],
  "commands": [
    {
      "command_name": "JMESPATH.QUERY",
      "command_arity": -3,
      "first_key": 1,
      "last_key": 1,
      "step": 1,
      "flags": ["readonly"]
    }
  ]
}
```

**Key fields:**
- `version`: Numeric version (e.g., 300 for 0.3.0)
- `semantic_version`: Human-readable version string
- `compatible_redis_version`: Maximum Redis version tested (important for RE8 upgrades)
- `commands`: Full command metadata including arity, key positions, and flags

### Deploying Custom Modules

Custom modules can be deployed via:

1. **Bootstrap** - Using `user_defined_modules` in cluster init:
   ```bash
   redisctl enterprise workflow init-cluster \
     --name my-cluster \
     --username admin@example.com \
     --password secret \
     --data '{
       "user_defined_modules": [
         {"url": "https://host/redis-jmespath.zip"}
       ]
     }'
   ```

2. **Admin UI** - Upload via Settings > Redis Modules

3. **Kubernetes** - Using `userDefinedModules` in RedisEnterpriseCluster spec
