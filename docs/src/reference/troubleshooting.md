# Troubleshooting

Solutions for common issues when using redisctl.

## Installation Issues

### Binary Not Found

**Problem:** `command not found: redisctl`

**Solutions:**
```bash
# Check if binary is in PATH
which redisctl

# Add to PATH (Linux/macOS)
export PATH="$PATH:/path/to/redisctl"
echo 'export PATH="$PATH:/path/to/redisctl"' >> ~/.bashrc

# Make executable
chmod +x /path/to/redisctl

# Verify installation
redisctl --version
```

### Permission Denied

**Problem:** `permission denied: redisctl`

**Solutions:**
```bash
# Make executable
chmod +x redisctl

# If installed system-wide
sudo chmod +x /usr/local/bin/redisctl

# Check ownership
ls -la $(which redisctl)
```

### SSL Certificate Errors

**Problem:** Certificate verification failed

**Solutions:**
```bash
# For self-signed certificates (Enterprise)
export REDIS_ENTERPRISE_INSECURE=true

# Update CA certificates (Linux)
sudo update-ca-certificates

# macOS
brew install ca-certificates
```

## Authentication Issues

### Invalid Credentials

**Problem:** `401 Unauthorized` or `Authentication failed`

**Diagnosis:**
```bash
# Test credentials directly
redisctl auth test --profile prod

# Check environment variables
env | grep REDIS

# Verify profile configuration
redisctl profile show prod
```

**Solutions:**
```bash
# Re-set credentials
redisctl profile set prod \
  --deployment cloud \
  --api-key "correct-key" \
  --api-secret "correct-secret"

# For Enterprise with special characters in password
redisctl profile set enterprise \
  --deployment enterprise \
  --url "https://cluster:9443" \
  --username "admin@domain.com" \
  --password 'p@$$w0rd!'  # Use single quotes
```

### Profile Not Found

**Problem:** `Profile 'name' not found`

**Solutions:**
```bash
# List available profiles
redisctl profile list

# Check config file location
redisctl profile path

# Create missing profile
redisctl profile set missing-profile \
  --deployment cloud \
  --api-key "$API_KEY" \
  --api-secret "$SECRET"

# Set default profile
redisctl profile default prod
```

### Environment Variable Issues

**Problem:** Environment variables not being read

**Solutions:**
```bash
# Export variables properly
export REDIS_CLOUD_API_KEY="key"
export REDIS_CLOUD_API_SECRET="secret"

# Check if set
echo $REDIS_CLOUD_API_KEY

# Use in same shell or source
source ~/.bashrc

# Debug with trace logging
RUST_LOG=trace redisctl cloud subscription list 2>&1 | grep -i env
```

## Connection Issues

### Network Timeout

**Problem:** `Connection timeout` or `Failed to connect`

**Diagnosis:**
```bash
# Test connectivity
curl -I https://api.redislabs.com/v1/
ping api.redislabs.com

# For Enterprise
curl -k https://your-cluster:9443/v1/bootstrap

# Check DNS
nslookup api.redislabs.com
```

**Solutions:**
```bash
# Increase timeout (if supported in future versions)
export REDISCTL_TIMEOUT=60

# Check proxy settings
export HTTP_PROXY=http://proxy:8080
export HTTPS_PROXY=http://proxy:8080

# Bypass proxy for local
export NO_PROXY=localhost,127.0.0.1

# Test with curl first
curl -x $HTTPS_PROXY https://api.redislabs.com/v1/
```

### SSL/TLS Errors

**Problem:** `SSL certificate problem` or `Certificate verify failed`

**Solutions for Enterprise:**
```bash
# Allow self-signed certificates
export REDIS_ENTERPRISE_INSECURE=true

# Or in profile
redisctl profile set enterprise \
  --deployment enterprise \
  --url "https://cluster:9443" \
  --username "admin" \
  --password "pass" \
  --insecure

# Import certificate
# Linux
sudo cp cluster-cert.pem /usr/local/share/ca-certificates/
sudo update-ca-certificates

# macOS
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain cluster-cert.pem
```

### Port Blocked

**Problem:** `Connection refused`

