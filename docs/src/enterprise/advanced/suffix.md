# DNS Suffix Management

The suffix commands allow you to manage DNS suffixes for Redis Enterprise database endpoints. DNS suffixes provide custom domain names for database endpoints, useful for multi-tenant deployments and organizing databases by environment or purpose.

## Available Commands

### List DNS Suffixes

List all configured DNS suffixes in the cluster:

```bash
# List all DNS suffixes
redisctl enterprise suffix list

# List suffixes as a table
redisctl enterprise suffix list -o table

# Filter to active suffixes
redisctl enterprise suffix list -q "[?status == 'active']"

# Get suffix names only
redisctl enterprise suffix list -q "[].name"
```

### Get DNS Suffix Details

Get detailed information about a specific DNS suffix:

```bash
# Get suffix details
redisctl enterprise suffix get example.redis.local

# Get suffix in YAML format
redisctl enterprise suffix get example.redis.local -o yaml

# Extract specific fields
redisctl enterprise suffix get example.redis.local -q '{name: name, dns_servers: dns_servers}'

# Check if suffix is in use
redisctl enterprise suffix get example.redis.local -q 'in_use'
```

## Output Examples

### DNS Suffix List
```json
[
  {
    "name": "prod.redis.local",
    "status": "active",
    "dns_servers": ["10.0.1.53", "10.0.2.53"],
    "databases": 5,
    "created": "2024-01-15T10:30:00Z"
  },
  {
    "name": "dev.redis.local",
    "status": "active",
    "dns_servers": ["10.0.3.53"],
    "databases": 12,
    "created": "2024-02-20T14:15:00Z"
  }
]
```

### DNS Suffix Details
```json
{
  "name": "prod.redis.local",
  "status": "active",
  "dns_servers": ["10.0.1.53", "10.0.2.53"],
  "dns_zone": "redis.local",
  "ttl": 60,
  "databases": [
    {
      "bdb_uid": 1,
      "name": "cache-db",
      "endpoint": "cache-db.prod.redis.local:16379"
    },
    {
      "bdb_uid": 2,
      "name": "session-db",
      "endpoint": "session-db.prod.redis.local:16380"
    }
  ],
  "in_use": true,
  "created": "2024-01-15T10:30:00Z",
  "modified": "2024-03-01T09:45:00Z"
}
```

## Common Use Cases

### Environment-Based Suffixes

Organize databases by environment using DNS suffixes:

```bash
# List production suffixes
redisctl enterprise suffix list -q "[?contains(name, 'prod')]"

# List development suffixes
redisctl enterprise suffix list -q "[?contains(name, 'dev')]"

# Check staging suffix configuration
redisctl enterprise suffix get staging.redis.local
```

### Multi-Tenant Deployments

Manage suffixes for multi-tenant scenarios:

```bash
# List suffixes by tenant
redisctl enterprise suffix list -q "[?contains(name, 'tenant')]" -o table

# Get tenant-specific suffix
redisctl enterprise suffix get tenant-a.redis.local

# Count databases per suffix
redisctl enterprise suffix list -q "[].{suffix: name, database_count: databases}"
```

### DNS Configuration Verification

Verify DNS suffix configurations:

```bash
# Check DNS servers for all suffixes
redisctl enterprise suffix list -q "[].{name: name, servers: dns_servers}"

# Find suffixes with specific DNS server
redisctl enterprise suffix list -q "[?contains(dns_servers, '10.0.1.53')]"

# Verify TTL settings
redisctl enterprise suffix list -q "[].{name: name, ttl: ttl}" -o table
```

## Integration Examples

### Database Creation with Suffix

When creating databases, specify the DNS suffix:

```bash
# Create database with specific suffix
cat <<EOF | redisctl enterprise database create --data -
{
  "name": "app-cache",
  "memory_size": 1073741824,
  "dns_suffix_name": "prod.redis.local"
}
EOF

# Verify database endpoint
redisctl enterprise database get <bdb_uid> -q 'endpoint'
```

### Monitoring Suffix Usage

Monitor DNS suffix utilization:

```bash
# Check suffix usage
for suffix in $(redisctl enterprise suffix list -q "[].name" --raw); do
  echo "Suffix: $suffix"
  redisctl enterprise suffix get "$suffix" -q 'length(databases)'
done

# Find unused suffixes
redisctl enterprise suffix list -q "[?databases == \`0\`].name"

# Get suffix with most databases
redisctl enterprise suffix list -q "max_by(@, &databases).{name: name, count: databases}"
```

### DNS Server Management

Manage DNS server configurations:

```bash
# List all DNS servers per suffix
redisctl enterprise suffix list -q "[].{name: name, dns_servers: dns_servers}" -o table

# Find suffixes by DNS server count
redisctl enterprise suffix list -q "[?length(dns_servers) > \`1\`]"

# Check DNS server availability (get first suffix's servers as example)
for server in $(redisctl enterprise suffix list -q "[0].dns_servers[]" --raw); do
  echo "Checking DNS server: $server"
  dig @$server test.redis.local +short
done
```

## Best Practices

1. **Naming Convention**: Use consistent naming patterns for suffixes (e.g., `<environment>.<domain>`)
2. **DNS Server Redundancy**: Configure multiple DNS servers for high availability
3. **TTL Settings**: Set appropriate TTL values based on your DNS infrastructure
4. **Environment Separation**: Use different suffixes for different environments
5. **Documentation**: Maintain documentation of suffix assignments and purposes

## Troubleshooting

### Suffix Not Resolving

If DNS suffixes are not resolving:

```bash
# Check suffix configuration
redisctl enterprise suffix get <suffix_name>

# Verify DNS servers
redisctl enterprise suffix get <suffix_name> -q 'dns_servers'

# Check database endpoints using the suffix
redisctl enterprise database list -q "[?dns_suffix_name == '<suffix_name>']"

# Test DNS resolution
dig @<dns_server> <database>.<suffix_name>
```

### Database Endpoint Issues

When databases aren't accessible via suffix:

```bash
# Check database suffix assignment
redisctl enterprise database get <bdb_uid> -q 'dns_suffix_name'

# Verify suffix is active
redisctl enterprise suffix get <suffix_name> -q 'status'

# List all endpoints for suffix
redisctl enterprise suffix get <suffix_name> -q 'databases[].endpoint'
```

## Related Commands

- `redisctl enterprise database` - Create and manage databases with DNS suffixes
- `redisctl enterprise cluster` - View cluster-wide DNS configuration
- `redisctl enterprise endpoint` - Monitor endpoint availability and statistics