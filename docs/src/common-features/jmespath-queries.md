# JMESPath Queries

Filter and transform output using JMESPath expressions with the `-q` or `--query` flag.

## Basic Usage

```bash
# Get specific field
redisctl enterprise cluster get -q 'name'

# Get nested field
redisctl cloud database get 123 456 -q 'security.ssl_client_authentication'

# Get multiple fields
redisctl enterprise database get 1 -q '{name: name, memory: memory_size, port: port}'
```

## Array Operations

```bash
# Get all names from a list
redisctl enterprise database list -q '[].name'

# Get first item
redisctl cloud subscription list -q '[0]'

# Get specific fields from each item
redisctl enterprise database list -q '[].{id: uid, name: name, status: status}'
```

## Filtering

```bash
# Filter by condition
redisctl enterprise database list -q "[?status=='active']"

# Filter and select fields
redisctl cloud database list -q "[?memoryLimitInGb > `1`].{name: name, memory: memoryLimitInGb}"

# Multiple conditions
redisctl enterprise database list -q "[?status=='active' && memory_size > `1073741824`]"
```

## Sorting and Slicing

```bash
# Sort by field
redisctl enterprise database list -q "sort_by(@, &name)"

# Reverse sort
redisctl cloud subscription list -q "reverse(sort_by(@, &id))"

# Get first 5
redisctl enterprise database list -q '[:5]'
```

## Common Patterns

### Extract Single Value

```bash
# Get cluster name as plain text
redisctl enterprise cluster get -q 'name'
# Output: my-cluster
```

### Build Custom Objects

```bash
redisctl enterprise database list -q '[].{
  database: name,
  size_gb: to_string(memory_size / `1073741824`),
  endpoints: endpoints[0].addr
}'
```

### Count Items

```bash
redisctl enterprise database list -q 'length(@)'
```

### Check if Empty

```bash
redisctl cloud subscription list -q 'length(@) == `0`'
```

## JMESPath Reference

- Strings: `'value'` (single quotes)
- Numbers: `` `123` `` (backticks)
- Booleans: `` `true` ``, `` `false` ``
- Current element: `@`
- Child: `.field`
- Index: `[0]`
- Slice: `[0:5]`
- Filter: `[?condition]`
- Multi-select: `{key1: field1, key2: field2}`
- Pipe: `expression | another`

For full syntax, see [jmespath.org](https://jmespath.org/).

## Combining with Output Formats

```bash
# Query then format as table
redisctl enterprise database list \
  -q "[?status=='active'].{name:name,memory:memory_size}" \
  -o table
```
