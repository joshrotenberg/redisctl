# Troubleshooting

Common issues and solutions for `redisctl`.

## Authentication Issues

### Cloud: 401 Unauthorized

```bash
# Error: Authentication failed: 401 Unauthorized
```

**Solution**: Check your API credentials:
```bash
# Verify environment variables are set
echo $REDIS_CLOUD_API_KEY
echo $REDIS_CLOUD_API_SECRET

# Test with explicit credentials
redisctl api cloud get /account \
  --api-key "your-key" \
  --api-secret "your-secret"
```

### Enterprise: Connection Refused

```bash
# Error: Connection refused
```

**Solution**: Verify cluster URL and port:
```bash
# Test connection
curl -k https://your-cluster:9443/v1/cluster

# Check environment
echo $REDIS_ENTERPRISE_URL
```

### Enterprise: Certificate Error

```bash
# Error: Certificate verify failed
```

**Solution**: For self-signed certificates:
```bash
# Allow insecure connections
export REDIS_ENTERPRISE_INSECURE=true

# Or in config file
[profiles.dev]
insecure = true
```

## Configuration Issues

### Profile Not Found

```bash
# Error: Profile 'production' not found
```

**Solution**: Check available profiles:
```bash
# List profiles
redisctl profile list

# Check config file exists
ls -la ~/.config/redisctl/config.toml
```

### Config File Not Found

```bash
# Error: Configuration file not found
```

**Solution**: Create the configuration:
```bash
# Create directory
mkdir -p ~/.config/redisctl

# Create config file
cat > ~/.config/redisctl/config.toml << EOF
[profiles.default]
deployment_type = "cloud"
api_key = "your-key"
api_secret = "your-secret"
EOF
```

## Command Issues

### Command Not Found

```bash
# Error: unrecognized subcommand 'auth'
```

**Note**: Some commands shown in documentation may not be implemented yet:
- `auth setup` - Not implemented
- `auth test` - Not implemented
- `profile set` - Not fully implemented
- `profile remove` - Not fully implemented

**Solution**: Use raw API access instead:
```bash
# Instead of auth test, use:
redisctl api cloud get /account
redisctl api enterprise get /v1/cluster
```

### Invalid Flag

```bash
# Error: unexpected argument '--deployment-type'
```

**Solution**: Use correct flag names:
```bash
# Correct flag is --deployment (not --deployment-type)
redisctl profile set myprofile --deployment cloud
```

## API Issues

### Endpoint Not Found

```bash
# Error: 404 Not Found
```

**Solution**: Check endpoint path:
```bash
# Cloud endpoints start with /
redisctl api cloud get /subscriptions  # Correct

# Enterprise endpoints start with /v1/
redisctl api enterprise get /v1/bdbs    # Correct
```

### Invalid JSON

```bash
# Error: Invalid JSON in request body
```

**Solution**: Validate JSON:
```bash
# Check JSON syntax
echo '{"name": "test"}' | jq .

# Use file for complex JSON
cat > database.json << EOF
{
  "name": "my-database",
  "memory_size": 1073741824
}
EOF
redisctl api enterprise post /v1/bdbs --data @database.json
```

## Debugging

### Enable Debug Logging

```bash
# See detailed request/response
export RUST_LOG=debug
redisctl api cloud get /account

# Even more detail
export RUST_LOG=trace
```

### Check Version

```bash
# Verify installation
redisctl --version
```

### Test Connectivity

```bash
# Test Cloud
curl -X GET https://api.redislabs.com/v1/account \
  -H "x-api-key: your-key" \
  -H "x-api-secret: your-secret"

# Test Enterprise
curl -k -u admin@cluster.local:password \
  https://cluster:9443/v1/cluster
```

## Common Errors

### Rate Limiting

```bash
# Error: 429 Too Many Requests
```

**Solution**: Add delay between requests:
```bash
for id in $(seq 1 10); do
  redisctl api cloud get /subscriptions/$id
  sleep 1  # Add delay
done
```

### Timeout

```bash
# Error: Request timeout
```

**Solution**: Increase timeout or check network:
```bash
# Check network connectivity
ping cluster.example.com

# Use verbose mode to see where it hangs
RUST_LOG=debug redisctl api enterprise get /v1/cluster
```

## Getting Help

1. Check command help:
   ```bash
   redisctl --help
   redisctl cloud --help
   redisctl api --help
   ```

2. Enable debug logging:
   ```bash
   export RUST_LOG=debug
   ```

3. Check the [GitHub issues](https://github.com/joshrotenberg/redisctl/issues)

4. File a bug report with:
   - Command that failed
   - Error message
   - Debug output (`RUST_LOG=debug`)
   - Version (`redisctl --version`)