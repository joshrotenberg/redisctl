# Docker Testing Environment

This project includes comprehensive Docker Compose configurations for testing the feature-complete Redis Enterprise CLI against a real Redis Enterprise cluster.

## Quick Start

```bash
# Start Redis Enterprise and initialize cluster
make docker-up

# Run interactive CLI
make docker-cli

# Run comprehensive tests
make docker-test

# Clean up everything
make docker-down
```

## Available Docker Profiles

The Docker environment supports multiple profiles for different testing scenarios:

### Basic Profiles (docker-compose.yml)

| Profile | Description | Command |
|---------|-------------|---------|
| default | Start Redis Enterprise, initialize cluster, create test database | `docker compose up -d` |
| cli | Interactive CLI container for manual testing | `docker compose --profile cli up` |
| examples | Create example databases using workflows | `docker compose --profile examples up` |
| monitor | Continuous cluster monitoring | `docker compose --profile monitor up` |
| showcase | Demonstrate all CLI features | `docker compose --profile showcase up` |

### Development Profiles (docker-compose.dev.yml)

| Profile | Description | Command |
|---------|-------------|---------|
| test | Run comprehensive CLI tests | `docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile test up` |
| all-dbs | Create all database types | `docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile all-dbs up` |
| perf | Performance testing | `docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile perf up` |
| debug | Debug container with verbose logging | `docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile debug up` |
| cleanup | Remove all test databases | `docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile cleanup up` |
| integration | Full integration test suite | `docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile integration up` |

## Feature Showcase

The `showcase` profile demonstrates all implemented CLI features:

```bash
docker compose --profile showcase up
```

This will demonstrate:
- Cluster management with info and stats
- Database operations with JMESPath queries
- Node management and statistics
- User creation and listing
- Role and ACL management
- Module listing
- License information and usage
- Bootstrap status checking
- Configuration profile management
- Raw API access

## Comprehensive Testing

The `test` profile runs tests for all feature-complete endpoints:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile test up
```

Tests include:
1. **Cluster Commands** - info, stats, update
2. **Database Commands** - list, get, create, update, delete, stats, wait
3. **Node Commands** - list, get, stats
4. **User Commands** - list, get, create
5. **Bootstrap Commands** - status, raw
6. **Module Commands** - list, get, upload, update, delete
7. **Role Commands** - list, get, create, update, delete, users
8. **License Commands** - get, usage, update, validate, cluster
9. **Configuration Commands** - list, set, get, remove
10. **API Commands** - get, post, put, delete
11. **JMESPath Queries** - filtering and transformation
12. **Output Formats** - json, yaml, table

## Integration Testing

Run the full integration test suite:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile integration up
```

Integration tests verify:
- Complete database lifecycle (create, wait, update, stats, delete)
- User and role management workflows
- Configuration profile management
- Workflow commands for optimized database creation
- Error handling and recovery

## Interactive CLI Testing

Start an interactive shell with the CLI:

```bash
docker compose --profile cli run --rm cli
```

Inside the container, you can run any CLI command:

```bash
# List all databases with table output
redis-enterprise database list --insecure --output table

# Get cluster info with JMESPath query
redis-enterprise cluster info --insecure --query 'name'

# Create a new database
redis-enterprise database create --name test-db-2 --memory 100MB --port 12001 --insecure

# Check license usage
redis-enterprise license usage --insecure

# List all users
redis-enterprise user list --insecure --output table

# Use raw API access
redis-enterprise api get /v1/modules --insecure
```

## Database Type Examples

Create different database types using workflows:

```bash
# Cache database with LRU eviction
redis-enterprise workflow create-database --name cache-db --db-type cache --insecure

# Persistent database with AOF
redis-enterprise workflow create-database --name persistent-db --db-type persistent --insecure

# Search database with RediSearch module
redis-enterprise workflow create-database --name search-db --db-type search --insecure

# Time-series database
redis-enterprise workflow create-database --name timeseries-db --db-type timeseries --insecure
```

## Performance Testing

Run performance benchmarks:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile perf up
```

This runs 100 iterations each of:
- Database list operations
- Database get operations
- Measuring total execution time

## Debug Mode

For troubleshooting, use the debug profile with verbose logging:

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile debug up
```

Environment variables set:
- `RUST_LOG=debug,redis_enterprise=trace,redis_enterprise_cli=trace`
- `RUST_BACKTRACE=1`

## Monitoring

Continuous monitoring of cluster health:

```bash
docker compose --profile monitor up
```

Displays every 30 seconds:
- Cluster status
- Database list
- Node status
- License usage

## Environment Variables

Configure behavior with environment variables:

```bash
# Set log level
export RUST_LOG=debug
docker compose up

# Use custom Redis Enterprise image
export ENTERPRISE_IMAGE=redislabs/redis:latest
docker compose up
```

## Cleanup

Remove all test databases (except the default test-db):

```bash
docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile cleanup up
```

Complete cleanup:

```bash
# Stop and remove all containers, networks, volumes
docker compose down -v

# Or use make
make docker-clean
```

## Default Credentials

The test environment uses these default credentials:
- **URL**: https://enterprise:9443 (from containers) or https://localhost:9443 (from host)
- **Username**: admin@redis.local
- **Password**: Redis123!

## Tips for Testing

1. **Use profiles** to test specific functionality without running everything
2. **Check logs** with `docker compose logs <service-name>` for debugging
3. **Access web UI** at https://localhost:8443 with the same credentials
4. **Create databases** on ports 12000-12010 (exposed to host)
5. **Use JMESPath queries** to filter output for specific fields
6. **Save configurations** using the config commands for repeated testing

## Troubleshooting

### Container fails to start
- Check if ports 9443, 8443, or 12000-12010 are already in use
- Ensure Docker has enough resources allocated (at least 4GB RAM)

### CLI commands fail with connection errors
- Wait for the enterprise container to be fully healthy: `docker compose ps`
- Check logs: `docker compose logs enterprise`
- Verify the --insecure flag is used (self-signed certificates)

### Tests fail
- Ensure the cluster is fully initialized before running tests
- Check that the test-db was created successfully
- Review logs with `docker compose logs test-runner`

## CI/CD Integration

For CI/CD pipelines:

```bash
# Start services and run tests
docker compose up -d
docker compose --profile test up --exit-code-from test-runner

# Check exit code
if [ $? -eq 0 ]; then
  echo "All tests passed!"
else
  echo "Tests failed!"
  exit 1
fi

# Cleanup
docker compose down -v
```