# JMESPath Queries

Filter and transform output using JMESPath expressions with the `-q` or `--query` flag. redisctl includes 300+ extended functions beyond standard JMESPath.

## Basic Usage

```bash
# Get specific field
redisctl enterprise cluster get -q 'name'

# Get nested field
redisctl cloud database get 123:456 -q 'security.ssl_client_authentication'

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

## Pipelines

Chain operations together with `|` for complex transformations:

```bash
# Filter -> Sort -> Take top 3 -> Reshape
redisctl enterprise database list -q '
  [?status==`active`]
  | sort_by(@, &memory_size)
  | reverse(@)
  | [:3]
  | [*].{name: name, memory_gb: to_string(memory_size / `1073741824`)}'
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

## Extended Functions

redisctl includes 300+ extended JMESPath functions. Here are the most useful categories:

### String Functions

```bash
# Case conversion
redisctl enterprise database list -q '[].{name: upper(name)}'

# String manipulation
redisctl enterprise cluster get -q 'split(name, `-`)'
redisctl enterprise database list -q '[].{name: trim(name)}'

# Case transformations
redisctl api cloud get /subscriptions -q '[].{id: id, name: snake_case(name)}'
```

### Formatting Functions

```bash
# Format bytes (human readable)
redisctl enterprise database list -q '[].{name: name, memory: format_bytes(memory_size)}'
# Output: [{"name": "cache", "memory": "1.00 GB"}]

# Format duration
redisctl enterprise database list -q '[].{name: name, uptime: format_duration(uptime_seconds)}'
# Output: [{"name": "cache", "uptime": "2d 5h 30m"}]

# Parse bytes
redisctl api cloud get /subscriptions -q '[].{name: name, bytes: parse_bytes(memory_limit)}'
```

### Date/Time Functions

```bash
# Human-readable relative time
redisctl cloud task list -q '[].{id: id, created: time_ago(created_time)}'
# Output: [{"id": "task-123", "created": "2 hours ago"}]

# Format timestamps
redisctl cloud subscription list -q '[].{name: name, created: format_date(createdAt, `"%Y-%m-%d"`)}'

# Current timestamp
redisctl enterprise cluster get -q '{name: name, checked_at: now()}'

# Check if weekend/weekday
redisctl cloud task list -q '[?is_weekday(created_timestamp)]'

# Time calculations
redisctl enterprise database list -q '[].{name: name, age_days: date_diff(now(), created_at, `"days"`)}'
```

### Duration Functions

```bash
# Convert seconds to human format
redisctl enterprise database list -q '[].{name: name, uptime: format_duration(uptime_seconds)}'

# Parse duration strings
redisctl api enterprise get /v1/cluster -q '{timeout: parse_duration(`"1h30m"`)}'
# Output: {"timeout": 5400}
```

### Network Functions

```bash
# Check if IP is private
redisctl enterprise node list -q '[?is_private_ip(addr)].addr'

# Check CIDR containment
redisctl enterprise node list -q '[?cidr_contains(`"10.0.0.0/8"`, addr)]'

# Get network info
redisctl api enterprise get /v1/cluster -q '{
  network: cidr_network(deployment_cidr),
  broadcast: cidr_broadcast(deployment_cidr)
}'
```

### Math Functions

```bash
# Rounding
redisctl enterprise database list -q '[].{name: name, memory_gb: round(memory_size / `1073741824`, `2`)}'

# Min/max
redisctl enterprise database list -q 'max_by(@, &memory_size).name'

# Statistics
redisctl enterprise database list -q '{
  total: sum([].memory_size),
  avg: avg([].memory_size),
  count: length(@)
}'
```

### Semver Functions

```bash
# Compare versions
redisctl enterprise cluster get -q '{
  version: version,
  needs_upgrade: semver_compare(version, `"7.4.0"`) < `0`
}'

# Check version constraints
redisctl enterprise node list -q '[?semver_satisfies(redis_version, `">=7.0.0"`)]'
```

### Type Functions

```bash
# Type checking
redisctl enterprise database get 1 -q '{name: name, type: type_of(memory_size)}'

# Default values for missing fields
redisctl cloud database get 123:456 -q '{name: name, region: default(region, `"unknown"`)}'

# Check if empty
redisctl enterprise database get 1 -q '{name: name, has_endpoints: not(is_empty(endpoints))}'
```

### Utility Functions

