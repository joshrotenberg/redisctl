# Output Formats

redisctl supports multiple output formats and filtering options for maximum flexibility.

## Available Formats

### JSON (Default)
```bash
# Default JSON output
redisctl database list

# Pretty-printed JSON
redisctl database list -o json
```

### YAML
```bash
# YAML output for better readability
redisctl database list -o yaml
```

### Table
```bash
# Human-readable table format
redisctl database list -o table
```

## JMESPath Filtering

Use the `-q` or `--query` flag with JMESPath expressions to filter and transform output.

### Basic Filtering
```bash
# Get only database names
redisctl database list -q "[].name"

# Get active databases
redisctl database list -q "[?status=='active']"

# Get specific fields
redisctl database list -q "[].{name: name, memory: memoryLimitInGb}"
```

### Advanced Queries
```bash
# Sort by memory size
redisctl database list -q "sort_by(@, &memoryLimitInGb)"

# Get databases with specific modules
redisctl database list -q "[?modules[?name=='RediSearch']]"

# Complex filtering and projection
redisctl database list -q "[?memoryLimitInGb > `5`].{name: name, region: region, memory: memoryLimitInGb}"
```

## Integration with Other Tools

### Using with jq
```bash
# JSON processing with jq
redisctl database list -o json | jq '.[] | select(.name | contains("prod"))'

# Extract specific values
redisctl database list -o json | jq -r '.[].id'
```

### Using with yq
```bash
# YAML processing with yq
redisctl database list -o yaml | yq '.[] | select(.status == "active")'
```

### Shell Scripting
```bash
#!/bin/bash
# Get database IDs into array
IDS=($(redisctl database list -q "[].id" -o json | jq -r '.[]'))

for ID in "${IDS[@]}"; do
  echo "Processing database $ID"
  redisctl database get $ID
done
```

## Output Format Examples

### JSON Output
```json
[
  {
    "id": 12345,
    "name": "production-db",
    "status": "active",
    "memoryLimitInGb": 10,
    "region": "us-east-1"
  },
  {
    "id": 67890,
    "name": "staging-db",
    "status": "active",
    "memoryLimitInGb": 5,
    "region": "us-west-2"
  }
]
```

### YAML Output
```yaml
- id: 12345
  name: production-db
  status: active
  memoryLimitInGb: 10
  region: us-east-1
- id: 67890
  name: staging-db
  status: active
  memoryLimitInGb: 5
  region: us-west-2
```

### Table Output
```
┌───────┬────────────────┬────────┬────────┬────────────┐
│ ID    │ Name           │ Status │ Memory │ Region     │
├───────┼────────────────┼────────┼────────┼────────────┤
│ 12345 │ production-db  │ active │ 10 GB  │ us-east-1  │
│ 67890 │ staging-db     │ active │ 5 GB   │ us-west-2  │
└───────┴────────────────┴────────┴────────┴────────────┘
```

## Formatting Best Practices

### Choosing the Right Format

- **JSON**: Best for programmatic processing and automation
- **YAML**: More readable for configuration and documentation
- **Table**: Best for human consumption and quick reviews

### Common Use Cases

#### Export Configuration
```bash
# Export database configuration
redisctl database get 12345 -o yaml > database-config.yaml
```

#### Generate Reports
```bash
# Create CSV report
redisctl database list -o json | \
  jq -r '.[] | [.id, .name, .status, .memoryLimitInGb] | @csv' > databases.csv
```

#### Monitor Resources
```bash
# Watch database status
watch -n 10 'redisctl database list -o table -q "[?status!='"'"'active'"'"']"'
```

## Pagination Support

For commands that return large result sets:

```bash
# Get first 10 results
redisctl cloud subscription list --limit 10

# Get next page
redisctl cloud subscription list --limit 10 --offset 10
```

## Error Output

Errors are always output to stderr in a consistent format:

```bash
# Redirect errors to file
redisctl database create --data @invalid.json 2> errors.log

# Suppress errors
redisctl database list 2>/dev/null
```

## Custom Formatting Examples

### Dashboard Script
```bash
#!/bin/bash
# Generate dashboard data
{
  echo "=== Database Status ==="
  redisctl database list -o table -q "[?status!='active']"
  
  echo -e "\n=== Resource Usage ==="
  redisctl database list -o json | \
    jq -r '"Total Memory: \(map(.memoryLimitInGb) | add) GB"'
  
  echo -e "\n=== Recent Tasks ==="
  redisctl cloud task list --limit 5 -o table
} | tee dashboard.txt
```

### Monitoring Script
```bash
#!/bin/bash
# Monitor specific metrics
while true; do
  clear
  echo "Database Monitor - $(date)"
  echo "========================"
  
  redisctl database list -o json | jq -r '
    .[] | 
    "\(.name): \(.status) - Memory: \(.memoryUsedInMb // 0)/\(.memoryLimitInGb * 1024) MB"
  '
  
  sleep 30
done
```