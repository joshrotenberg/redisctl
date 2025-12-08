# ACL Management

Redis Cloud Access Control Lists (ACLs) provide fine-grained control over database access. redisctl supports managing Redis rules, roles, and ACL users.

## Commands Overview

```bash
redisctl cloud acl --help
```

### Redis Rules

Redis rules define what Redis commands and keys a user can access.

```bash
# List all Redis ACL rules
redisctl cloud acl list-redis-rules <subscription_id>

# Create a new Redis ACL rule
redisctl cloud acl create-redis-rule <subscription_id> --data '{"name": "read-only", "redisRules": ["+@read", "-@write"]}'

# Update an existing rule
redisctl cloud acl update-redis-rule <subscription_id> <rule_id> --data '{"redisRules": ["+@read", "+@hash"]}'

# Delete a rule
redisctl cloud acl delete-redis-rule <subscription_id> <rule_id>
```

### Roles

Roles group Redis rules and database associations together.

```bash
# List all ACL roles
redisctl cloud acl list-roles <subscription_id>

# Create a new role
redisctl cloud acl create-role <subscription_id> --data '{"name": "app-reader", "redisRules": [{"ruleId": 123}]}'

# Update a role
redisctl cloud acl update-role <subscription_id> <role_id> --data '{"name": "app-reader-v2"}'

# Delete a role
redisctl cloud acl delete-role <subscription_id> <role_id>
```

### ACL Users

ACL users are the actual accounts that connect to databases with specific permissions.

```bash
# List all ACL users
redisctl cloud acl list-acl-users <subscription_id>

# Get user details
redisctl cloud acl get-acl-user <subscription_id> <user_id>

# Create a new ACL user
redisctl cloud acl create-acl-user <subscription_id> --data '{"name": "app-user", "password": "secure-password", "roles": [{"roleId": 456}]}'

# Update an ACL user
redisctl cloud acl update-acl-user <subscription_id> <user_id> --data '{"password": "new-password"}'

# Delete an ACL user
redisctl cloud acl delete-acl-user <subscription_id> <user_id>
```

## JSON Output

All commands support `-o json` for structured output:

```bash
redisctl cloud acl list-roles 12345 -o json | jq '.[] | {name, id}'
```

## Common Patterns

### Create a Read-Only User

```bash
# 1. Create a read-only rule
redisctl cloud acl create-redis-rule 12345 --data '{
  "name": "readonly-rule",
  "redisRules": ["+@read", "-@write", "-@admin", "-@dangerous"]
}'

# 2. Create a role using that rule
redisctl cloud acl create-role 12345 --data '{
  "name": "readonly-role",
  "redisRules": [{"ruleId": <rule_id>}]
}'

# 3. Create a user with that role
redisctl cloud acl create-acl-user 12345 --data '{
  "name": "readonly-user",
  "password": "secure-password",
  "roles": [{"roleId": <role_id>}]
}'
```
