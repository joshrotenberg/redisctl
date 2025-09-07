# ACL Management

Access Control List (ACL) management supports async operations for rules, roles, and users.

## Redis ACL Rules

### Create ACL Rule
```bash
redisctl cloud acl create-redis-rule --name "read-only" \
  --rule "+@read" --wait

# Complex rule with multiple permissions
redisctl cloud acl create-redis-rule --name "app-rule" \
  --rule "+@read +@write -@dangerous" --wait
```

### Update ACL Rule
```bash
redisctl cloud acl update-redis-rule --id 123 \
  --rule "+@read +@write" --wait
```

### Delete ACL Rule
```bash
redisctl cloud acl delete-redis-rule --id 123 --force --wait
```

## ACL Roles

### Create Role
```bash
# With single rule
redisctl cloud acl create-role --name "read-only-role" \
  --redis-rules "123" --wait

# With multiple rules (JSON array)
redisctl cloud acl create-role --name "app-role" \
  --redis-rules '[{"rule_id": 123}, {"rule_id": 456}]' --wait
```

### Update Role
```bash
redisctl cloud acl update-role --id 456 \
  --name "updated-role" \
  --redis-rules '[{"rule_id": 789}]' --wait
```

### Delete Role
```bash
redisctl cloud acl delete-role --id 456 --force --wait
```

## ACL Users

### Create ACL User
```bash
redisctl cloud acl create-acl-user \
  --name "app-user" \
  --role "app-role" \
  --password "SecurePass123!" --wait
```

### Update ACL User
```bash
# Change role
redisctl cloud acl update-acl-user --id 789 \
  --role "admin-role" --wait

# Update password
redisctl cloud acl update-acl-user --id 789 \
  --password "NewSecurePass456!" --wait
```

### Delete ACL User
```bash
redisctl cloud acl delete-acl-user --id 789 --force --wait
```

## ACL Configuration Examples

### Redis ACL Rule Patterns

Common ACL rule patterns:

```bash
# Read-only access
"+@read"

# Read and write, no admin
"+@read +@write -@admin"

# Specific commands only
"+get +set +del"

# All commands except dangerous ones
"~* +@all -@dangerous"

# Key pattern restrictions
"~user:* +@all"
```

### Role Hierarchy Example

```bash
# 1. Create base rules
redisctl cloud acl create-redis-rule --name "read" --rule "+@read" --wait
redisctl cloud acl create-redis-rule --name "write" --rule "+@write" --wait
redisctl cloud acl create-redis-rule --name "admin" --rule "+@all" --wait

# 2. Create roles with rules
redisctl cloud acl create-role --name "viewer" --redis-rules "1" --wait
redisctl cloud acl create-role --name "editor" --redis-rules '[{"rule_id": 1}, {"rule_id": 2}]' --wait
redisctl cloud acl create-role --name "admin" --redis-rules "3" --wait

# 3. Create users with roles
redisctl cloud acl create-acl-user --name "bob" --role "viewer" --password "pass1" --wait
redisctl cloud acl create-acl-user --name "alice" --role "editor" --password "pass2" --wait
redisctl cloud acl create-acl-user --name "admin" --role "admin" --password "pass3" --wait
```

## Managing ACL at Scale

### Bulk User Creation
```bash
#!/bin/bash
# Create multiple users from CSV
while IFS=',' read -r username role password; do
  redisctl cloud acl create-acl-user \
    --name "$username" \
    --role "$role" \
    --password "$password" \
    --wait &
done < users.csv

wait
echo "All users created!"
```

### ACL Audit
```bash
# List all rules
redisctl cloud acl list-redis-rules -o table

# List all roles with their rules
redisctl cloud acl list-roles -o json | jq '.[] | {name, rules}'

# List all users with their roles
redisctl cloud acl list-acl-users -o table
```

## Best Practices

### Security Guidelines
1. **Principle of Least Privilege**: Grant minimum required permissions
2. **Regular Audits**: Review ACL rules and users regularly
3. **Strong Passwords**: Use complex passwords for ACL users
4. **Role-Based Access**: Use roles instead of direct rule assignments
5. **Key Patterns**: Restrict access to specific key patterns when possible

### ACL Migration Example
```bash
#!/bin/bash
# Export ACL configuration
redisctl cloud acl list-redis-rules -o json > rules.json
redisctl cloud acl list-roles -o json > roles.json
redisctl cloud acl list-acl-users -o json > users.json

# Import to new environment
# ... (parse and recreate with --wait flags)
```