# 6. Layer 3: Workflows

**Multi-step orchestrated operations**

## Why Workflows?

Some operations require multiple API calls in sequence. Workflows handle this automatically.

## Enterprise: Cluster Initialization

**Before workflows:** 10+ manual steps (bootstrap, credentials, license, database, verify...)

**With workflows:** One command

```bash
# As shown in docker-compose
redisctl enterprise workflow init-cluster \
  --name "docker-cluster" \
  --username "admin@redis.local" \
  --password "Redis123!"
```

**What It Does:**
- Bootstraps the cluster
- Sets credentials
- Accepts license terms
- Creates default database
- Verifies connectivity

All with progress feedback!

**Output:**
```
Initializing Redis Enterprise cluster...
Bootstrap completed successfully
Cluster is ready
Creating default database 'default-db'...
Database created successfully (ID: 1)
Database connectivity verified (PING successful)

Cluster initialization completed successfully
```

## Benefits

- **Automatic polling** - No manual status checks
- **Error handling** - Rollback on failure
- **Progress indicators** - See what's happening
- **Validation** - Checks preconditions
- **Consistency** - Same steps every time

## Available Workflows

**Enterprise:**
- `init-cluster` - Complete cluster setup

**Cloud:**
- `subscription-setup` - End-to-end subscription creation

**Future (see issue #411):**
- Database migration
- Active-Active setup
- Disaster recovery

## When to Use

Use workflows for complex multi-step operations that need consistency and progress feedback.

---

**Previous:** [5. Human-Friendly Layer](./05-human-friendly.md)  
**Next:** [7. Advanced Features](./07-advanced.md)

**Layer Stack:** Raw API → Human-Friendly → **Workflows**
