# Enterprise Access Control

Manage users, roles, and LDAP integration for Redis Enterprise.

## Users

### List Users

```bash
redisctl enterprise user list [OPTIONS]
```

**Examples:**

```bash
# List all users
redisctl enterprise user list

# Table format
redisctl enterprise user list -o table

# Get usernames and roles
redisctl enterprise user list -q "[].{name:name,role:role,email:email}"
```

### Get User

```bash
redisctl enterprise user get <ID> [OPTIONS]
```

### Create User

```bash
redisctl enterprise user create --data <JSON>
```

**Examples:**

```bash
# Create admin user
redisctl enterprise user create --data '{
  "name": "admin",
  "email": "admin@example.com",
  "password": "SecurePass123!",
  "role": "admin"
}'

# Create viewer user
redisctl enterprise user create --data '{
  "name": "viewer",
  "email": "viewer@example.com",
  "password": "ViewPass123!",
  "role": "db_viewer"
}'
```

### Update User

```bash
redisctl enterprise user update <ID> --data <JSON>
```

**Examples:**

```bash
# Change password
redisctl enterprise user update 2 --data '{"password": "NewPass123!"}'

# Update role
redisctl enterprise user update 2 --data '{"role": "db_member"}'
```

### Delete User

```bash
redisctl enterprise user delete <ID>
```

## Roles

### List Roles

```bash
redisctl enterprise role list
```

### Get Role

```bash
redisctl enterprise role get <ID>
```

### Create Role

```bash
redisctl enterprise role create --data <JSON>
```

**Example:**

```bash
redisctl enterprise role create --data '{
  "name": "custom-role",
  "management": "db_member"
}'
```

### Built-in Roles

| Role | Description |
|------|-------------|
| `admin` | Full cluster access |
| `cluster_member` | Cluster management, no user management |
| `cluster_viewer` | Read-only cluster access |
| `db_member` | Database management |
| `db_viewer` | Read-only database access |

## Redis ACLs

Manage Redis ACL rules for database-level access control.

### List ACLs

```bash
redisctl enterprise acl list
```

### Get ACL

```bash
redisctl enterprise acl get <ID>
```

### Create ACL

```bash
redisctl enterprise acl create --data <JSON>
```

**Examples:**

```bash
# Read-only ACL
redisctl enterprise acl create --data '{
  "name": "readonly",
  "acl": "+@read ~*"
}'

# Write to specific keys
redisctl enterprise acl create --data '{
  "name": "app-writer",
  "acl": "+@all ~app:*"
}'
```

### Common ACL Patterns

| Pattern | Description |
|---------|-------------|
| `+@all ~*` | Full access |
| `+@read ~*` | Read-only |
| `+@write ~prefix:*` | Write to prefix:* keys |
| `-@dangerous` | Deny dangerous commands |
| `+get +set ~*` | Only GET and SET |

## LDAP Integration

### Get LDAP Configuration

```bash
redisctl enterprise ldap get-config
```

### Update LDAP Configuration

```bash
redisctl enterprise ldap update-config --data <JSON>
```

**Example:**

```bash
redisctl enterprise ldap update-config --data '{
  "protocol": "ldaps",
  "servers": [
    {"host": "ldap.example.com", "port": 636}
  ],
  "bind_dn": "cn=admin,dc=example,dc=com",
  "bind_pass": "password",
  "base_dn": "dc=example,dc=com",
  "user_dn_query": "(uid=%u)"
}'
```

### LDAP Mappings

Map LDAP groups to Redis Enterprise roles.

```bash
# List mappings
redisctl enterprise ldap list-mappings

# Create mapping
redisctl enterprise ldap create-mapping --data '{
  "name": "admins-mapping",
  "ldap_group_dn": "cn=admins,ou=groups,dc=example,dc=com",
  "role": "admin"
}'
```

## Examples

### Set Up Service Account

```bash
# Create user for application
redisctl enterprise user create --data '{
  "name": "myapp",
  "email": "myapp@service.local",
  "password": "ServicePass123!",
  "role": "db_member"
}'
```

### Audit User Access

```bash
# List all users with their roles
redisctl enterprise user list \
  -q "[].{name:name,email:email,role:role,auth_method:auth_method}" \
  -o table
```

### Rotate All Passwords

```bash
for user in $(redisctl enterprise user list -q '[].uid' --raw); do
  NEW_PASS=$(openssl rand -base64 16)
  redisctl enterprise user update $user --data "{\"password\": \"$NEW_PASS\"}"
  echo "User $user: $NEW_PASS"
done
```

## Troubleshooting

### "Authentication failed"
- Check username/password
- Verify user exists: `redisctl enterprise user list`
- Check user role has required permissions

### "LDAP connection failed"
- Verify LDAP server is reachable
- Check bind credentials
- Verify SSL certificates for LDAPS

### "ACL denied"
- Check ACL rules: `redisctl enterprise acl get <id>`
- Verify user is associated with correct ACL

## API Reference

REST endpoints:
- `GET/POST /v1/users` - User management
- `GET/POST /v1/roles` - Role management
- `GET/POST /v1/redis_acls` - Redis ACL management
- `GET/PUT /v1/cluster/ldap` - LDAP configuration
- `GET/POST /v1/ldap_mappings` - LDAP mappings

For direct API access: `redisctl api enterprise get /v1/users`
