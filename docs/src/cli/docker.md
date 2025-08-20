# Docker Testing

The CLI can be easily tested using our Docker environment, which provides a complete Redis Enterprise setup without manual configuration.

## Quick CLI Testing

### Interactive Session

```bash
# Start Redis Enterprise environment
make docker-up

# Launch interactive CLI container
make docker-cli

# Inside the container, all commands work:
redis-enterprise cluster info --insecure
redis-enterprise database list --insecure --output table
redis-enterprise workflow create-database --name test --db-type cache --insecure
```

### One-off Commands

```bash
# Run single commands without entering container
docker compose exec cli redis-enterprise cluster info --insecure
docker compose exec cli redis-enterprise database list --insecure --output json
```

## Testing Workflows

### Database Creation Workflows

```bash
# Test all database types
make docker-all-dbs

# Or manually test specific types
docker compose exec cli redis-enterprise workflow create-database \
  --name cache-example --db-type cache --insecure

docker compose exec cli redis-enterprise workflow create-database \
  --name search-example --db-type search --insecure
```

### Cluster Bootstrap Workflow

```bash
# The environment automatically tests cluster initialization
# But you can test bootstrap status
docker compose exec cli redis-enterprise bootstrap status --insecure
```

## Testing Output Formats

### JSON Output

```bash
docker compose exec cli redis-enterprise database list --insecure --output json
```

### YAML Output

```bash
docker compose exec cli redis-enterprise cluster info --insecure --output yaml
```

### Table Output

```bash
docker compose exec cli redis-enterprise database list --insecure --output table
docker compose exec cli redis-enterprise node list --insecure --output table
```

## Performance Testing

### Load Testing

```bash
# Run performance tests
make docker-perf

# Or manually test performance
docker compose exec cli sh -c '
for i in $(seq 1 100); do
  redis-enterprise database list --insecure --output json > /dev/null
  echo "Request $i completed"
done
'
```

### Response Time Testing

```bash
docker compose exec cli sh -c '
start=$(date +%s.%N)
redis-enterprise cluster info --insecure --output json > /dev/null
end=$(date +%s.%N)
echo "Request took $(echo "$end - $start" | bc) seconds"
'
```

## Debugging CLI Issues

### Verbose Logging

```bash
# Set debug logging
docker compose exec cli env RUST_LOG=debug redis-enterprise cluster info --insecure

# Maximum verbosity
docker compose exec cli env RUST_LOG=trace redis-enterprise database create \
  --name debug-test --memory 100MB --insecure
```

### Debug Container

```bash
# Start debug environment with verbose logging
make docker-debug

# Inside debug container:
redis-enterprise --verbose cluster info --insecure
```

### Network Debugging

```bash
# Test connectivity to Redis Enterprise
docker compose exec cli ping enterprise
docker compose exec cli curl -k https://enterprise:9443/v1/bootstrap

# Test DNS resolution
docker compose exec cli nslookup enterprise
```

## Testing Custom Builds

### Rebuild and Test

```bash
# Make changes to CLI code, then:
docker compose build cli
docker compose up --force-recreate cli

# Or use development container with live mounting
make docker-debug
# Make changes to code on host, test immediately in container
```

### Testing Unreleased Features

```bash
# Build from current source
docker compose build --no-cache cli

# Test new commands
docker compose exec cli redis-enterprise your-new-command --help
```

## Environment Variables

### Authentication

The Docker environment pre-configures these variables:

```bash
REDIS_ENTERPRISE_URL=https://enterprise:9443
REDIS_ENTERPRISE_USER=admin@redis.local  
REDIS_ENTERPRISE_PASSWORD=Redis123!
```

### Logging

```bash
# Run commands with different log levels
docker compose exec cli env RUST_LOG=info redis-enterprise cluster info --insecure
docker compose exec cli env RUST_LOG=debug redis-enterprise database list --insecure
docker compose exec cli env RUST_LOG=trace redis-enterprise node list --insecure
```

## Automated Testing

### Test Runner

```bash
# Run comprehensive test suite
make docker-test

# This tests:
# - All cluster commands
# - Database CRUD operations  
# - Node and user management
# - Direct API access
# - Output format validation
```

### Custom Test Scripts

```bash
# Create and run custom test scripts
docker compose exec cli sh -c '
echo "Testing database lifecycle..." &&
redis-enterprise database create --name lifecycle-test --memory 100MB --insecure &&
redis-enterprise database get lifecycle-test --insecure &&
redis-enterprise database delete lifecycle-test --yes --insecure &&
echo "Lifecycle test completed successfully"
'
```

## Clean Up

### Remove Test Data

```bash
# Remove all test databases
make docker-cleanup

# Or manually clean up
docker compose exec cli redis-enterprise database list --insecure --output json | \
jq -r '.[].name' | while read db; do
  echo "Deleting database: $db"
  docker compose exec cli redis-enterprise database delete "$db" --yes --insecure
done
```

### Reset Environment

```bash
# Complete reset
make docker-down
make docker-up

# Or just restart services
docker compose restart
```

## Tips and Best Practices

### Development Workflow

1. **Start with Docker**: Always test in Docker first
2. **Use profiles**: Use specific profiles for specific testing needs
3. **Check logs**: Monitor container logs for issues
4. **Test all formats**: Verify JSON, YAML, and Table outputs
5. **Clean up**: Remove test data between test runs

### Performance Considerations

- Use `--output json` for fastest performance
- Test with realistic data sizes
- Monitor memory usage during large operations
- Validate response times meet expectations

### Debugging Tips

- Use `--verbose` flag for detailed output
- Set `RUST_LOG=debug` for HTTP request/response logging  
- Check network connectivity if commands hang
- Verify credentials are correctly configured
- Test with `--insecure` flag for certificate issues