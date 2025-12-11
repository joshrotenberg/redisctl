# Common Recipes

> **Note:** We're developing a powerful workflow system that will automate many of these common tasks. Soon you'll be able to run pre-built workflows like `redisctl workflow run create-ha-database` instead of manually executing multiple commands. Stay tuned!

This page will contain common recipes and patterns for using redisctl effectively. In the meantime, check out:

## Quick Examples

### Create a High-Availability Database
```bash
# Coming soon as a workflow!
# redisctl workflow run create-ha-database --name prod-cache --size 2gb

# Current manual process:
redisctl cloud database create --data '{
  "name": "prod-cache",
  "memoryLimitInGb": 2,
  "replication": true,
  "dataPersistence": "aof-every-1-second"
}' --wait
```

### Migrate Database Between Regions
```bash
# Coming soon as a workflow!
# redisctl workflow run migrate-database --source 12345 --target-region us-west-2

# Current manual process involves multiple steps...
```

### Set Up Monitoring
```bash
# Coming soon as a workflow!
# redisctl workflow run setup-monitoring --database 12345 --prometheus-url http://prometheus:9090
```

## Workflow System Preview

The upcoming workflow system will provide:

- **Pre-built workflows** for common operations
- **Custom workflow definitions** in YAML/JSON
- **Parameterized templates** for reusable patterns
- **Conditional logic** and error handling
- **Progress tracking** with detailed output
- **Rollback capabilities** for safety

Example workflow definition (coming soon):
```yaml
name: create-ha-database
description: Create a high-availability database with best practices
parameters:
  - name: database_name
    required: true
  - name: size_gb
    default: 1
  - name: region
    default: us-east-1

steps:
  - name: create_subscription
    command: cloud subscription create
    data:
      name: "{{ database_name }}-subscription"
      
  - name: create_database
    command: cloud database create
    data:
      name: "{{ database_name }}"
      memoryLimitInGb: "{{ size_gb }}"
      replication: true
      dataPersistence: aof-every-1-second
    wait: true
    
  - name: configure_alerts
    command: cloud database alert create
    data:
      threshold: 80
      metric: memory-usage
```

## Current Best Practices

Until workflows are available, here are some patterns:

### Use JSON Files for Complex Operations
```bash
# Save configuration in files
cat > database.json <<EOF
{
  "name": "production-db",
  "memoryLimitInGb": 4,
  "replication": true
}
EOF

redisctl cloud database create --data @database.json --wait
```

### Chain Commands with Shell Scripts
```bash
#!/bin/bash
# Create database and wait for completion
DB_ID=$(redisctl cloud database create --data @config.json --wait -o json | jq -r '.resourceId')

# Configure ACL
redisctl cloud acl create --database $DB_ID --data @acl.json

# Set up monitoring
redisctl cloud metrics enable --database $DB_ID
```

### Use Profiles for Different Environments
```bash
# Development
redisctl --profile dev database list

# Staging  
redisctl --profile staging database list

# Production
redisctl --profile prod database list
```

## See Also

- [Managing Production Databases](production-databases.md)
- [CI/CD Integration](cicd.md)
- [Best Practices](../reference/best-practices.md)
- Async Operations