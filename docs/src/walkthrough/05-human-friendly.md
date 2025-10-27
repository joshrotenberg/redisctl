# 5. Layer 2: Human-Friendly Commands

**Type-safe wrappers with better UX**

## Why Human-Friendly Layer?

Better experience for day-to-day operations:
- ✅ Type-safe parameters
- ✅ Automatic error handling
- ✅ Progress indicators
- ✅ Better output formatting
- ✅ Command completion

## Cloud Operations

```bash
# List subscriptions
redisctl cloud subscription list -o table

# Get subscription details
redisctl cloud subscription get 12345

# Create database (with automatic polling)
redisctl cloud database create \
  --subscription-id 12345 \
  --data '{"name": "mydb", "memoryLimitInGb": 1}' \
  --wait

# List databases
redisctl cloud database list --subscription-id 12345
```

## Enterprise Operations

```bash
# Cluster information
redisctl enterprise cluster get

# List databases
redisctl enterprise database list -o table

# Get database details
redisctl enterprise database get 1

# Create database
redisctl enterprise database create \
  --name "mydb" \
  --memory-size "1GB" \
  --port 12000

# List nodes
redisctl enterprise node list
```

## Output Formats

### JSON (default)
```bash
redisctl enterprise database list
# Parseable for scripts
```

### Table (human-readable)
```bash
redisctl enterprise database list -o table
# ┌────┬──────────┬────────┬──────┐
# │ ID │ Name     │ Status │ Port │
# └────┴──────────┴────────┴──────┘
```

### YAML (config files)
```bash
redisctl enterprise database get 1 -o yaml
```

## Automatic Polling with --wait

The `--wait` flag automatically polls until operations complete:

```bash
# Create and wait for completion
redisctl cloud database create \
  --subscription-id 12345 \
  --data '{...}' \
  --wait \
  --wait-timeout 300

# Shows progress, exits when done
```

No more manual polling loops!

## JMESPath Filtering

```bash
# Get only database names
redisctl enterprise database list -q "[].name"

# Active databases with specific fields
redisctl enterprise database list \
  -q "[?status=='active'].{name:name,mem:memory_size}"

# Count databases
redisctl enterprise database list -q "length(@)"
```

## Live Demo

```bash
# Try these commands (if you have Docker running)
redisctl enterprise cluster get
redisctl enterprise database list -o table
redisctl enterprise node list
```

---

**← Previous:** [4. Raw API Layer](./04-raw-api.md)  
**Next →** [6. Workflows Layer](./06-workflows.md)

**Layer Stack:** Raw API → **Human-Friendly** → Workflows

See [Cloud Commands](../cloud/commands.md) | [Enterprise Commands](../enterprise/README.md)
