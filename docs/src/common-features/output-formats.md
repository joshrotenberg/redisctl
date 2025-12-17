# Output Formats

redisctl supports multiple output formats for different use cases.

## Available Formats

### JSON (Default)

```bash
redisctl cloud database list
redisctl enterprise cluster get
```

Output:
```json
{
  "name": "my-cluster",
  "nodes": 3,
  "version": "7.2.4"
}
```

### Table

Human-readable tabular format:

```bash
redisctl cloud database list -o table
redisctl enterprise database list --output table
```

Output:
```
ID    NAME       MEMORY    STATUS
1     cache      1GB       active
2     sessions   512MB     active
```

### YAML

```bash
redisctl enterprise cluster get -o yaml
```

Output:
```yaml
name: my-cluster
nodes: 3
version: 7.2.4
```

## Combining with JMESPath

Filter and format in one command:

```bash
# JSON with filtered fields
redisctl enterprise database list -q "[].{name:name,memory:memory_size}"

# Table with specific columns
redisctl cloud subscription list -o table -q "[].{id:id,name:name,status:status}"
```

## Use Cases

| Format | Best For |
|--------|----------|
| JSON | Scripting, CI/CD pipelines |
| Table | Interactive use, quick overview |
| YAML | Config files, readable structured data |

## Using JMESPath Queries

Use the built-in `-q/--query` flag for filtering and transforming output without external tools:

```bash
# Get first database name
redisctl cloud database list -q '[0].name'

# Count items
redisctl enterprise database list -q 'length(@)'

# Get specific fields from all items
redisctl cloud subscription list -q '[].{id: id, name: name}'

# Filter by condition
redisctl enterprise database list -q "[?status=='active'].name"

# Get raw values for shell scripts (no JSON quotes)
redisctl cloud database list -q '[0].name' --raw
```

> **Note**: JMESPath is built into redisctl, so you don't need external tools like `jq` for most operations.
