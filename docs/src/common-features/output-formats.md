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
| JSON | Scripting, piping to `jq`, CI/CD |
| Table | Interactive use, quick overview |
| YAML | Config files, readable structured data |

## Piping to Other Tools

```bash
# Parse with jq
redisctl cloud database list | jq '.[0].name'

# Count items
redisctl enterprise database list | jq 'length'

# Convert to CSV
redisctl cloud subscription list -o json | jq -r '.[] | [.id, .name] | @csv'
```
