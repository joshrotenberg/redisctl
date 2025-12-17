# User Management

Manage Redis Cloud account users who can access the Redis Cloud console and API.

> **Note**: These are Redis Cloud *account* users (console/API access), not database ACL users. For database-level access control, see [ACL Management](acl.md).

## Commands Overview

```bash
redisctl cloud user --help
```

## List Users

```bash
# List all users in your account
redisctl cloud user list

# Output as JSON
redisctl cloud user list -o json
```

## Get User Details

```bash
# Get details for a specific user
redisctl cloud user get <user_id>

# Get specific fields with JMESPath
redisctl cloud user get <user_id> -q '{name: name, email: email, role: role}'
```

## Update User

```bash
# Update user information
redisctl cloud user update <user_id> --data '{"name": "New Name"}'
```

## Delete User

```bash
# Delete a user from the account
redisctl cloud user delete <user_id>
```

## JMESPath Queries

All commands support JMESPath queries for filtering and formatting:

```bash
# List all user emails
redisctl cloud user list -q '[].email'

# Find owner users
redisctl cloud user list -q "[?role=='owner']"

# Get specific fields
redisctl cloud user list -q '[].{email: email, role: role}' -o table
```

## User Roles

Redis Cloud account users have roles that determine their permissions:

- **owner**: Full access to all account features
- **member**: Access to assigned resources
- **viewer**: Read-only access

For managing database-level permissions (what Redis commands users can run), use the [ACL commands](acl.md) instead.
