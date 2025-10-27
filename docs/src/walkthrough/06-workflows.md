# 6. Layer 3: Workflows

**Multi-step orchestrated operations**

## Why Workflows?

Some operations require multiple API calls in sequence:
- Cluster initialization
- Database migration
- Multi-region setup

Workflows handle this automatically.

## Enterprise: Cluster Initialization

**Before workflows:** 10+ manual steps
1. Bootstrap cluster
2. Set admin password
3. Accept license
4. Configure settings
5. Create default database
6. Verify connectivity
...

**With workflows:** One command

```bash
redisctl enterprise workflow init-cluster \
  --cluster-name "production" \
  --username "admin@cluster.local" \
  --password "YourSecurePassword"
```

### What It Does

✅ Bootstraps the cluster  
✅ Sets credentials  
✅ Accepts license terms  
✅ Creates default database  
✅ Verifies everything works  

All with progress feedback!

## Cloud: Subscription Setup

```bash
redisctl cloud workflow subscription-setup \
  --name "production" \
  --cloud-provider "AWS" \
  --region "us-east-1" \
  --create-database \
  --wait
```

### What It Does

✅ Creates subscription  
✅ Waits for provisioning  
✅ Creates database (if requested)  
✅ Verifies connectivity  

## Benefits

- **Automatic polling** - No manual status checks
- **Error handling** - Rollback on failure
- **Progress indicators** - See what's happening
- **Validation** - Checks preconditions
- **Consistency** - Same steps every time

## Available Workflows

### Enterprise
- `init-cluster` - Complete cluster setup

### Cloud
- `subscription-setup` - End-to-end subscription creation

### Future (see issue #411)
- Database migration
- Active-Active setup
- Disaster recovery
- Rolling upgrades

## When to Use

✅ Complex multi-step operations  
✅ Need consistency across environments  
✅ Want progress feedback  
✅ Avoid manual coordination  

---

**← Previous:** [5. Human-Friendly Layer](./05-human-friendly.md)  
**Next →** [7. Advanced Features](./07-advanced.md)

**Layer Stack:** Raw API → Human-Friendly → **Workflows**

See [Enterprise Workflows](../enterprise/advanced/workflows.md)