**Solutions:**
```bash
# Check if port is open
nc -zv api.redislabs.com 443
nc -zv your-cluster 9443

# Check firewall rules
# Linux
sudo iptables -L -n | grep 9443

# macOS
sudo pfctl -s rules

# Windows
netsh advfirewall firewall show rule name=all
```

## API Errors

### Rate Limiting

**Problem:** `429 Too Many Requests`

**Solutions:**
```bash
# Add delay between requests
for sub in $(cat subscriptions.txt); do
  redisctl cloud subscription get $sub
  sleep 2  # Wait 2 seconds
done

# Implement exponential backoff
retry_with_backoff() {
  local max_attempts=5
  local attempt=0
  local delay=1
  
  while [ $attempt -lt $max_attempts ]; do
    if "$@"; then
      return 0
    fi
    
    echo "Rate limited, waiting ${delay}s..."
    sleep $delay
    
    attempt=$((attempt + 1))
    delay=$((delay * 2))
  done
  
  return 1
}

retry_with_backoff redisctl cloud subscription list
```

### Resource Not Found

**Problem:** `404 Not Found`

**Diagnosis:**
```bash
# Verify resource exists
redisctl cloud subscription list
redisctl cloud database list --subscription-id 123456

# Check ID format
# Cloud: subscription_id:database_id
# Enterprise: numeric
```

**Solutions:**
```bash
# Use correct ID format
# Cloud
redisctl cloud database get \
  --subscription-id 123456 \
  --database-id 789

# Enterprise  
redisctl enterprise database get 1

# List to find correct ID
redisctl cloud subscription list -q "[].{id: id, name: name}"
```

### Invalid Request

**Problem:** `400 Bad Request`

**Solutions:**
```bash
# Validate JSON
cat payload.json | jq .

# Check required fields
# Example: database creation requires name
cat > database.json <<EOF
{
  "name": "my-database",  # Required
  "memoryLimitInGb": 1    # Required
}
EOF

# Use schema validation (if available)
redisctl validate database.json

# Test with minimal payload first
echo '{"name": "test", "memoryLimitInGb": 1}' | \
  redisctl api cloud post /subscriptions/123/databases --data @-
```

## Command Issues

### Command Not Recognized

**Problem:** `Unknown command`

**Solutions:**
```bash
# Check available commands
redisctl --help
redisctl cloud --help
redisctl enterprise --help

# Update to latest version
# Download latest from GitHub releases

# Check command syntax
redisctl cloud database list --subscription-id 123  # Correct
redisctl cloud database list 123                    # Incorrect
```

### Missing Required Arguments

**Problem:** `Missing required argument`

**Solutions:**
```bash
# Check command requirements
redisctl cloud database get --help

# Provide all required arguments
redisctl cloud database get \
  --subscription-id 123456 \  # Required
  --database-id 789           # Required

# Use environment variables for defaults
export REDIS_SUBSCRIPTION_ID=123456
```

### Output Parsing Errors

**Problem:** JMESPath query errors or unexpected output

**Solutions:**
```bash
# Test query separately
redisctl cloud subscription list -o json | jq .
redisctl cloud subscription list -q "[].name"

# Escape special characters
redisctl cloud database list -q "[?name=='my-db']"  # Correct
redisctl cloud database list -q '[?name==`my-db`]'  # Also correct

# Debug output format
redisctl cloud subscription list -o json > output.json
cat output.json | jq '.[] | keys'
```

## Async Operation Issues

### Operation Timeout

**Problem:** `Operation timeout` when using `--wait`

**Solutions:**
```bash
# Increase timeout
redisctl cloud database create \
  --subscription-id 123 \
  --data @db.json \
  --wait \
  --wait-timeout 1200  # 20 minutes

# Check operation status manually
TASK_ID=$(redisctl cloud database create \
  --subscription-id 123 \
  --data @db.json \
  -q "taskId")

# Poll manually
while true; do
  STATUS=$(redisctl api cloud get /tasks/$TASK_ID -q "status")
  echo "Status: $STATUS"
  if [ "$STATUS" = "completed" ] || [ "$STATUS" = "failed" ]; then
    break
  fi
  sleep 30
done
```

