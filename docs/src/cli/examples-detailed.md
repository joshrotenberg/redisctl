# redisctl Examples

Comprehensive examples for common Redis Cloud and Enterprise operations.

## Table of Contents

- [Authentication & Configuration](#authentication-configuration)
- [Database Operations](#database-operations)
- [Cluster Management](#cluster-management)
- [User & ACL Management](#user-acl-management)
- [Backup & Recovery](#backup-recovery)
- [Monitoring & Metrics](#monitoring-metrics)
- [Networking & Security](#networking-security)
- [Advanced Workflows](#advanced-workflows)

## Authentication & Configuration

### Setting up profiles

```bash
# Create a Redis Cloud profile
redisctl profile set cloud-prod \
  --deployment-type cloud \
  --api-key YOUR_API_KEY \
  --api-secret YOUR_API_SECRET

# Create a Redis Enterprise profile
redisctl profile set enterprise-dev \
  --deployment-type enterprise \
  --url https://cluster.example.com:9443 \
  --username admin@example.com \
  --password SecurePassword123

# Set default profile
redisctl profile default cloud-prod

# List all profiles
redisctl profile list
```

### Using environment variables

```bash
# Redis Cloud
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"

# Redis Enterprise
export REDIS_ENTERPRISE_URL="https://cluster:9443"
export REDIS_ENTERPRISE_USER="admin@example.com"
export REDIS_ENTERPRISE_PASSWORD="password"
export REDIS_ENTERPRISE_INSECURE="true"  # For self-signed certificates
```

## Database Operations

### Creating databases

```bash
# Create a Redis Cloud database
redisctl cloud database create \
  --subscription-id 12345 \
  --name "production-cache" \
  --memory-limit-gb 10 \
  --throughput-measurement-by operations-per-second \
  --throughput-measurement-value 10000

# Create a Redis Enterprise database
redisctl enterprise database create cache-db \
  --memory-limit 1024 \
  --modules search,json

# Create database with specific configuration
redisctl enterprise database create session-store \
  --memory-limit 2048 \
  --replication \
  --persistence aof \
  --eviction-policy allkeys-lru
```

### Listing and filtering databases

```bash
# List all databases
redisctl database list

# List with table output
redisctl database list -o table

# Filter active databases using JMESPath
redisctl database list -q "[?status=='active'].{name:name,memory:memory_size,port:port}"

# Show specific database details
redisctl enterprise database show 1 -o yaml

# Get database endpoints
redisctl cloud database list \
  --subscription-id 12345 \
  -q "[].{name:name,endpoint:public_endpoint}"
```

### Database updates

```bash
# Update memory limit
redisctl enterprise database update 1 \
  --memory-limit 2048

# Enable replication
redisctl cloud database update \
  --subscription-id 12345 \
  --database-id 67890 \
  --replication true

# Change eviction policy
redisctl enterprise database update cache-db \
  --eviction-policy volatile-lru
```

## Cluster Management

### Initialize Enterprise cluster

```bash
# Bootstrap new cluster
redisctl enterprise bootstrap create \
  --name "production-cluster" \
  --username admin@company.com \
  --password SecurePassword123

# Check bootstrap status
redisctl enterprise bootstrap status

# Get cluster information
redisctl enterprise cluster info -o table

# Update cluster settings
redisctl enterprise cluster update \
  --name "production-cluster-v2"
```

### Node management

```bash
# List all nodes
redisctl enterprise node list -o table

# Show node details
redisctl enterprise node show 1

# Join node to cluster
redisctl enterprise node join \
  --cluster-url https://master:9443 \
  --username admin \
  --password password

# Remove node from cluster
redisctl enterprise node remove 3
```

## User & ACL Management

### User management

```bash
# Create user (Cloud)
redisctl cloud user create \
  --email developer@company.com \
  --first-name John \
  --last-name Doe \
  --role db-member

# Create user (Enterprise)
redisctl enterprise user create \
  --email ops@company.com \
  --password TempPass123 \
  --role db-viewer

# List users with specific roles
redisctl user list -q "[?role=='admin'].email"

# Update user role
redisctl enterprise user update ops@company.com \
  --role admin
```

### ACL management

```bash
# Create ACL rule (Cloud)
redisctl cloud acl create \
  --subscription-id 12345 \
  --database-id 67890 \
  --name "read-only-acl" \
  --rule "+@read ~*"

# Create Redis ACL (Enterprise)
redisctl enterprise redis-acl create \
  --name "app-acl" \
  --acl "+@all -@dangerous ~app:*"

# List ACLs
redisctl cloud acl list \
  --subscription-id 12345 \
  --database-id 67890 \
  -o table

# Associate ACL with user
redisctl cloud acl-user create \
  --subscription-id 12345 \
  --database-id 67890 \
  --username app-user \
  --password SecurePass123 \
  --acl-id acl-123
```

## Backup & Recovery

### Creating backups

```bash
# Create Cloud backup
redisctl cloud backup create \
  --subscription-id 12345 \
  --database-id 67890

# Create Enterprise backup
redisctl enterprise database backup 1

# List backups
redisctl cloud backup list \
  --subscription-id 12345 \
  --database-id 67890 \
  -o table

# Backup with custom location (Enterprise)
redisctl enterprise database export 1 \
  --export-location s3://bucket/backups/
```

### Restoring from backup

```bash
# Restore Cloud database
redisctl cloud backup restore \
  --subscription-id 12345 \
  --database-id 67890 \
  --backup-id backup-123

# Import data (Enterprise)
redisctl enterprise database import 1 \
  --source-url redis://source-cluster:6379 \
  --source-password SourcePass123
```

## Monitoring & Metrics

### Getting metrics

```bash
# Cloud database metrics
redisctl cloud metrics database \
  --subscription-id 12345 \
  --database-id 67890 \
  --metric used-memory \
  --from "2024-01-01T00:00:00Z" \
  --to "2024-01-02T00:00:00Z"

# Enterprise statistics
redisctl enterprise stats database 1 \
  --metric all \
  --interval 1hour

# Cluster statistics
redisctl enterprise stats cluster \
  --metric cpu,memory,network
```

### Monitoring alerts

```bash
# List alerts (Enterprise)
redisctl enterprise alert list -o table

# Get alert details
redisctl enterprise alert show alert-123

# Clear alert
redisctl enterprise alert clear alert-123

# Set alert settings
redisctl enterprise alert settings \
  --database-id 1 \
  --threshold-memory 90 \
  --threshold-cpu 80
```

### Viewing logs

```bash
# Cloud system logs
redisctl cloud logs list \
  --limit 100 \
  -q "[?level=='error'].{time:timestamp,message:message}"

# Enterprise event logs
redisctl enterprise logs list \
  --severity error \
  --limit 50 \
  -o table
```

## Networking & Security

### VPC Peering (Cloud)

```bash
# Create VPC peering
redisctl cloud peering create \
  --subscription-id 12345 \
  --aws-account-id 123456789012 \
  --region us-east-1 \
  --vpc-id vpc-12345 \
  --vpc-cidr 10.0.0.0/16

# List peerings
redisctl cloud peering list \
  --subscription-id 12345 \
  -o table

# Accept peering
redisctl cloud peering accept \
  --subscription-id 12345 \
  --peering-id peer-123
```

### Transit Gateway (Cloud)

```bash
# Create Transit Gateway attachment
redisctl cloud transit-gateway create \
  --subscription-id 12345 \
  --aws-account-id 123456789012 \
  --tgw-id tgw-12345 \
  --cidrs "10.0.0.0/24,10.0.1.0/24"

# List attachments
redisctl cloud transit-gateway list \
  --subscription-id 12345 \
  -q "[?status=='active'].id"
```

### Private Service Connect (Cloud)

```bash
# Create Private Service Connect
redisctl cloud private-service-connect create \
  --subscription-id 12345 \
  --gcp-project-id my-project \
  --service-attachment sa-12345
```

## Advanced Workflows

### Database migration

```bash
# Export from source database
redisctl enterprise database export 1 \
  --export-location /tmp/backup.rdb

# Import to target database
redisctl enterprise database import 2 \
  --source-url file:///tmp/backup.rdb
```

### Active-Active (CRDB) setup

```bash
# Create Active-Active database (Cloud)
redisctl cloud crdb create \
  --subscription-id 12345 \
  --name "global-cache" \
  --memory-limit-gb 10 \
  --regions us-east-1,eu-west-1,ap-southeast-1

# Create Active-Active database (Enterprise)
redisctl enterprise crdb create \
  --name "multi-region-db" \
  --memory-limit 1024 \
  --participating-clusters cluster1,cluster2,cluster3
```

### Module management

```bash
# List available modules
redisctl enterprise module list -o table

# Upload custom module
redisctl enterprise module upload \
  --name custom-module \
  --version 1.0.0 \
  --file /path/to/module.so

# Add module to database
redisctl enterprise database update 1 \
  --modules search,json,custom-module
```

### License management

```bash
# Get license information
redisctl enterprise license get -o yaml

# Update license
redisctl enterprise license update \
  --license-file /path/to/license.key

# Check license expiration
redisctl enterprise license get \
  -q "expiration_date"
```

## Scripting & Automation

### Batch operations

```bash
# Delete all inactive databases
for db in $(redisctl database list -q "[?status=='inactive'].uid" -o json | jq -r '.[]'); do
  redisctl enterprise database delete $db --force
done

# Backup all databases
redisctl database list -q "[].uid" | while read db_id; do
  echo "Backing up database $db_id"
  redisctl enterprise database backup $db_id
done
```

### Health checks

```bash
# Check cluster health
if redisctl enterprise cluster info -q "alert_count" | grep -q "0"; then
  echo "Cluster healthy"
else
  echo "Cluster has alerts"
  redisctl enterprise alert list -o table
fi

# Monitor database memory usage
redisctl database list -q "[?used_memory_percent>90].{name:name,usage:used_memory_percent}" -o table
```

### CI/CD integration

```bash
# Deploy new database in CI pipeline
#!/bin/bash
set -e

# Create database
DB_ID=$(redisctl enterprise database create staging-db \
  --memory-limit 512 \
  --output json \
  -q "uid")

echo "Created database: $DB_ID"

# Wait for database to be active
while [ "$(redisctl enterprise database show $DB_ID -q status)" != "active" ]; do
  echo "Waiting for database to be active..."
  sleep 5
done

echo "Database $DB_ID is active"

# Get connection details
ENDPOINT=$(redisctl enterprise database show $DB_ID -q endpoint)
echo "Database endpoint: $ENDPOINT"
```

## Docker Usage

### Using Docker image

```bash
# Run command with Docker
docker run --rm \
  -e REDIS_CLOUD_API_KEY="your-key" \
  -e REDIS_CLOUD_API_SECRET="your-secret" \
  joshrotenberg/redisctl:latest \
  cloud subscription list

# Interactive shell
docker run -it --rm \
  -e REDIS_ENTERPRISE_URL="https://cluster:9443" \
  -e REDIS_ENTERPRISE_USER="admin" \
  -e REDIS_ENTERPRISE_PASSWORD="password" \
  --entrypoint /bin/sh \
  joshrotenberg/redisctl:latest
```

### Docker Compose development

```bash
# Start local Redis Enterprise for testing
docker compose up -d

# Run CLI against local cluster
docker compose exec init-cluster redisctl enterprise cluster info

# Create test database
docker compose exec create-db redisctl enterprise database create test
```

## Troubleshooting

### Debug output

```bash
# Enable debug logging
RUST_LOG=debug redisctl database list

# Verbose output
redisctl -vvv cloud subscription list

# Show raw API responses
redisctl cloud api GET /subscriptions
```

### Common issues

```bash
# SSL certificate issues (Enterprise)
export REDIS_ENTERPRISE_INSECURE=true
redisctl enterprise cluster info

# Authentication failures
# Check credentials
redisctl profile get current

# Test API access directly
redisctl enterprise api GET /v1/cluster
```

## Additional Resources

- [Full Command Reference](cli-reference/index.md)
- [API Documentation](https://docs.redis.com/latest/rs/references/rest-api/)
- [GitHub Repository](https://github.com/joshrotenberg/redisctl)
- [Rust Library Documentation](https://docs.rs/redisctl)