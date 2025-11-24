# Cloud Access Control

Manage users, roles, and ACLs for Redis Cloud.

## Users

### List Users

```bash
redisctl cloud user list
```

### Get User

```bash
redisctl cloud user get <user-id>
```

### Create User

```bash
redisctl cloud user create --data '{
  "name": "app-user",
  "email": "user@example.com",
  "role": "viewer"
}'
```

### Update User

```bash
redisctl cloud user update <user-id> --data '{
  "role": "member"
}'
```

### Delete User

```bash
redisctl cloud user delete <user-id>
```

## Roles

### List Roles

```bash
redisctl cloud acl role list
```

### Get Role

```bash
redisctl cloud acl role get <role-id>
```

### Create Role

```bash
redisctl cloud acl role create --data '{
  "name": "read-only",
  "redisRules": [
    {
      "ruleName": "Read-Only",
      "databases": [
        {"subscriptionId": 123456, "databaseId": 789}
      ]
    }
  ]
}'
```

### Update Role

```bash
redisctl cloud acl role update <role-id> --data '{
  "name": "read-write"
}'
```

### Delete Role

```bash
redisctl cloud acl role delete <role-id>
```

## Redis Rules

Redis ACL rules define permissions at the Redis command level.

### List Redis Rules

```bash
redisctl cloud acl redis-rule list
```

### Get Redis Rule

```bash
redisctl cloud acl redis-rule get <rule-id>
```

### Create Redis Rule

```bash
redisctl cloud acl redis-rule create --data '{
  "name": "Read-Only",
  "acl": "+@read ~*"
}'
```

### Common ACL Patterns

| Pattern | Description |
|---------|-------------|
| `+@all ~*` | Full access to all keys |
| `+@read ~*` | Read-only access |
| `+@write ~cache:*` | Write only to cache:* keys |
| `-@dangerous` | Deny dangerous commands |

## Examples

### Set Up Read-Only User

```bash
# Create redis rule
redisctl cloud acl redis-rule create --data '{
  "name": "readonly-rule",
  "acl": "+@read -@dangerous ~*"
}'

# Create role with rule
redisctl cloud acl role create --data '{
  "name": "readonly-role",
  "redisRules": [{"ruleName": "readonly-rule", "databases": [...]}]
}'
```

### Audit Access

```bash
# List all users and their roles
redisctl cloud user list -q "[].{name:name,role:role,email:email}" -o table
```

## API Reference

These commands use the following REST endpoints:
- `GET/POST /v1/acl/users` - User management
- `GET/POST /v1/acl/roles` - Role management
- `GET/POST /v1/acl/redisRules` - Redis rule management

For direct API access: `redisctl api cloud get /acl/users`
