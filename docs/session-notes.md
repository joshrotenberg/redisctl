# MCP Database Tools & jpx Discovery - Session Notes

## Current Status (Updated)

**Branch:** `feat/mcp-database-tools`

**What we built:**
- 189+ MCP tool handlers total
- Full CRUD operations for all Redis data types
- Redis Stack module support (RediSearch, RedisJSON, RedisTimeSeries, RedisBloom)
- Discovery spec v0.3.0 with 77 documented tools

---

## Future Refactoring Notes

### Consider splitting database_tools.rs
The `database_tools.rs` file is getting large with all Redis Stack module implementations. Consider splitting into:
- `database_tools/mod.rs` - Core DatabaseTools struct, connection management
- `database_tools/core.rs` - Basic Redis commands (GET, SET, HASH, LIST, etc.)
- `database_tools/search.rs` - RediSearch (FT.*) commands
- `database_tools/json.rs` - RedisJSON (JSON.*) commands  
- `database_tools/timeseries.rs` - RedisTimeSeries (TS.*) commands
- `database_tools/bloom.rs` - RedisBloom (BF.*, CF.*, CMS.*, TOPK.*, TDIGEST.*) commands

Similarly for `server.rs` tool handlers - could split by module category.

---

## Redis Stack Module Tools Added This Session

### RediSearch (4 tools)
- `database_ft_search` - Full-text search with filtering, sorting, pagination, highlighting, scoring
- `database_ft_aggregate` - Complex aggregations with GROUPBY, REDUCE, APPLY, SORTBY
- `database_ft_info` - Index schema, document count, memory usage
- `database_ft_list` - List all indexes

### RedisJSON (8 tools)
- `database_json_get` - Get JSON at paths (JSONPath syntax: `$.store.book[0]`, `$..price`)
- `database_json_set` - Set JSON with NX/XX conditions
- `database_json_del` - Delete JSON at path
- `database_json_type` - Get value type (object, array, string, number, boolean, null)
- `database_json_arrappend` - Append to JSON arrays
- `database_json_arrlen` - Get array length
- `database_json_numincrby` - Atomic increment/decrement numbers
- `database_json_strlen` - Get string length

### RedisTimeSeries (5 tools)
- `database_ts_add` - Add samples with retention, encoding, labels, duplicate policy
- `database_ts_get` - Get latest sample
- `database_ts_range` - Query with aggregation (avg, sum, min, max, count, first, last, range, std.p, std.s, var.p, var.s)
- `database_ts_info` - Time series metadata (retention, chunks, memory, labels)
- `database_ts_create` - Create with options

### RedisBloom (5 tools)
- `database_bf_reserve` - Create filter with error rate and capacity
- `database_bf_add` - Add single item
- `database_bf_madd` - Bulk add items
- `database_bf_exists` - Check single item
- `database_bf_mexists` - Bulk check items
- `database_bf_info` - Filter metadata

---

## Next Session: Testing Plan

### 1. Verify Basic Setup

```bash
# Restart MCP server to pick up new tools
# Then test:
database_ping  # Should return PONG
database_dbsize  # Check key count
database_module_list  # Verify Redis Stack modules loaded
```

### 2. Test RediSearch Tools

**Create a test index and documents:**
```
# Using database_execute or JSON tools first:
# Create some JSON documents
database_json_set key="product:1" path="$" value='{"name":"Laptop","category":"electronics","price":999.99,"description":"High performance laptop with 16GB RAM"}'
database_json_set key="product:2" path="$" value='{"name":"Headphones","category":"electronics","price":149.99,"description":"Wireless noise-canceling headphones"}'
database_json_set key="product:3" path="$" value='{"name":"Coffee Maker","category":"kitchen","price":79.99,"description":"Programmable drip coffee maker"}'

# Create index (use database_execute for FT.CREATE)
database_execute command="FT.CREATE" args=["idx:products", "ON", "JSON", "PREFIX", "1", "product:", "SCHEMA", "$.name", "AS", "name", "TEXT", "$.category", "AS", "category", "TAG", "$.price", "AS", "price", "NUMERIC", "$.description", "AS", "description", "TEXT"]
```

**Test search tools:**
```
# List indexes
database_ft_list  # Should show idx:products

# Get index info
database_ft_info index="idx:products"

# Basic search
database_ft_search index="idx:products" query="laptop"

# Search with filters
database_ft_search index="idx:products" query="@category:{electronics}" withscores=true

# Search with sorting
database_ft_search index="idx:products" query="*" sortby="price" limit_num=10

# Aggregation - count by category
database_ft_aggregate index="idx:products" query="*" groupby=[{"properties":["@category"],"reducers":[{"function":"COUNT","args":[],"alias":"count"}]}]

# Aggregation - average price by category
database_ft_aggregate index="idx:products" query="*" groupby=[{"properties":["@category"],"reducers":[{"function":"AVG","args":["@price"],"alias":"avg_price"}]}]
```

### 3. Test RedisJSON Tools

