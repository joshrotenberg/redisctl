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

## Real-World Examples

These examples work with actual Redis Cloud API responses:

### Counting and Aggregation

```bash
# Count all subscriptions
redisctl cloud subscription list -o json -q 'length(@)'
# Output: 192

# Aggregate statistics across all subscriptions
redisctl cloud subscription list -o json \
  -q '{total_subscriptions: length(@), total_size_gb: sum([*].cloudDetails[0].totalSizeInGb), avg_size_gb: avg([*].cloudDetails[0].totalSizeInGb)}'
# Output:
# {
#   "avg_size_gb": 1.96,
#   "total_size_gb": 23.56,
#   "total_subscriptions": 192
# }
```

### Projections - Reshaping Data

```bash
# Extract specific fields from each subscription
redisctl cloud subscription list -o json \
  -q '[*].{id: id, name: name, provider: cloudDetails[0].provider, region: cloudDetails[0].regions[0].region} | [:5]'
# Output:
# [
#   {"id": 2983053, "name": "time-series-demo", "provider": "AWS", "region": "ap-southeast-1"},
#   {"id": 2988697, "name": "workshop-sub", "provider": "AWS", "region": "us-east-1"},
#   ...
# ]
```

### Unique Values and Sorting

```bash
# Get unique cloud providers
redisctl cloud subscription list -o json -q '[*].cloudDetails[0].provider | unique(@)'
# Output: ["AWS", "GCP"]

# Get unique regions, sorted
redisctl cloud subscription list -o json \
  -q '[*].cloudDetails[0].regions[0].region | unique(@) | sort(@)'
# Output: ["ap-northeast-1", "ap-south-1", "ap-southeast-1", "europe-west1", ...]

# Sort subscription names alphabetically
redisctl cloud subscription list -o json -q '[*].name | sort(@) | [:10]'
```

### Filtering with Patterns

```bash
# Find subscriptions containing 'demo'
redisctl cloud subscription list -o json \
  -q "[*].name | [?contains(@, 'demo')] | [:5]"
# Output: ["xw-time-series-demo", "gabs-redis-streams-demo", "anton-live-demo", ...]

# Filter by prefix
redisctl cloud subscription list -o json \
  -q "[*].name | [?starts_with(@, 'gabs')] | [:5]"
# Output: ["gabs-aws-workshop-sub", "gabs-santander-rdi", "gabs-redis-streams-demo", ...]

# Filter by suffix
redisctl cloud subscription list -o json \
  -q "[*].name | [?ends_with(@, 'demo')] | [:5]"
```

### String Transformations

```bash
# Convert names to uppercase
redisctl cloud subscription list -o json -q 'map(&upper(name), [*]) | [:3]'
# Output: ["XW-TIME-SERIES-DEMO", "GABS-AWS-WORKSHOP-SUB", "BAMOS-TEST"]

# Replace substrings
redisctl cloud subscription list -o json \
  -q "[*].{name: name, replaced: replace(name, 'demo', 'DEMO')} | [?contains(name, 'demo')] | [:3]"
# Output:
# [
#   {"name": "xw-time-series-demo", "replaced": "xw-time-series-DEMO"},
#   {"name": "gabs-redis-streams-demo", "replaced": "gabs-redis-streams-DEMO"},
#   ...
# ]
```

### Fuzzy Matching with Levenshtein Distance

```bash
# Find subscriptions with names similar to "production"
redisctl cloud subscription list -o json \
  -q "[*].{name: name, distance: levenshtein(name, 'production')} | sort_by(@, &distance) | [:5]"
# Output:
# [
#   {"distance": 8.0, "name": "piyush-db"},
#   {"distance": 8.0, "name": "erni-rdi-1"},
#   ...
# ]
```

### Sorting by Computed Values

```bash
# Sort by name length (shortest first)
redisctl cloud subscription list -o json \
  -q "[*].{name: name, len: length(name)} | sort_by(@, &len) | [:5]"
# Output:
# [
#   {"len": 5, "name": "bgiri"},
#   {"len": 6, "name": "abhidb"},
#   {"len": 6, "name": "CM-rag"},
#   ...
# ]
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

## Pipelines

Chain operations together with `|` for complex transformations:

```bash
# Get unique regions -> sort -> count
redisctl cloud subscription list -o json \
  -q '[*].cloudDetails[0].regions[0].region | unique(@) | sort(@) | length(@)'