```bash
# Conditional output
redisctl enterprise database list -q '[].{name: name, healthy: if(status == `"active"`, `"YES"`, `"NO"`)}'

# Coalesce (first non-null)
redisctl cloud database get 123:456 -q '{region: coalesce(region, cloud_region, `"default"`)}'

# Unique values
redisctl enterprise database list -q 'unique([].status)'
```

### Validation Functions

```bash
# Validate formats
redisctl enterprise database list -q '[].{
  name: name,
  valid_endpoint: is_ipv4(endpoints[0].addr),
  valid_uuid: is_uuid(database_id)
}'

# Email validation
redisctl api cloud get /users -q '[?is_email(email)].email'
```

### Encoding Functions

```bash
# Base64 encode/decode
redisctl enterprise cluster get -q '{encoded: base64_encode(name)}'
redisctl api enterprise get /v1/cluster -q '{decoded: base64_decode(encoded_field)}'

# URL encode/decode
redisctl api cloud get /subscriptions -q '[].{safe_name: url_encode(name)}'
```

### Hash Functions

```bash
# Generate hashes
redisctl enterprise database list -q '[].{name: name, hash: sha256(name)}'
redisctl api cloud get /subscriptions -q '[].{id: id, checksum: md5(to_string(@))}'
```

### JSON Patch Functions

```bash
# Compare two configs
redisctl enterprise database get 1 -q 'json_diff(current_config, desired_config)'

# Apply patches
redisctl api enterprise get /v1/bdbs/1 -q 'json_patch(@, `[{"op": "add", "path": "/tags", "value": ["prod"]}]`)'
```

### Fuzzy Matching

```bash
# Levenshtein distance
redisctl enterprise database list -q '[?levenshtein(name, `"cache"`) < `3`]'

# Phonetic matching
redisctl api cloud get /users -q '[?sounds_like(name, `"Smith"`)]'
```

## Function Categories Reference

| Category | Example Functions |
|----------|-------------------|
| String | `upper`, `lower`, `trim`, `split`, `snake_case`, `camel_case` |
| Array | `unique`, `flatten`, `chunk`, `zip`, `intersection` |
| Object | `keys`, `values`, `pick`, `omit`, `deep_merge` |
| Math | `round`, `floor`, `ceil`, `sum`, `avg`, `stddev` |
| Type | `type_of`, `is_array`, `is_string`, `to_boolean` |
| Utility | `if`, `coalesce`, `default`, `now` |
| DateTime | `format_date`, `time_ago`, `relative_time`, `is_weekend` |
| Duration | `format_duration`, `parse_duration` |
| Network | `is_private_ip`, `cidr_contains`, `ip_to_int` |
| Computing | `format_bytes`, `parse_bytes`, `bit_and`, `bit_or` |
| Validation | `is_email`, `is_uuid`, `is_ipv4`, `is_url` |
| Encoding | `base64_encode`, `base64_decode`, `url_encode` |
| Hash | `md5`, `sha256`, `hmac_sha256` |
| Regex | `regex_match`, `regex_replace`, `regex_extract` |
| Semver | `semver_compare`, `semver_satisfies`, `semver_parse` |
| Fuzzy | `levenshtein`, `soundex`, `jaro_winkler` |
| JSON Patch | `json_diff`, `json_patch`, `json_merge_patch` |

For the complete list, see the [jmespath-extensions documentation](https://docs.rs/jmespath_extensions).

## JMESPath Syntax Reference

| Syntax | Description | Example |
|--------|-------------|---------|
| `'value'` | String literal | `[?name=='cache']` |
| `` `123` `` | Number literal | `[?size > `1024`]` |
| `` `true` `` | Boolean literal | `[?active == `true`]` |
| `@` | Current element | `sort_by(@, &name)` |
| `.field` | Child access | `cluster.name` |
| `[0]` | Index access | `nodes[0]` |
| `[0:5]` | Slice | `databases[:5]` |
| `[?expr]` | Filter | `[?status=='active']` |
| `{k: v}` | Multi-select | `{id: uid, n: name}` |
| `\|` | Pipe | `[].name \| sort(@)` |
| `&field` | Expression reference | `sort_by(@, &name)` |

For full syntax, see [jmespath.org](https://jmespath.org/).

## Combining with Output Formats

```bash
# Query then format as table
redisctl enterprise database list \
  -q "[?status=='active'].{name:name,memory:memory_size}" \
  -o table
```