### Task Not Found

**Problem:** Cannot find task ID for async operation

**Solutions:**
```bash
# Check if operation returns task ID
redisctl cloud database create \
  --subscription-id 123 \
  --data @db.json \
  -o json | jq .

# Some operations might not be async
# Check API documentation

# List recent tasks
redisctl api cloud get /tasks --query-params "limit=10"
```

## Configuration Issues

### Config File Not Found

**Problem:** Configuration file not loading

**Solutions:**
```bash
# Check file location
redisctl profile path

# Create config directory
mkdir -p ~/.config/redisctl

# Initialize config
redisctl profile set default \
  --deployment cloud \
  --api-key "key" \
  --api-secret "secret"

# Check permissions
chmod 600 ~/.config/redisctl/config.toml
```

### Environment Variable Expansion

**Problem:** Variables in config not expanding

**Solutions:**
```toml
# config.toml
[profiles.prod]
deployment_type = "cloud"
api_key = "${REDIS_API_KEY}"  # Will expand
api_secret = "$REDIS_SECRET"   # Won't expand - needs braces

# With defaults
api_url = "${REDIS_API_URL:-https://api.redislabs.com/v1}"
```

## Performance Issues

### Slow Response Times

**Solutions:**
```bash
# Enable caching (if implemented)
export REDISCTL_CACHE=true

# Reduce response size
redisctl cloud subscription list --query-params "fields=id,name"

# Use specific queries
redisctl cloud database list -q "[0:5]"  # First 5 only

# Parallel processing
for id in $(cat database-ids.txt); do
  redisctl cloud database get --subscription-id 123 --database-id $id &
done
wait
```

### Large Output Handling

**Solutions:**
```bash
# Paginate results
LIMIT=50
OFFSET=0
while true; do
  RESULTS=$(redisctl api cloud get /subscriptions \
    --query-params "limit=$LIMIT&offset=$OFFSET")
  # Process results
  OFFSET=$((OFFSET + LIMIT))
done

# Stream to file
redisctl cloud database list --subscription-id 123 > databases.json

# Process with streaming tools
redisctl cloud database list --subscription-id 123 | jq -c '.[]' | while read db; do
  echo "Processing: $(echo $db | jq -r .name)"
done
```

## Debug Techniques

### Enable Debug Logging

```bash
# Basic debug
export RUST_LOG=debug
redisctl cloud subscription list

# Trace everything
export RUST_LOG=trace

# Specific modules
export RUST_LOG=redisctl=debug,redis_cloud=trace

# Save debug output
RUST_LOG=trace redisctl cloud subscription list 2> debug.log
```

### Inspect HTTP Traffic

```bash
# Use proxy for inspection
export HTTP_PROXY=http://localhost:8888
# Run Charles Proxy or similar

# Or use trace logging
RUST_LOG=trace redisctl api cloud get /subscriptions 2>&1 | grep -i "http"
```

### Test with Curl

```bash
# Replicate redisctl request with curl
# Cloud
curl -H "x-api-key: $API_KEY" \
     -H "x-api-secret-key: $SECRET" \
     https://api.redislabs.com/v1/subscriptions

# Enterprise
curl -k -u "admin:password" \
     https://cluster:9443/v1/cluster
```

## Getting Help

### Resources

1. **Check documentation**
   ```bash
   redisctl --help
   redisctl <command> --help
   ```

2. **View debug information**
   ```bash
   redisctl --version
   RUST_LOG=debug redisctl profile list
   ```

3. **Report issues**
   - GitHub Issues: https://github.com/joshrotenberg/redisctl/issues
   - Include: version, command, error message, debug output

4. **Community support**
   - Redis Discord
   - Stack Overflow with tag `redisctl`

### Information to Provide

When reporting issues, include:

```bash
# Version
redisctl --version

# Command that failed
redisctl cloud database list --subscription-id 123

# Error message
# Full error output

# Debug output
RUST_LOG=debug redisctl cloud database list --subscription-id 123 2>&1

# Environment
uname -a
echo $SHELL

# Config (sanitized)
redisctl profile show prod | sed 's/api_key=.*/api_key=REDACTED/'
```