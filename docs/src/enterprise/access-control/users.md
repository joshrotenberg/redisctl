# User Management

Manage users on your Redis Enterprise cluster who can access the cluster management UI and API.

## Commands Overview

```bash
redisctl enterprise user --help
```

## List Users

```bash
# List all users in the cluster
redisctl enterprise user list

# Output as JSON
redisctl enterprise user list -o json
```

## Get User Details

```bash
# Get details for a specific user
redisctl enterprise user get <user_id>

# Get specific fields
redisctl enterprise user get <user_id> -q '{name: name, email: email, role: role}'
```

## Create User

```bash
# Create a new user
redisctl enterprise user create --data '{
  "name": "operator",
  "email": "operator@example.com",
  "password": "secure-password",
  "role": "db_viewer"
}'
```

## Update User

```bash
# Update user information
redisctl enterprise user update <user_id> --data '{
  "name": "Senior Operator"
}'
```

## Delete User

```bash
# Delete a user
redisctl enterprise user delete <user_id>
```

## Password Management

```bash
# Reset a user's password
redisctl enterprise user reset-password <user_id> --data '{
  "password": "new-secure-password"
}'
```

## Role Management

Users are assigned roles that determine their permissions.

```bash
# Get user's current roles
redisctl enterprise user get-roles <user_id>

# Assign a role to a user
redisctl enterprise user assign-role <user_id> --data '{
  "role_uid": "<role_id>"
}'

# Remove a role from a user
redisctl enterprise user remove-role <user_id> <role_id>
```

## Built-in Roles

Redis Enterprise includes these built-in roles:

| Role | Description |
|------|-------------|
| `admin` | Full cluster administration |
| `cluster_member` | View cluster info, manage some settings |
| `cluster_viewer` | Read-only cluster access |
| `db_member` | Manage databases |
| `db_viewer` | Read-only database access |
| `none` | No default permissions |

## JMESPath Queries

```bash
# List all users with their roles
redisctl enterprise user list -q '[].{name: name, email: email, role: role}'

# Find admin users
redisctl enterprise user list -q "[?role=='admin']"
```

## Scripting Examples

### Audit All Users

```bash
# Export user list for audit
redisctl enterprise user list -q '[].{name: name, email: email, role: role}' -o table
```

### Bulk User Creation

```bash
# Create users from a JSON file (one user per line in JSONL format)
while read user; do
  redisctl enterprise user create --data "$user"
done < users.jsonl
```
