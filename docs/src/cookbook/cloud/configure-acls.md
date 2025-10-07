# Configure ACL Security

Time: 10-15 minutes  
Prerequisites:
- Redis Cloud database already created
- redisctl configured with Cloud credentials
- Basic understanding of Redis ACL commands

## What are ACLs?

Access Control Lists (ACLs) allow you to create users with specific permissions, limiting which commands they can run and which keys they can access. This is essential for:
- Multi-tenant applications
- Restricting administrative access
- Compliance requirements
- Defense in depth security

## Quick Command

Create a read-only user for your application:

```bash
# Create Redis rule
redisctl cloud acl create-redis-rule \
  --subscription-id YOUR_SUB_ID \
  --data '{"name": "readonly-rule", "rule": "+@read ~*"}' \
  --wait

# Create role with the rule
redisctl cloud acl create-role \
  --subscription-id YOUR_SUB_ID \
  --data '{"name": "readonly-role", "redis_rules": [{"rule_name": "readonly-rule"}]}' \
  --wait

# Create user with the role
redisctl cloud acl create-acl-user \
  --subscription-id YOUR_SUB_ID \
  --data '{"name": "app-reader", "role": "readonly-role", "password": "SecurePass123!"}' \
  --wait
```

## Step-by-Step Guide

### Understanding the ACL Hierarchy

Redis Cloud uses a three-level ACL system:
1. **Redis Rules** - Define command and key access patterns (Redis ACL syntax)
2. **Roles** - Group multiple Redis rules together
3. **Users** - Assigned one role and a password

### 1. List Existing ACL Components

```bash
# View current Redis rules
redisctl cloud acl list-redis-rules --subscription-id 42 -o table

# View current roles
redisctl cloud acl list-roles --subscription-id 42 -o table

# View current users
redisctl cloud acl list-acl-users --subscription-id 42 -o table
```

### 2. Create Redis ACL Rules

Redis rules use standard Redis ACL syntax.

#### Common Rule Patterns

**Read-only access:**
```bash
redisctl cloud acl create-redis-rule \
  --subscription-id 42 \
  --data '{
    "name": "readonly",
    "rule": "+@read ~*"
  }' \
  --wait
```

**Write-only to specific keys:**
```bash
redisctl cloud acl create-redis-rule \
  --subscription-id 42 \
  --data '{
    "name": "write-metrics",
    "rule": "+set +del ~metrics:*"
  }' \
  --wait
```

**Full access except dangerous commands:**
```bash
redisctl cloud acl create-redis-rule \
  --subscription-id 42 \
  --data '{
    "name": "safe-admin",
    "rule": "+@all -@dangerous ~*"
  }' \
  --wait
```

**Access to specific key prefix:**
```bash
redisctl cloud acl create-redis-rule \
  --subscription-id 42 \
  --data '{
    "name": "user-sessions",
    "rule": "+@all ~session:*"
  }' \
  --wait
```

### 3. Create ACL Roles

Roles combine one or more Redis rules:

```bash
# Simple role with one rule
redisctl cloud acl create-role \
  --subscription-id 42 \
  --data '{
    "name": "readonly-role",
    "redis_rules": [
      {"rule_name": "readonly"}
    ]
  }' \
  --wait

# Complex role with multiple rules
redisctl cloud acl create-role \
  --subscription-id 42 \
  --data '{
    "name": "app-worker",
    "redis_rules": [
      {"rule_name": "readonly"},
      {"rule_name": "write-metrics"}
    ]
  }' \
  --wait
```

### 4. Create ACL Users

Users are assigned a role and password:

```bash
redisctl cloud acl create-acl-user \
  --subscription-id 42 \
  --data '{
    "name": "app-reader",
    "role": "readonly-role",
    "password": "SecureReadOnlyPass123!"
  }' \
  --wait
```

**What you should see:**
```json
{
  "taskId": "abc123...",
  "status": "processing"
}
...
ACL user created successfully!
{
  "id": 456,
  "name": "app-reader",
  "role": "readonly-role",
  "status": "active"
}
```

### 5. Assign Users to Databases

After creating users, assign them to specific databases:

