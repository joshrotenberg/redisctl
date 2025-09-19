# Output Formats

redisctl supports multiple output formats to suit different use cases.

## Available Formats

### Auto (Default)
Automatically selects the best format based on context:
```bash
# Auto-detects format
redisctl cloud database list
```

### JSON
Machine-readable JSON output:
```bash
redisctl cloud database list -o json
```

### YAML
Human-readable structured format:
```bash
redisctl cloud database list -o yaml
```

### Table
Formatted tables for human consumption:
```bash
redisctl cloud database list -o table
```

## JMESPath Filtering

Use the `-q` flag for powerful JSON queries:

```bash
# Get all database names
redisctl cloud database list -q "[].name"

# Filter by status
redisctl cloud database list -q "[?status=='active']"

# Custom projections
redisctl cloud database list -q "[].{name: name, memory: planMemoryLimit}"
```

### Advanced Queries
```bash
# Sort by memory
redisctl cloud database list -q "sort_by(@, &planMemoryLimit)"

# Find databases with specific modules
redisctl cloud database list -q "[?modules[?name=='RediSearch']]"

# Complex filtering (memory > 250MB)
redisctl cloud database list -q "[?planMemoryLimit > `250`].{name: name, region: region, memory: planMemoryLimit}"
```

## Working with Other Tools

### jq Integration
```bash
# Filter with jq
redisctl cloud database list -o json | jq '.[] | select(.name | contains("prod"))'

# Extract IDs
redisctl cloud database list -o json | jq -r '.[].databaseId'
```

### yq for YAML
```bash
redisctl cloud database list -o yaml | yq '.[] | select(.status == "active")'
```

## Scripting Examples

### Batch Operations
```bash
# Get all database IDs
IDS=($(redisctl cloud database list -q "[].databaseId" -o json | jq -r '.[]'))

# Process each database
for ID in "${IDS[@]}"; do
  redisctl cloud database get $ID
done
```

### Output Redirection
```bash
# Save to file
redisctl cloud database list -o json > databases.json

# Append to log
redisctl cloud database list >> operations.log

# Error handling
redisctl cloud database list 2> errors.log || echo "Failed"
```

## Environment Detection

redisctl automatically detects the output environment:

- **Terminal**: Defaults to table format for readability
- **Pipe**: Defaults to JSON for processing
- **Redirect**: Defaults to JSON for storage

Override with `-o` flag when needed.

## Format-Specific Features

### Table Features
- Automatic column width adjustment
- Row highlighting for important data
- Pagination for large datasets
- Color support when terminal supports it

### JSON Features
- Pretty-printed by default
- Compact mode available with `--compact`
- Proper escaping for special characters
- Null values handled correctly

### YAML Features
- Comments for clarity
- Multi-line string support
- Proper indentation
- Type preservation

## Error Handling

Different formats handle errors differently:

### JSON Errors
```json
{
  "error": "Authentication failed",
  "details": "Invalid API key"
}
```

### Table Errors
```
Error: Authentication failed
Details: Invalid API key
```

### YAML Errors
```yaml
error: Authentication failed
details: Invalid API key
```

## Performance Considerations

- **JSON**: Fastest parsing, smallest size
- **YAML**: Human-readable, larger size
- **Table**: Terminal rendering overhead

## Examples

### Save Configuration
```bash
redisctl cloud database get 12345 -o yaml > database-config.yaml
```

### Generate Reports
```bash
# CSV-like output for spreadsheets
redisctl cloud database list -o json | \
  jq -r '.[] | [.name, .status, .memory] | @csv'
```

### Monitor Changes
```bash
# Watch for inactive databases
watch -n 10 'redisctl cloud database list -o table -q "[?status!='"'"'active'"'"']"'
```

## Tips and Tricks

1. **Default Format**: Set `REDISCTL_OUTPUT` environment variable
   ```bash
   export REDISCTL_OUTPUT=json
   ```

2. **Raw Output**: Use `-r` or `--raw` for unformatted output
   ```bash
   redisctl cloud database list -q "[].id" -r
   ```

3. **Silent Mode**: Suppress non-essential output
   ```bash
   redisctl cloud database create --data @db.json 2> errors.log
   ```

4. **Pretty Print**: Control JSON formatting
   ```bash
   redisctl cloud database list 2>/dev/null
   ```

## Complex Workflows

### Health Dashboard
```bash
#!/bin/bash
while true; do
  clear
  echo "=== Database Health ==="
  redisctl cloud database list -o table -q "[?status!='active']"
  echo ""
  echo "=== Resource Usage ==="
  redisctl cloud database list -o json | \
    jq -r '.[] | "\(.name): \(.usedMemoryInMb)MB / \(.memoryLimitInGb)GB"'
  sleep 60
done
```

### Automated Reporting
```bash
#!/bin/bash
REPORT_DATE=$(date +%Y-%m-%d)
REPORT_FILE="database-report-${REPORT_DATE}.json"

# Collect all database information
{
  echo "{"
  echo "  \"report_date\": \"${REPORT_DATE}\","
  echo "  \"databases\": "
  redisctl cloud database list -o json | jq -r '
    map({
      name: .name,
      status: .status,
      region: .region,
      memory_gb: .memoryLimitInGb,
      throughput: .throughputMeasurement
    })
  '
  echo "}"
} > "$REPORT_FILE"

echo "Report saved to $REPORT_FILE"
```

## Best Practices

1. **Use JSON for automation** - Most reliable for parsing
2. **Use Table for human review** - Easiest to read
3. **Use YAML for configuration** - Best for config files
4. **Use JMESPath for filtering** - More powerful than jq for simple queries
5. **Combine tools** - Use redisctl with jq, yq, awk for complex processing