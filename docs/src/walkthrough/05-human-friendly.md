# 5. Layer 2: Human-Friendly Commands

**Type-safe wrappers with better UX**

## Why Human-Friendly Layer?

Better experience for day-to-day operations with type-safe parameters, automatic error handling, and progress indicators.

## Cloud Operations

```bash
# List subscriptions
redisctl cloud subscription list

# Get account info
redisctl cloud account get

# List databases
redisctl cloud database list

# Get database with filtering (as shown in demo)
redisctl cloud database get 2969240:13684622 \
  -o json -q '{endpoint: publicEndpoint, password: security.password}'
```

## Enterprise Operations

```bash
# Cluster information (as shown in docker-compose)
redisctl enterprise cluster get \
  -o json -q '{name: name, nodes: nodes_count, version: software_version}'

# List databases with filtering
redisctl enterprise database list \
  -o json -q '[].{uid: uid, name: name, port: port, status: status}'

# Create database (as shown in docker-compose)
redisctl enterprise database create \
  --data '{"name": "cache-db", "memory_size": 104857600, "port": 12001}' \
  -o json -q '{uid: uid, name: name, port: port, status: status}'

# List nodes
redisctl enterprise node list \
  -o json -q '[].{id: uid, address: addr, status: status}'

# List users
redisctl enterprise user list \
  -o json -q '[].{uid: uid, name: name, email: email, role: role}'
```

## Output Formats

**JSON** (default, scriptable)
```bash
redisctl enterprise database list
```

**Table** (human-readable)
```bash
redisctl enterprise database list -o table
```

**YAML** (config files)
```bash
redisctl enterprise database get 1 -o yaml
```

## Automatic Polling with --wait

The `--wait` flag automatically polls until operations complete:

```bash
# Create and wait for completion
redisctl cloud database create \
  --subscription 12345 \
  --data '{"name": "mydb", "memoryLimitInGb": 1}' \
  --wait

# Shows progress spinner and completes when done
```

## JMESPath Filtering

Extract exactly what you need:

```bash
# Get just the cluster name
redisctl enterprise cluster get -q 'name'

# Reshape output
redisctl enterprise node get 1 \
  -q '{address: addr, cores: cores, memory: total_memory}'
```

---

**Previous:** [4. Raw API Access](./04-raw-api.md)  
**Next:** [6. Workflows](./06-workflows.md)