```
# Create nested JSON
database_json_set key="user:json:1" path="$" value='{"name":"Alice","profile":{"age":30,"city":"NYC"},"orders":[{"id":1,"total":99.99},{"id":2,"total":149.99}]}'

# Read paths
database_json_get key="user:json:1" paths=["$.name"]
database_json_get key="user:json:1" paths=["$.profile.city"]
database_json_get key="user:json:1" paths=["$.orders[*].total"]
database_json_get key="user:json:1" paths=["$..id"]  # Recursive

# Get type
database_json_type key="user:json:1" path="$.orders"  # Should be "array"
database_json_type key="user:json:1" path="$.profile"  # Should be "object"

# Array operations
database_json_arrlen key="user:json:1" path="$.orders"  # Should be 2
database_json_arrappend key="user:json:1" path="$.orders" values=['{"id":3,"total":49.99}']
database_json_arrlen key="user:json:1" path="$.orders"  # Should be 3

# Numeric operations
database_json_numincrby key="user:json:1" path="$.profile.age" value=1  # Age becomes 31

# Delete path
database_json_del key="user:json:1" path="$.orders[0]"  # Remove first order
```

### 4. Test RedisTimeSeries Tools

```
# Create a time series with labels
database_ts_create key="sensor:temp:1" retention=86400000 labels=[{"label":"sensor","value":"temperature"},{"label":"location","value":"room1"}]

# Add samples (use "*" for auto-timestamp)
database_ts_add key="sensor:temp:1" timestamp="*" value=22.5
database_ts_add key="sensor:temp:1" timestamp="*" value=23.1
database_ts_add key="sensor:temp:1" timestamp="*" value=22.8

# Or with specific timestamps
database_ts_add key="sensor:temp:2" timestamp="1704067200000" value=20.0
database_ts_add key="sensor:temp:2" timestamp="1704067260000" value=20.5
database_ts_add key="sensor:temp:2" timestamp="1704067320000" value=21.0

# Get latest
database_ts_get key="sensor:temp:1"

# Get info
database_ts_info key="sensor:temp:1"

# Query range
database_ts_range key="sensor:temp:1" from="-" to="+"

# Query with aggregation
database_ts_range key="sensor:temp:1" from="-" to="+" aggregation="avg" bucket_duration=60000

# Query with count limit
database_ts_range key="sensor:temp:1" from="-" to="+" count=10
```

### 5. Test RedisBloom Tools

```
# Create a bloom filter
database_bf_reserve key="users:seen" error_rate=0.01 capacity=10000

# Add items
database_bf_add key="users:seen" item="user123"
database_bf_add key="users:seen" item="user456"

# Bulk add
database_bf_madd key="users:seen" items=["user789","user101","user102"]

# Check existence
database_bf_exists key="users:seen" item="user123"  # Should be true
database_bf_exists key="users:seen" item="unknown"  # Should be false (probably)

# Bulk check
database_bf_mexists key="users:seen" items=["user123","user456","nothere"]

# Get filter info
database_bf_info key="users:seen"
```

### 6. Test jpx Integration with Module Data

After the jpx bugs are fixed, test pipelines:

```
# Get Redis INFO and parse with jpx
database_info section="memory"
# Then use jpx to extract specific values

# Get time series data and aggregate with jpx
database_ts_range key="sensor:temp:1" from="-" to="+"
# Then use jpx: avg(samples[*].value)

# Get JSON and transform with jpx
database_json_get key="user:json:1" paths=["$"]
# Then use jpx to reshape/filter
```

### 7. Verify Read-Only Mode Works

All write operations should fail in read-only mode:
```
# These should return errors about read-only mode:
database_json_set key="test" path="$" value="{}"
database_ts_add key="test" timestamp="*" value=1.0
database_bf_add key="test" item="foo"
database_ft_aggregate  # This is actually read-only, should work
```

### 8. Error Handling Tests

```
# Non-existent index
database_ft_search index="nonexistent" query="*"

# Invalid JSON
database_json_set key="test" path="$" value="not json"

# Non-existent time series
database_ts_get key="nonexistent:ts"

# Non-existent bloom filter
database_bf_exists key="nonexistent:bf" item="test"
```

---

## jpx Bugs (For Reference)

These were fixed in the jpx project. Test with correct syntax:

**Correct JMESPath literal syntax:**
- Single quotes `'text'` create **identifiers** (field references)
- Backticks with JSON `\`"text"\`` create **string literals**

```
# Correct: split on newline
split(info, `"\n"`)

# Correct: regex extract
regex_extract(info, `"\\w+"`)
```

---

## Key Files

- `crates/redisctl-mcp/src/server.rs` - MCP tool handlers (189 tools)
- `crates/redisctl-mcp/src/database_tools.rs` - Database operations layer
- `docs/mcp-discovery.json` - Discovery spec v0.3.0 (77 tools documented)

---

## Quick Reference: Tool Counts

| Category | Count |
|----------|-------|
| Cloud tools | ~50 |
| Enterprise tools | ~80 |
| Database tools | ~60 |
| **Total MCP handlers** | **189** |

| Module | Tools |
|--------|-------|
| RediSearch | 4 |
| RedisJSON | 8 |
| RedisTimeSeries | 5 |
| RedisBloom | 5 |
| **Module total** | **22** |
