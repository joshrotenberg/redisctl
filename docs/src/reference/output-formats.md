# Output Formats

`redisctl` supports multiple output formats and filtering options.

## Available Formats

### JSON (Default)

```bash
redisctl cloud database list
# or explicitly:
redisctl cloud database list -o json
```

Output:
```json
[
  {
    "databaseId": 12345,
    "name": "cache-db",
    "status": "active",
    "planMemoryLimit": 250.0
  }
]
```

### YAML

```bash
redisctl cloud database list -o yaml
```

Output:
```yaml
- databaseId: 12345
  name: cache-db
  status: active
  planMemoryLimit: 250.0
```

### Table

Human-readable table format:

```bash
redisctl cloud database list -o table
```

Output:
```
ID       NAME      STATUS   MEMORY(MB)
-------- --------- -------- ----------
12345    cache-db  active   250
67890    user-db   active   500
```

## JMESPath Filtering

Use `-q` or `--query` to filter output with JMESPath expressions:

### Basic Selection

```bash
# Get only names
redisctl cloud database list -q "[].name"

# Get specific fields
redisctl cloud database list -q "[].{name:name,endpoint:publicEndpoint}"
```

### Filtering

```bash
# Active databases only
redisctl cloud database list -q "[?status=='active']"

# Databases using more than 250MB
redisctl cloud database list -q "[?planMemoryLimit > `250`]"
```

### Sorting

```bash
# Sort by name
redisctl cloud database list -q "sort_by([], &name)"

# Sort by memory, descending
redisctl cloud database list -q "reverse(sort_by([], &planMemoryLimit))"
```

### Complex Queries

```bash
# Get top 5 databases by memory
redisctl cloud database list \
  -q "reverse(sort_by([], &planMemoryLimit))[:5].{name:name,memory:planMemoryLimit}"

# Count active databases
redisctl cloud database list -q "[?status=='active'] | length(@)"
```

## Raw Output

For scripting, use `-r` or `--raw` to get unformatted output:

```bash
# Get database IDs only
redisctl cloud database list -q "[].databaseId" -r
12345
67890

# Use in scripts
for db_id in $(redisctl cloud database list -q "[].databaseId" -r); do
  echo "Processing database $db_id"
done
```

## Combining Formats and Queries

```bash
# Query then format as table
redisctl cloud database list \
  -q "[?status=='active'].{name:name,memory:planMemoryLimit}" \
  -o table

# Query and output as YAML
redisctl cloud database list \
  -q "[?planMemoryLimit > `250`]" \
  -o yaml
```

## API Response Formats

When using raw API access:

```bash
# Pretty-printed JSON (default)
redisctl api cloud get /subscriptions

# Raw response
redisctl api cloud get /subscriptions --raw

# With headers
redisctl api cloud get /subscriptions --include-headers
```

## Tips

1. Use `-o table` for human reading
2. Use `-o json` (default) for parsing
3. Use `-q` with JMESPath for filtering
4. Use `-r` for script-friendly output
5. Combine query and format for best results

## JMESPath Reference

Common JMESPath patterns:

| Pattern | Description |
|---------|-------------|
| `[]` | All items |
| `[0]` | First item |
| `[-1]` | Last item |
| `[].name` | All names |
| `[?status=='active']` | Filter by condition |
| `[].{name:name,id:id}` | Select fields |
| `sort_by([], &field)` | Sort ascending |
| `reverse(sort_by([], &field))` | Sort descending |
| `[:5]` | First 5 items |
| `length([])` | Count items |

For more, see [JMESPath documentation](https://jmespath.org/).