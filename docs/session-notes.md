# MCP Database Tools & jpx Discovery - Session Notes

## Current Status (Updated after JSON session)

**Branch:** `feat/mcp-database-tools`

**Recent commits:**
- `d3e8484` - feat(mcp): add RedisJSON tools for array and object operations (+525 lines)
- `7c8f825` - feat(mcp): add RediSearch index management, query debugging, and autocomplete tools (+2738 lines)

**What we built:**
- 200+ MCP tool handlers total
- Full CRUD operations for all Redis data types
- Comprehensive Redis Stack module support:
  - **RediSearch**: 21 tools (search, aggregate, index management, aliases, autocomplete, synonyms)
  - **RedisJSON**: 18 tools (full JSON document manipulation)
  - **RedisTimeSeries**: 5 tools (samples, ranges, aggregation)
  - **RedisBloom**: 5 tools (bloom filter operations)

---

## Next Session: Testing & Index Tuning Plan

### Goal
Add test data for both HASH and JSON document types, build and execute searches, measure performance, and tune indexes. Also validate jpx discovery integration with Redis command schemas.

### 1. Create Test Data Sets

**HASH-based product catalog:**
```bash
# Use database_execute for HSET operations
database_execute command="HSET" args=["product:hash:1", "name", "Gaming Laptop", "category", "electronics", "price", "1299.99", "brand", "TechPro", "description", "High-performance gaming laptop with RTX 4080"]
database_execute command="HSET" args=["product:hash:2", "name", "Wireless Mouse", "category", "electronics", "price", "49.99", "brand", "LogiTech", "description", "Ergonomic wireless mouse with precision tracking"]
# ... add 50-100 products for meaningful testing
```

**JSON-based product catalog:**
```bash
database_json_set key="product:json:1" path="$" value='{"name":"Gaming Laptop","category":"electronics","subcategory":"computers","price":1299.99,"brand":"TechPro","specs":{"ram":"32GB","storage":"1TB SSD","gpu":"RTX 4080"},"tags":["gaming","portable","high-performance"],"description":"High-performance gaming laptop with RTX 4080","rating":4.8,"reviews_count":245}'
# ... add matching JSON products
```

### 2. Create Search Indexes

**HASH index:**
```bash
database_ft_create index="idx:products:hash" on="HASH" prefixes=["product:hash:"] schema=[
  {"name":"name","field_type":"TEXT","weight":2.0,"sortable":true},
  {"name":"category","field_type":"TAG"},
  {"name":"brand","field_type":"TAG"},
  {"name":"price","field_type":"NUMERIC","sortable":true},
  {"name":"description","field_type":"TEXT"}
]
```

**JSON index (with nested fields):**
```bash
database_ft_create index="idx:products:json" on="JSON" prefixes=["product:json:"] schema=[
  {"name":"$.name","alias":"name","field_type":"TEXT","weight":2.0,"sortable":true},
  {"name":"$.category","alias":"category","field_type":"TAG"},
  {"name":"$.subcategory","alias":"subcategory","field_type":"TAG"},
  {"name":"$.brand","alias":"brand","field_type":"TAG"},
  {"name":"$.price","alias":"price","field_type":"NUMERIC","sortable":true},
  {"name":"$.tags[*]","alias":"tags","field_type":"TAG"},
  {"name":"$.description","alias":"description","field_type":"TEXT"},
  {"name":"$.rating","alias":"rating","field_type":"NUMERIC","sortable":true}
]
```

### 3. Search Queries to Test

**Basic queries:**
```bash
database_ft_search index="idx:products:hash" query="laptop" withscores=true
database_ft_search index="idx:products:json" query="laptop" withscores=true
```

**Filtered queries:**
```bash
database_ft_search index="idx:products:json" query="@category:{electronics} @price:[0 500]" sortby="price"
database_ft_search index="idx:products:json" query="@tags:{gaming|portable}" limit_num=20
```

**Aggregations:**
```bash
database_ft_aggregate index="idx:products:json" query="*" groupby=[{"properties":["@category"],"reducers":[{"function":"COUNT","alias":"count"},{"function":"AVG","args":["@price"],"alias":"avg_price"}]}]
```

### 4. Performance Testing & Tuning

**Measure query times:**
```bash
# Use FT.PROFILE for query analysis
database_execute command="FT.PROFILE" args=["idx:products:json", "SEARCH", "QUERY", "@category:{electronics}"]
```

**Index tuning experiments:**
- Compare SORTABLE vs non-SORTABLE fields (memory vs query speed)
- Test NOSTEM on specific fields for exact matching
- Adjust TEXT field weights for relevance tuning
- Try TAG vs TEXT for categorical fields

**Use FT.EXPLAIN to understand query plans:**
```bash
database_ft_explain index="idx:products:json" query="@category:{electronics} @price:[100 500]"
```

### 5. jpx Discovery Integration Testing

