# Cloud Workflows

Workflows orchestrate multiple API calls to accomplish common tasks. This guide covers the available Cloud workflows.

## Subscription Setup Workflow

The `subscription-setup` workflow creates a complete Redis Cloud subscription with optional database configuration.

### Basic Usage

```bash
# Create a subscription with default settings
redisctl cloud workflow subscription-setup \
  --name "my-subscription" \
  --wait

# Create with custom configuration
redisctl cloud workflow subscription-setup \
  --name "production" \
  --provider AWS \
  --region us-west-2 \
  --database-name "main-db" \
  --database-memory-gb 2 \
  --wait
```

### Options

- `--name`: Subscription name (default: redisctl-test)
- `--provider`: Cloud provider - AWS, GCP, or Azure (default: AWS)
- `--region`: Cloud region (default: us-east-1)
- `--payment-method-id`: Payment method ID (auto-detected if not specified)
- `--database-name`: Database name (default: default-db)
- `--database-memory-gb`: Database memory in GB (default: 1)
- `--database-throughput`: Operations per second (default: 1000)
- `--modules`: Comma-separated list of modules (e.g., "RedisJSON,RediSearch")
- `--high-availability`: Enable HA replication
- `--data-persistence`: Enable data persistence (default: true)
- `--skip-database`: Only create subscription without database
- `--wait`: Wait for operations to complete (default: true)
- `--wait-timeout`: Maximum wait time in seconds (default: 600)
- `--wait-interval`: Polling interval in seconds (default: 10)
- `--dry-run`: Preview what would be created without executing

### What It Does

1. **Validates payment method**: Looks up your account's payment method
2. **Creates subscription**: Provisions infrastructure in the specified cloud/region
3. **Creates database**: Sets up a Redis database with your configuration
4. **Waits for completion**: Monitors async operations until resources are ready
5. **Returns connection details**: Provides endpoints and credentials

### Output Formats

```bash
# Human-readable output (default)
redisctl cloud workflow subscription-setup --name "test"

# JSON output for automation
redisctl cloud workflow subscription-setup --name "test" --output json

# YAML output
redisctl cloud workflow subscription-setup --name "test" --output yaml
```

### Example JSON Output

```json
{
  "success": true,
  "message": "Subscription setup completed successfully",
  "outputs": {
    "subscription_id": 12345,
    "subscription_name": "test",
    "database_id": 67890,
    "database_name": "default-db",
    "connection_string": "redis://redis-12345.c1.us-east-1.ec2.cloud.redislabs.com:12345",
    "provider": "AWS",
    "region": "us-east-1",
    "status": "active"
  }
}
```

### Use Cases

1. **Quick Development Environment**
   ```bash
   redisctl cloud workflow subscription-setup \
     --name "dev-env" \
     --database-memory-gb 0.1 \
     --wait
   ```

2. **Production Setup with Modules**
   ```bash
   redisctl cloud workflow subscription-setup \
     --name "production" \
     --database-memory-gb 10 \
     --modules "RedisJSON,RediSearch,RedisTimeSeries" \
     --high-availability \
     --wait
   ```

3. **Multi-Region Preparation** (subscription only)
   ```bash
   redisctl cloud workflow subscription-setup \
     --name "global-app" \
     --region eu-west-1 \
     --skip-database \
     --wait
   ```

## Future Workflows

Additional workflows are planned:

- **active-active-setup**: Multi-region Active-Active configuration
- **database-migration**: Migrate databases between subscriptions
- **acl-setup**: Configure comprehensive ACL security

See issue tracker for workflow development status.