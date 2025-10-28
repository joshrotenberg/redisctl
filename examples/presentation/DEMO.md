# Live Demo Script - redisctl Presentation

Clean, copy-paste ready commands for terminal demo.

## Setup (Run Once Before Presentation)

```bash
# Start Docker cluster
docker compose up -d

# Set environment variables (keep in terminal)
export REDIS_ENTERPRISE_URL="https://localhost:9443"
export REDIS_ENTERPRISE_USER="admin@redis.local"
export REDIS_ENTERPRISE_PASSWORD="Redis123!"
export REDIS_ENTERPRISE_INSECURE="true"
```

## Demo Flow

### 1. Show the Problem (30 seconds)

**Before redisctl - The old way with curl + jq + polling:**

```bash
# Get cluster info - verbose curl
curl -k -u "admin@redis.local:Redis123!" \
  https://localhost:9443/v1/cluster | jq '.name'

# List databases - messy output
curl -k -u "admin@redis.local:Redis123!" \
  https://localhost:9443/v1/bdbs | jq
```

### 2. Show redisctl - The new way (2 minutes)

**Clean, simple commands:**

```bash
# Cluster info - one command
redisctl enterprise cluster get -o json -q 'name'

# List databases - clean table
redisctl enterprise database list -o table

# Database details with filtering
redisctl enterprise database get 1 -o json -q '{name: name, memory: memory_size, status: status}'
```

### 3. Show the Comprehensive Status Command (1 minute)

**NEW feature - single view of everything:**

```bash
# Show everything at once (like rladmin status)
redisctl enterprise status -o json -q 'summary'

# Full status
redisctl enterprise status
```

### 4. Show Structured Output Power (1 minute)

**JSON/YAML for automation:**

```bash
# Get all database names as array
redisctl enterprise database list -o json -q '[].name'

# Filter active databases only
redisctl enterprise database list -o json -q '[?status==`active`].{name: name, memory: memory_size}'

# YAML output for configs
redisctl enterprise cluster get -o yaml
```

### 5. Show Multi-Cluster Management (1 minute)

**Profile system:**

```bash
# List profiles
redisctl profile list

# Switch between clusters (if you have multiple)
redisctl enterprise database list --profile prod
redisctl enterprise database list --profile staging
```

### 6. Show Support Package Automation (30 seconds)

**From 10 minutes to 30 seconds:**

```bash
# Generate optimized support package
redisctl enterprise support-package cluster --optimize

# With upload (if configured)
# redisctl enterprise support-package cluster --optimize --upload
```

## Recovery Commands (If Demo Fails)

```bash
# Restart Docker cluster
docker compose restart

# Check cluster health
curl -k -u "admin@redis.local:Redis123!" https://localhost:9443/v1/cluster/healthcheck

# Rebuild if needed
docker compose down -v && docker compose up -d
```

## Key Talking Points During Demo

- **No SSH required** - All remote via REST API
- **Structured output** - JSON/YAML for automation
- **JMESPath queries** - Filter output easily
- **Cross-platform** - Works on macOS, Linux, Windows
- **One binary** - No dependencies
- **Profile management** - Multi-cluster support
- **Library-first** - Can be used in Terraform, automation tools

## Notes

- Keep the terminal window large (readable font)
- Have commands pre-typed or in clipboard manager
- Run through sequence 2-3 times before presentation
- If a command hangs, Ctrl+C and move on
- The Docker cluster is local - everything is fast