**Register Redis tools with jpx:**
```bash
# Test that jpx can index our tool schemas
mcp__jpx__query_tools query="redis search"
mcp__jpx__query_tools query="json array"
```

**Test search result transformation with jpx:**
```bash
# Get search results and transform with JMESPath
database_ft_search index="idx:products:json" query="laptop"
# Then pipe through jpx to extract/reshape results
mcp__jpx__evaluate input='<search_results>' expression='results[*].{name: name, price: price}'
```

**Validate tool discoverability:**
- Check jpx can find tools by functionality ("how to create an index")
- Check tool descriptions are being indexed
- Test fuzzy matching on tool names

### 6. Continue Implementation

After testing, continue adding tools:
- **RedisTimeSeries**: TS.MRANGE, TS.MGET, TS.QUERYINDEX
- **RedisBloom**: CF.* (Cuckoo filters), CMS.* (Count-Min Sketch), TOPK.*
- **RediSearch**: FT.PROFILE (dedicated tool), FT.CURSOR commands

### 7. PR Update

Once testing validates the implementations:
```bash
git push origin feat/mcp-database-tools
# Update PR description with:
# - Tool counts and categories
# - Testing results
# - jpx integration notes
```

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

## Redis Stack Module Tools Summary

### RediSearch (21 tools)
**Search & Aggregation:**
- `database_ft_search` - Full-text search with filtering, sorting, pagination, highlighting, scoring
- `database_ft_aggregate` - Complex aggregations with GROUPBY, REDUCE, APPLY, SORTBY
- `database_ft_info` - Index schema, document count, memory usage
- `database_ft_list` - List all indexes

**Index Management:**
- `database_ft_create` - Create search index with full schema support (TEXT, TAG, NUMERIC, GEO, VECTOR)
- `database_ft_dropindex` - Delete index (optionally with documents)
- `database_ft_alter` - Add fields to existing index

**Query Debugging:**
- `database_ft_explain` - Get query execution plan
- `database_ft_tagvals` - Get unique values for TAG field
- `database_ft_spellcheck` - Suggest spelling corrections

**Aliases:**
- `database_ft_aliasadd` - Create index alias
- `database_ft_aliasdel` - Delete index alias
- `database_ft_aliasupdate` - Update index alias

**Autocomplete:**
- `database_ft_sugadd` - Add autocomplete suggestion
- `database_ft_sugget` - Get autocomplete suggestions
- `database_ft_sugdel` - Delete suggestion
- `database_ft_suglen` - Get suggestion dictionary size

**Synonyms:**
- `database_ft_syndump` - Get all synonym groups
- `database_ft_synupdate` - Update synonym group

### RedisJSON (18 tools)
**Core Operations:**
- `database_json_get` - Get JSON at paths (JSONPath syntax)
- `database_json_set` - Set JSON with NX/XX conditions
- `database_json_del` - Delete JSON at path
- `database_json_type` - Get value type
- `database_json_mget` - Get values from multiple keys

**Object Operations:**
- `database_json_objkeys` - Get all keys from JSON object
- `database_json_objlen` - Get number of keys in object

**Array Operations:**
- `database_json_arrappend` - Append to arrays
- `database_json_arrlen` - Get array length
- `database_json_arrindex` - Find element index in array
- `database_json_arrpop` - Pop element from array
- `database_json_arrtrim` - Trim array to range
- `database_json_arrinsert` - Insert elements into array

**Other Operations:**
- `database_json_numincrby` - Atomic increment/decrement numbers
- `database_json_strlen` - Get string length
- `database_json_clear` - Clear containers or set numbers to 0
- `database_json_toggle` - Toggle boolean values
- `database_json_forget` - Alias for JSON.DEL

### RedisTimeSeries (5 tools)
- `database_ts_add` - Add samples with retention, encoding, labels
- `database_ts_get` - Get latest sample
- `database_ts_range` - Query with aggregation (avg, sum, min, max, etc.)
- `database_ts_info` - Time series metadata
- `database_ts_create` - Create with options

### RedisBloom (5 tools)
- `database_bf_reserve` - Create filter with error rate and capacity
- `database_bf_add` - Add single item
- `database_bf_madd` - Bulk add items
- `database_bf_exists` - Check single item
- `database_bf_mexists` - Bulk check items
- `database_bf_info` - Filter metadata

---

## Key Files

- `crates/redisctl-mcp/src/server.rs` - MCP tool handlers (200+ tools)
- `crates/redisctl-mcp/src/database_tools.rs` - Database operations layer
- `docs/mcp-discovery.json` - Discovery spec (needs update with new tools)

---

## Quick Reference: Tool Counts

| Category | Count |
|----------|-------|
| Cloud tools | ~50 |
| Enterprise tools | ~80 |
| Database tools | ~70 |
| **Total MCP handlers** | **200+** |

| Module | Tools |
|--------|-------|
| RediSearch | 21 |
| RedisJSON | 18 |
| RedisTimeSeries | 5 |
| RedisBloom | 5 |
| **Module total** | **49** |