```bash
# Get database ID
redisctl cloud database list \
  --subscription-id 42 \
  -q '[].{id: database_id, name: name}'

# Update database with ACL users
redisctl cloud database update \
  --subscription-id 42 \
  --database-id 12345 \
  --data '{
    "security": {
      "users": ["app-reader", "app-writer"]
    }
  }' \
  --wait
```

### 6. Test ACL User

Connect to your database with the new user:

```bash
# Get database endpoint
redisctl cloud database get \
  --subscription-id 42 \
  --database-id 12345 \
  -q '{endpoint: public_endpoint, port: port}'

# Test connection
redis-cli -h redis-12345.cloud.redislabs.com \
  -p 12345 \
  --user app-reader \
  --pass SecureReadOnlyPass123! \
  PING

# Test permissions (should succeed)
redis-cli --user app-reader --pass SecureReadOnlyPass123! \
  -h redis-12345.cloud.redislabs.com -p 12345 \
  GET mykey

# Test restricted command (should fail)
redis-cli --user app-reader --pass SecureReadOnlyPass123! \
  -h redis-12345.cloud.redislabs.com -p 12345 \
  SET mykey value
# Error: NOPERM this user has no permissions to run the 'set' command
```

## Common ACL Patterns

### Application Access Pattern

Separate users for read, write, and admin operations:

```bash
# Read-only for queries
redisctl cloud acl create-redis-rule --subscription-id 42 \
  --data '{"name": "app-read", "rule": "+@read +@connection ~*"}' --wait

# Write access for updates
redisctl cloud acl create-redis-rule --subscription-id 42 \
  --data '{"name": "app-write", "rule": "+@write +@read +@connection ~*"}' --wait

# Admin for maintenance
redisctl cloud acl create-redis-rule --subscription-id 42 \
  --data '{"name": "app-admin", "rule": "+@all ~*"}' --wait

# Create roles and users
redisctl cloud acl create-role --subscription-id 42 \
  --data '{"name": "reader", "redis_rules": [{"rule_name": "app-read"}]}' --wait

redisctl cloud acl create-role --subscription-id 42 \
  --data '{"name": "writer", "redis_rules": [{"rule_name": "app-write"}]}' --wait

redisctl cloud acl create-role --subscription-id 42 \
  --data '{"name": "admin", "redis_rules": [{"rule_name": "app-admin"}]}' --wait
```

### Multi-Tenant Pattern

Isolate tenants by key prefix:

```bash
# Tenant A access
redisctl cloud acl create-redis-rule --subscription-id 42 \
  --data '{"name": "tenant-a", "rule": "+@all ~tenant:a:*"}' --wait

# Tenant B access
redisctl cloud acl create-redis-rule --subscription-id 42 \
  --data '{"name": "tenant-b", "rule": "+@all ~tenant:b:*"}' --wait

# Create roles and users
redisctl cloud acl create-role --subscription-id 42 \
  --data '{"name": "tenant-a-role", "redis_rules": [{"rule_name": "tenant-a"}]}' --wait

redisctl cloud acl create-acl-user --subscription-id 42 \
  --data '{"name": "tenant-a-user", "role": "tenant-a-role", "password": "TenantAPass123!"}' --wait
```

## Using Configuration Files

For complex ACL setups:

```bash
cat > acl-setup.json << 'EOF'
{
  "rules": [
    {
      "name": "readonly",
      "rule": "+@read ~*"
    },
    {
      "name": "write-cache",
      "rule": "+set +get +del +expire ~cache:*"
    }
  ],
  "roles": [
    {
      "name": "cache-worker",
      "redis_rules": [
        {"rule_name": "readonly"},
        {"rule_name": "write-cache"}
      ]
    }
  ],
  "users": [
    {
      "name": "worker-1",
      "role": "cache-worker",
      "password": "Worker1Pass!"
    }
  ]
}
EOF

# Create rules
jq -r '.rules[] | @json' acl-setup.json | while read rule; do
  redisctl cloud acl create-redis-rule \
    --subscription-id 42 \
    --data "$rule" \
    --wait
done

# Create roles
jq -r '.roles[] | @json' acl-setup.json | while read role; do
  redisctl cloud acl create-role \
    --subscription-id 42 \
    --data "$role" \
    --wait
done

# Create users
jq -r '.users[] | @json' acl-setup.json | while read user; do
  redisctl cloud acl create-acl-user \
    --subscription-id 42 \
    --data "$user" \
    --wait
done
```