# Output: 7

# Filter -> Sort -> Take top 3 -> Reshape
redisctl enterprise database list -q '
  [?status==`active`]
  | sort_by(@, &memory_size)
  | reverse(@)
  | [:3]
  | [*].{name: name, memory_gb: to_string(memory_size / `1073741824`)}'
```

## Extended Functions

redisctl includes 300+ extended JMESPath functions. Here are the most useful categories:

### String Functions

```bash
# Case conversion
redisctl cloud subscription list -o json -q '[*].{name: name, upper_name: upper(name)} | [:3]'

# Trim whitespace
redisctl enterprise database list -q '[].{name: trim(name)}'

# Replace substrings
redisctl cloud subscription list -o json \
  -q "[*].{original: name, modified: replace(name, '-', '_')} | [:3]"
```

### Formatting Functions

```bash
# Format bytes (human readable)
redisctl enterprise database list -q '[].{name: name, memory: format_bytes(memory_size)}'
# Output: [{"name": "cache", "memory": "1.00 GB"}]

# Format duration
redisctl enterprise database list -q '[].{name: name, uptime: format_duration(uptime_seconds)}'
# Output: [{"name": "cache", "uptime": "2d 5h 30m"}]
```

### Date/Time Functions

```bash
# Current timestamp
redisctl cloud subscription list -o json -q '{count: length(@), timestamp: now()}'
# Output: {"count": 192, "timestamp": 1765661197.0}

# Human-readable relative time
redisctl cloud task list -q '[].{id: id, created: time_ago(created_time)}'
# Output: [{"id": "task-123", "created": "2 hours ago"}]
```

### Network Functions

```bash
# Check if IP is private
redisctl enterprise node list -q '[?is_private_ip(addr)].addr'

# Check CIDR containment
redisctl enterprise node list -q '[?cidr_contains(`"10.0.0.0/8"`, addr)]'
```

### Math Functions

```bash
# Get max value
redisctl cloud subscription list -o json -q '[*].cloudDetails[0].totalSizeInGb | max(@)'
# Output: 23.2027

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
redisctl cloud subscription list -o json -q '[*].{name: name, type: type_of(id)} | [:3]'
# Output:
# [
#   {"name": "xw-time-series-demo", "type": "number"},
#   ...
# ]

# Default values for missing fields
redisctl cloud database get 123:456 -q '{name: name, region: default(region, `"unknown"`)}'

# Check if empty
redisctl enterprise database get 1 -q '{name: name, has_endpoints: not(is_empty(endpoints))}'
```

### Utility Functions

```bash
# Unique values
redisctl cloud subscription list -o json -q 'unique([*].status)'
# Output: ["active"]

# Coalesce (first non-null)
redisctl cloud database get 123:456 -q '{region: coalesce(region, cloud_region, `"default"`)}'
```

### Fuzzy Matching

```bash
# Levenshtein distance for fuzzy search
redisctl cloud subscription list -o json \
  -q "[*].{name: name, distance: levenshtein(name, 'cache')} | sort_by(@, &distance) | [:5]"

# Find similar names (distance < 3)
redisctl enterprise database list -q '[?levenshtein(name, `"cache"`) < `3`]'
```

### Encoding Functions

```bash
# Base64 encode/decode
redisctl enterprise cluster get -q '{encoded: base64_encode(name)}'

# URL encode/decode
redisctl api cloud get /subscriptions -q '[].{safe_name: url_encode(name)}'
```

### Hash Functions

```bash
# Generate hashes
redisctl enterprise database list -q '[].{name: name, hash: sha256(name)}'
```

### JSON Patch Functions

```bash
# Compare two configs
redisctl enterprise database get 1 -q 'json_diff(current_config, desired_config)'
```

## Function Categories Reference

| Category | Example Functions |
|----------|-------------------|
| String | `upper`, `lower`, `trim`, `replace`, `split`, `snake_case`, `camel_case` |
| Array | `unique`, `flatten`, `chunk`, `zip`, `intersection` |
| Object | `keys`, `values`, `pick`, `omit`, `deep_merge` |
| Math | `round`, `floor`, `ceil`, `sum`, `avg`, `max`, `min`, `stddev` |
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
