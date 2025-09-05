# Output Formats

`redisctl` supports multiple output formats and filtering options.

## Available Formats

### JSON (Default)

```bash
redisctl database list
# or explicitly:
redisctl database list -o json
```

Output:
```json
[
  {
    "id": "12345",
    "name": "cache-db",
    "status": "active"
  }
]
```

### YAML

```bash
redisctl database list -o yaml
```

Output:
```yaml
- id: "12345"
  name: cache-db
  status: active
```

### Table

Human-readable table format:

```bash
redisctl database list -o table
```

Output:
```
ID     NAME      STATUS
------ --------- -------
12345  cache-db  active
67890  user-db   active
```

## JMESPath Filtering

Use `-q` or `--query` to filter output with JMESPath expressions:

### Basic Selection

```bash
# Get only names
redisctl database list -q "[].name"

# Get specific fields
redisctl database list -q "[].{name:name,port:port}"
```

### Filtering

```bash
# Active databases only
redisctl database list -q "[?status=='active']"

# Databases using more than 1GB
redisctl database list -q "[?memory_size > `1073741824`]"
```

### Sorting

```bash
# Sort by name
redisctl database list -q "sort_by([], &name)"

# Sort by memory, descending
redisctl database list -q "reverse(sort_by([], &memory_size))"
```

### Complex Queries

```bash
# Get top 5 databases by memory
redisctl database list \
  -q "reverse(sort_by([], &memory_size))[:5].{name:name,memory:memory_size}"

# Count active databases
redisctl database list -q "[?status=='active'] | length(@)"
```

## Raw Output

For scripting, use `-r` or `--raw` to get unformatted output:

```bash
# Get database IDs only
redisctl database list -q "[].id" -r
12345
67890

# Use in scripts
for db_id in $(redisctl database list -q "[].id" -r); do
  echo "Processing database $db_id"
done
```

## Combining Formats and Queries

```bash
# Query then format as table
redisctl database list \
  -q "[?status=='active'].{name:name,memory:memory_size}" \
  -o table

# Query and output as YAML
redisctl database list \
  -q "[?memory_size > `1073741824`]" \
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