## Redis ACL Syntax Reference

Common patterns in Redis ACL rules:

**Command categories:**
- `+@read` - All read commands
- `+@write` - All write commands
- `+@admin` - Administrative commands
- `+@dangerous` - Dangerous commands (FLUSHDB, KEYS, etc.)
- `+@all` - All commands
- `-@dangerous` - Deny dangerous commands

**Specific commands:**
- `+get` - Allow GET command
- `+set` - Allow SET command
- `-flushdb` - Deny FLUSHDB

**Key patterns:**
- `~*` - All keys
- `~cache:*` - Keys starting with "cache:"
- `~user:*` - Keys starting with "user:"
- `~*` `~-secret:*` - All keys except those starting with "secret:"

## Managing ACLs

### View ACL Details

```bash
# Get specific user details
redisctl cloud acl get-acl-user \
  --subscription-id 42 \
  --user-id 456 \
  -o json

# List all users with their roles
redisctl cloud acl list-acl-users \
  --subscription-id 42 \
  -o json \
  -q '[].{name: name, role: role, id: id}'
```

### Update ACL Rules

```bash
# Update existing rule
redisctl cloud acl update-redis-rule \
  --subscription-id 42 \
  --rule-id 789 \
  --data '{
    "name": "readonly",
    "rule": "+@read +@connection ~*"
  }' \
  --wait
```

### Update User Password

```bash
redisctl cloud acl update-acl-user \
  --subscription-id 42 \
  --user-id 456 \
  --data '{
    "password": "NewSecurePass456!"
  }' \
  --wait
```

### Delete ACL Components

```bash
# Delete user
redisctl cloud acl delete-acl-user \
  --subscription-id 42 \
  --user-id 456 \
  --wait

# Delete role
redisctl cloud acl delete-role \
  --subscription-id 42 \
  --role-id 321 \
  --wait

# Delete Redis rule
redisctl cloud acl delete-redis-rule \
  --subscription-id 42 \
  --rule-id 789 \
  --wait
```

## Common Issues

### Cannot Create User with Reserved Name

```
Error: User name 'default' is reserved
```

**Solution:** Avoid reserved names: `default`, `admin`. Use descriptive application-specific names.

### ACL Rule Syntax Error

```
Error: Invalid ACL rule syntax
```

**Solution:** Test your ACL rule locally first:
```bash
redis-cli ACL SETUSER testuser "+@read ~*"
redis-cli ACL GETUSER testuser
redis-cli ACL DELUSER testuser
```

### User Cannot Connect

**Troubleshooting:**
1. Verify user is assigned to the database
2. Check password is correct
3. Ensure user status is "active"
4. Test with default user first to isolate ACL vs. network issues

### Permission Denied

```
Error: NOPERM this user has no permissions to run the 'set' command
```

**Solution:** Review and update the user's role and rules:
```bash
# Check user's role
redisctl cloud acl get-acl-user --subscription-id 42 --user-id 456 -q 'role'

# Check role's rules
redisctl cloud acl list-roles --subscription-id 42 -q '[?name==`readonly-role`]'
```

## Best Practices

1. **Principle of Least Privilege:** Give users only the permissions they need
2. **Use Key Prefixes:** Design your key naming to support ACLs (e.g., `user:123:profile`)
3. **Separate Credentials:** Different users for read vs. write operations
4. **Rotate Passwords:** Regularly update user passwords
5. **Test Before Production:** Verify ACL rules in a test database first
6. **Document Rules:** Keep track of what each rule and role does

## Next Steps

- [Setup VPC Peering](setup-vpc-peering.md) - Private network connectivity
- [Configure TLS/SSL](configure-tls.md) - Encryption in transit
- [Backup and Restore](backup-restore.md) - Protect your data
- [Monitor Performance](../common/monitor-performance.md) - Track database metrics

## See Also

- [ACL Command Reference](../../cloud/acl-management.md) - Complete command documentation
- [Redis ACL Documentation](https://redis.io/docs/latest/operate/oss_and_stack/management/security/acl/) - Redis ACL syntax
- [Redis Cloud Security](https://redis.io/docs/latest/operate/rc/security/) - Security best practices
