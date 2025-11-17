# redisctl Live Demo

Clean, slide-style reference for terminal demo.

---

## 1. Installation

```bash
brew tap joshrotenberg/brew
brew install redisctl

# Verify
redisctl --version
```

---

## 2. Profile Setup - Redis Cloud

```bash
# Configure Cloud profile
redisctl profile set demo --deployment cloud \
  --api-key <YOUR_API_KEY> \
  --api-secret <YOUR_API_SECRET>

# Verify profile
redisctl profile list

# Show profile details
redisctl profile show demo
```

---

## 3. Explore Your Account

```bash
# Get account information
redisctl cloud account get

# List subscriptions
redisctl cloud subscription list
```

---

## 4. List and View Databases

```bash
# List all databases
redisctl cloud database list

# Get specific database details
redisctl cloud database get 2969240:13684622

# Get connection info with filtering
redisctl cloud database get 2969240:13684622 \
  -o json -q '{endpoint: publicEndpoint, password: security.password}'
```

---

## 5. Connect with redis-cli

```bash
# Connect using the endpoint and password from above
redis-cli -u redis://default:<YOUR_PASSWORD>@<YOUR_ENDPOINT>:<PORT>

# Example: redis-cli -u redis://default:password123@redis-12345.c1.us-east-1-3.ec2.redns.redis-cloud.com:12345

# Test some commands
SET demo:key "Hello from redisctl demo"
GET demo:key
```

---

---

# Redis Enterprise Demo

## 6. Start Local Enterprise Cluster

```bash
# Start Redis Enterprise with automatic initialization
docker compose up -d

# Watch the initialization (will show all redisctl commands)
docker compose logs -f redis-enterprise-init
```

---

## 7. Profile Setup - Redis Enterprise

```bash
# Configure Enterprise profile (already exists as 'local')
redisctl profile list

# Or create a new one
redisctl profile set demo-enterprise --deployment enterprise \
  --url https://localhost:9443 \
  --username admin@redis.local \
  --password Redis123! \
  --insecure

# Show profile details
redisctl profile show demo-enterprise
```

---

## 8. Explore Enterprise Cluster

```bash
# Get cluster information
redisctl enterprise cluster get

# List nodes
redisctl enterprise node list

# List databases (default-db, cache-db, persistent-db)
redisctl enterprise database list

# Get database details with filtering
redisctl enterprise database get 2 \
  -o json -q '{name: name, status: status, memory: memory_size}'
```

---

## 9. Check Available Modules

```bash
# List available Redis modules
redisctl enterprise module list

# Get license info
redisctl enterprise license get -o json
```

---

## 10. Raw API Access

```bash
# Get cluster alerts
redisctl api enterprise get /v1/cluster/alerts

# Get cluster stats
redisctl enterprise cluster stats -o json
```

---

## 11. First-Class Parameters (NEW Feature)

```bash
# OLD WAY - JSON required (easy to mess up!)
redisctl enterprise database create \
  --data '{"name":"demo","memory_size":1073741824,"replication":true}'

# NEW WAY - clean CLI flags
redisctl enterprise database create \
  --name demo --memory 1073741824 --replication --dry-run

# Works for Cloud too
redisctl cloud database create --subscription 123 \
  --name mydb --memory 5 --replication \
  --data-persistence aof --eviction-policy allkeys-lru

# Cloud subscription creation
redisctl cloud subscription create \
  --name prod-subscription \
  --payment-method marketplace \
  --memory-storage ram-and-flash \
  --data @subscription.json  # For complex nested config
```

**Benefits:**
- No JSON syntax errors
- Clear parameter names
- Built-in validation
- Tab completion works!
- Still supports `--data` for advanced configs

---

## Notes
- Commands ready to copy/paste
- Fill in real IDs during demo
- Keep it moving - don't wait for long operations
- Docker Compose shows complete working examples with all commands
- Highlight the new first-class parameters - it's a major UX win!
