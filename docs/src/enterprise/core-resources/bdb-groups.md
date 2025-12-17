# Database Groups

Database groups (BDB groups) allow you to organize and manage related databases as a single unit in Redis Enterprise. This is useful for grouping databases that belong to the same application, tenant, or environment.

## Overview

Database groups provide a way to:
- Organize databases logically by application or purpose
- Apply bulk operations to related databases
- Simplify management of multi-database deployments
- Track and monitor groups of databases together

## Available Commands

### List Database Groups

List all database groups in the cluster:

```bash
# List all groups
redisctl enterprise bdb-group list

# List groups with specific fields
redisctl enterprise bdb-group list -q "[].{uid: uid, databases: bdbs}"

# Output as table
redisctl enterprise bdb-group list -o table
```

### Get Group Details

Get detailed information about a specific database group:

```bash
# Get group by UID
redisctl enterprise bdb-group get <uid>

# Get specific fields
redisctl enterprise bdb-group get <uid> -q "bdbs"
```

### Create Database Group

Create a new database group:

```bash
# Create from JSON file
redisctl enterprise bdb-group create --data @group.json

# Create from stdin
echo '{"name": "web-app-group"}' | redisctl enterprise bdb-group create --data -

# Create with initial databases
redisctl enterprise bdb-group create --data '{"name": "api-group", "bdbs": [1, 2, 3]}'
```

### Update Database Group

Update an existing database group:

```bash
# Update from JSON file
redisctl enterprise bdb-group update <uid> --data @updates.json

# Update inline
redisctl enterprise bdb-group update <uid> --data '{"name": "new-name"}'
```

### Delete Database Group

Delete a database group:

```bash
# Delete with confirmation
redisctl enterprise bdb-group delete <uid>

# Delete without confirmation
redisctl enterprise bdb-group delete <uid> --force
```

### Manage Group Membership

Add or remove databases from a group:

```bash
# Add database to group
redisctl enterprise bdb-group add-database <group_uid> --database <bdb_uid>

# Remove database from group
redisctl enterprise bdb-group remove-database <group_uid> --database <bdb_uid>

# List databases in group
redisctl enterprise bdb-group list-databases <group_uid>
```

## Group Configuration

### Basic Group Structure

```json
{
  "uid": 1,
  "name": "production-group",
  "bdbs": [1, 2, 3, 4],
  "description": "Production application databases"
}
```

### Creating Groups

When creating a group, the UID is auto-assigned by the cluster:

```json
{
  "name": "staging-group",
  "bdbs": [],
  "description": "Staging environment databases"
}
```

## Use Cases

### Application Grouping

Group all databases for a specific application:

```bash
# Create application group
redisctl enterprise bdb-group create --data '{
  "name": "ecommerce-app",
  "description": "E-commerce platform databases"
}'

# Add databases to the group
redisctl enterprise bdb-group add-database 1 --database 10  # Session store
redisctl enterprise bdb-group add-database 1 --database 11  # Product cache
redisctl enterprise bdb-group add-database 1 --database 12  # Shopping cart
```

### Environment Separation

Organize databases by environment:

```bash
# Create environment groups
redisctl enterprise bdb-group create --data '{"name": "dev-databases"}'
redisctl enterprise bdb-group create --data '{"name": "staging-databases"}'
redisctl enterprise bdb-group create --data '{"name": "production-databases"}'

# Add databases to appropriate groups
for db in 1 2 3; do
  redisctl enterprise bdb-group add-database 1 --database $db  # Dev
done

for db in 4 5 6; do
  redisctl enterprise bdb-group add-database 2 --database $db  # Staging
done
```

### Multi-Tenant Organization

Group databases by tenant:

```bash
# Create tenant groups
redisctl enterprise bdb-group create --data '{
  "name": "tenant-acme",
  "description": "ACME Corp databases"
}'

redisctl enterprise bdb-group create --data '{
  "name": "tenant-globex",
  "description": "Globex Inc databases"
}'
```

## Practical Examples

### Bulk Operations Script

Perform operations on all databases in a group:

```bash
#!/bin/bash
# Get all databases in a group
GROUP_UID=1
DATABASES=$(redisctl enterprise bdb-group get $GROUP_UID -q "bdbs[]")

# Perform operation on each database
for db in $DATABASES; do
  echo "Processing database $db..."
  redisctl enterprise database get $db
done
```

### Group Health Check

Monitor all databases in a group:

```bash
# Get group databases and check each
GROUP_UID=1
for db_uid in $(redisctl enterprise bdb-group list-databases $GROUP_UID -q '[].uid' --raw); do
  echo "Checking database $db_uid..."
  redisctl enterprise database get $db_uid -q "{uid: uid, status: status}"
done
```

### Migration Helper

Move databases between groups:

```bash
# Move database from one group to another
move_database() {
  local db_uid=$1
  local from_group=$2
  local to_group=$3
  
  # Remove from old group
  redisctl enterprise bdb-group remove-database $from_group --database $db_uid
  
  # Add to new group
  redisctl enterprise bdb-group add-database $to_group --database $db_uid
  
  echo "Moved database $db_uid from group $from_group to $to_group"
}

# Usage
move_database 5 1 2
```

### Group Report

Generate a report of all groups and their databases:

```bash
# Generate group report
for group_uid in $(redisctl enterprise bdb-group list -q '[].uid' --raw); do
  name=$(redisctl enterprise bdb-group get $group_uid -q 'name')
  db_count=$(redisctl enterprise bdb-group get $group_uid -q 'length(bdbs)')
  
  echo "Group $group_uid: $name ($db_count databases)"
  for db_uid in $(redisctl enterprise bdb-group get $group_uid -q 'bdbs[]' --raw); do
    db_name=$(redisctl enterprise database get $db_uid -q 'name')
    echo "  - Database $db_uid: $db_name"
  done
  echo
done
```

## Best Practices

1. **Logical Organization** - Group databases by application, environment, or purpose
2. **Naming Conventions** - Use clear, descriptive names for groups
3. **Documentation** - Include descriptions to explain group purpose
4. **Regular Review** - Periodically review group membership
5. **Avoid Overlaps** - Each database should typically belong to one logical group
6. **Use for Bulk Operations** - Leverage groups for maintenance and monitoring

## Limitations

- Database groups are a logical organization feature
- They don't affect database performance or configuration
- Groups don't enforce any policies on member databases
- A database can belong to multiple groups
- Deleting a group doesn't delete the databases

## Troubleshooting

### Group Creation Fails

```bash
# Check cluster status
redisctl enterprise cluster get -q 'cluster_state'

# Verify required fields
redisctl enterprise api get /v1/jsonschema -q 'bdb_group'
```

### Database Not Added to Group

```bash
# Verify database exists
redisctl enterprise database get <bdb_uid>

# Check current group membership
redisctl enterprise bdb-group get <group_uid> -q "bdbs"
```

### Group Operations Slow

```bash
# Check number of databases in group
redisctl enterprise bdb-group get <group_uid> -q "bdbs | length"

# Consider splitting large groups
```

## Related Commands

- `enterprise database` - Individual database management
- `enterprise cluster` - Cluster-wide operations
- `enterprise stats` - Statistics for grouped databases