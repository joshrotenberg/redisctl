# Configure Redis ACLs

Time: 10 minutes  
Prerequisites:
- Redis Enterprise cluster (v6.0+)
- redisctl configured with Enterprise credentials
- Understanding of Redis ACL syntax

## Quick Setup

```bash
# Create ACL with read-only access
redisctl enterprise redis-acl create \
  --data '{
    "name": "readonly",
    "acl": "+@read ~*"
  }' \
  --wait

# Apply to database
redisctl enterprise database update \
  --database-id 1 \
  --data '{
    "redis_acls": [{"name": "readonly"}]
  }' \
  --wait
```

## Redis ACL Syntax

### Command Permissions

```bash
+@read      # Allow all read commands
+@write     # Allow all write commands
+@admin     # Allow admin commands
-@dangerous # Deny dangerous commands
+get +set   # Allow specific commands
-flushdb    # Deny specific command
```

### Key Patterns

```bash
~*              # All keys
~cache:*        # Keys starting with "cache:"
~user:123:*     # Specific user keys
~* ~-secret:*   # All except "secret:" prefix
```

## Creating ACL Rules

### Basic ACL Rules

```bash
# Read-only access
redisctl enterprise redis-acl create \
  --data '{
    "name": "readonly",
    "acl": "+@read ~*"
  }'

# Write to specific keys
redisctl enterprise redis-acl create \
  --data '{
    "name": "cache-writer",
    "acl": "+@write +@read ~cache:*"
  }'

# Admin without dangerous commands
redisctl enterprise redis-acl create \
  --data '{
    "name": "safe-admin",
    "acl": "+@all -@dangerous ~*"
  }'
```

### Apply ACLs to Database

```bash
redisctl enterprise database update \
  --database-id 1 \
  --data '{
    "redis_acls": [
      {"name": "readonly", "password": "ReadPass123!"},
      {"name": "cache-writer", "password": "WritePass456!"}
    ]
  }' \
  --wait
```

## Testing ACLs

```bash
# Test readonly user
redis-cli -h localhost -p 12000 \
  --user readonly \
  --pass ReadPass123! \
  GET mykey  # Works

redis-cli --user readonly --pass ReadPass123! \
  SET mykey value  # Fails with NOPERM

# Test cache-writer user
redis-cli --user cache-writer --pass WritePass456! \
  SET cache:item value  # Works

redis-cli --user cache-writer --pass WritePass456! \
  SET other:item value  # Fails
```

## Common ACL Patterns

### Application Access Tiers

```bash
# Level 1: Read-only
redisctl enterprise redis-acl create \
  --data '{"name": "app-read", "acl": "+@read +ping ~*"}'

# Level 2: Read + Write cache
redisctl enterprise redis-acl create \
  --data '{"name": "app-cache", "acl": "+@read +@write ~cache:* ~session:*"}'

# Level 3: Full access
redisctl enterprise redis-acl create \
  --data '{"name": "app-admin", "acl": "+@all -flushdb -flushall ~*"}'
```

### Multi-Tenant Isolation

```bash
# Tenant A
redisctl enterprise redis-acl create \
  --data '{"name": "tenant-a", "acl": "+@all ~tenant:a:*"}'

# Tenant B
redisctl enterprise redis-acl create \
  --data '{"name": "tenant-b", "acl": "+@all ~tenant:b:*"}'
```

## Managing ACLs

### List ACLs

```bash
redisctl enterprise redis-acl list -o table
```

### Update ACL

```bash
redisctl enterprise redis-acl update \
  --acl-id 123 \
  --data '{
    "name": "readonly",
    "acl": "+@read +@connection ~*"
  }'
```

### Delete ACL

```bash
redisctl enterprise redis-acl delete --acl-id 123
```

## Best Practices

1. **Principle of Least Privilege** - Grant minimum required access
2. **Use Key Prefixes** - Design schema for ACL isolation
3. **Deny Dangerous Commands** - Always exclude FLUSHDB, KEYS, etc.
4. **Strong Passwords** - Use secure passwords for each ACL
5. **Test Thoroughly** - Verify ACLs before production use
6. **Document ACLs** - Maintain clear documentation of each rule

## Next Steps

- [Create Database](create-database.md) - Database setup
- [Configure Replication](configure-replication.md) - High availability
- [Cluster Health Check](cluster-health.md) - Monitoring

## See Also

- [Redis ACL Documentation](https://redis.io/docs/latest/operate/oss_and_stack/management/security/acl/)
- [Enterprise Security](https://redis.io/docs/latest/operate/rs/security/)
