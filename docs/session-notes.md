# MCP Database Tools & jpx Discovery - Session Notes

## Current Status (Updated after Search Index Optimization session)

**Branch:** `feat/mcp-database-tools`

**Recent work:**
- Loaded 5000 product documents using MCP pipeline
- Created naive vs optimized search indexes
- Benchmarked and documented index optimization best practices
- **Key finding: TAG fields + SORTABLE = up to 6x faster queries**

**What we built:**
- 200+ MCP tool handlers total (now ~101 database tools)
- Full CRUD operations for all Redis data types
- Comprehensive Redis Stack module support
- **Pipeline support for bulk operations** (tested & benchmarked)
- **Search index optimization workflow** (documented below)

---

## IMPORTANT: Session Checklist

### Before ending a session:
1. **Always rebuild the binary in release mode:**
   ```bash
   cargo build --release
   ```
2. Commit any changes
3. Update this notes file with current status

### At start of a session:
1. Check the binary is current: `ls -la target/release/redisctl`
2. Restart Claude Code if needed to pick up new MCP tools
3. Use dedicated tools (e.g., `database_hset`) instead of `database_execute` when available

---

## New: Pipeline Tool

Added `database_pipeline` for executing multiple Redis commands in a single network round-trip:

```bash
# Example: Bulk insert with pipeline
database_pipeline commands=[
  {"command": "HSET", "args": ["product:1", "name", "Laptop", "price", "999"]},
  {"command": "HSET", "args": ["product:2", "name", "Mouse", "price", "49"]},
  {"command": "HSET", "args": ["product:3", "name", "Keyboard", "price", "149"]}
] atomic=false
```

**Benefits:**
- Reduces N network round-trips to 1
- `atomic=true` wraps in MULTI/EXEC for transactional execution
- Returns individual results for each command with timing info

**Files changed:**
- `crates/redisctl-mcp/src/database_tools.rs` - Added `execute_pipeline()` method and `PipelineCommand` struct
- `crates/redisctl-mcp/src/server.rs` - Added `database_pipeline` tool handler

---

## Test Data in Redis (Port 6380)

**Large dataset (5000 products):**
- `product:0` through `product:4999` - JSON documents
- Fields: id, name, brand, category, product_type, color, price, rating, review_count, stock, condition, availability, description, tags

**Indexes:**
- `idx:products:naive` - Naive index (5000 docs, ~2.65 MB) - all TEXT fields
- `idx:products:optimized` - Optimized index (5000 docs, ~3.12 MB) - TAG + SORTABLE
- `idx:products:hash` - HASH index (50 docs, ~0.11 MB) - from earlier session
- `idx:products:json` - JSON index (50 docs, ~0.12 MB) - from earlier session

**jpx Discovery:** Registered 16 redisctl tools for search.

---

## Tool Usage Notes

### Use dedicated tools, not database_execute!
The MCP server has 100+ database tools. Use them instead of raw commands:

| Instead of... | Use... |
|--------------|--------|
| `database_execute("HSET", [...])` | `database_hset(key, field, value)` |
| `database_execute("JSON.SET", [...])` | `database_json_set(key, path, value)` |
| `database_execute("SET", [...])` | `database_set(key, value)` |
| `database_execute("FT.SEARCH", [...])` | `database_ft_search(index, query, ...)` |

**Why?** Dedicated tools:
- Have structured parameters (no need to know Redis syntax)
- Have better error messages
- Are discoverable via jpx
- Have proper JSON schema documentation

### For bulk operations:
Use `database_pipeline` instead of multiple individual calls - significantly faster.

---

## Pipeline Performance Benchmark (Completed)

Tested 100 HSET commands with 3 fields each:

| Method | Time | Speedup |
|--------|------|---------|
| Pipeline (1 round-trip) | 5.43ms | **380x faster** |
| Individual calls (100 round-trips) | 2065ms | baseline |

**Key insight:** The ~380x speedup comes from eliminating network round-trip latency. Each individual call pays ~20ms of network overhead, while pipeline batches everything into a single round-trip.

---

## Search Index Optimization Case Study (Completed)

### Scenario
Loaded 5000 JSON product documents with fields: name, brand, category, product_type, color, price, rating, review_count, stock, condition, availability, description, tags.

### Naive Index (idx:products:naive)
Created with all fields as TEXT (except numerics):
```
FT.CREATE idx:products:naive ON JSON PREFIX 1 product: SCHEMA
  $.name AS name TEXT
  $.brand AS brand TEXT          # ❌ Should be TAG
  $.category AS category TEXT    # ❌ Should be TAG
  $.tags[*] AS tags TEXT         # ❌ Should be TAG
  $.price AS price NUMERIC       # ❌ Not SORTABLE
  ...
```

**Problem identified with FT.EXPLAIN:**
```
@brand:Sony  →  UNION { @brand:sony, @brand:+soni(expanded), @brand:soni(expanded) }
```
TEXT fields get stemmed, causing unnecessary work for exact-match fields.

### Optimized Index (idx:products:optimized)
```
FT.CREATE idx:products:optimized ON JSON PREFIX 1 product: SCHEMA
  $.name AS name TEXT WEIGHT 2 SORTABLE    # ✅ Boosted, sortable
  $.brand AS brand TAG SORTABLE            # ✅ Exact match, sortable
  $.category AS category TAG               # ✅ Exact match
  $.tags[*] AS tags TAG                    # ✅ Exact match
  $.price AS price NUMERIC SORTABLE        # ✅ Sortable
  $.rating AS rating NUMERIC SORTABLE      # ✅ Sortable
  $.description AS description TEXT WEIGHT 0.5  # ✅ Lower weight
  ...
```

### Benchmark Results (5 runs averaged)

| Query                           | Naive (ms) | Optimized (ms) | Speedup |
|---------------------------------|------------|----------------|---------|
| Simple brand: Sony              | 1.46       | 0.25           | **5.9x** |
| Category: electronics           | 0.66       | 0.28           | **2.3x** |
| Description: premium quality    | 2.93       | 0.69           | **4.3x** |
| Search + sort by price          | 0.70       | 0.29           | **2.4x** |
| Tag search: bestseller          | 0.34       | 0.28           | **1.2x** |

### Best Practices for Index Design

1. **Use TAG for exact-match fields** (brand, category, status, tags, IDs)
   - No stemming overhead
   - Use `@field:{value}` syntax

2. **Add SORTABLE to fields you'll sort/aggregate on**
   - Trades memory for query speed
   - Essential for price, date, rating fields

3. **Weight TEXT fields appropriately**
   - Boost important fields (name: 2.0)
   - Lower weight for verbose fields (description: 0.5)

4. **Use FT.EXPLAIN to verify query plans**
   - Identify stemming bloat
   - Check for inefficient UNIONs

### Index Memory Comparison

| Index     | Total Size | Sortable Values | Notes |
|-----------|------------|-----------------|-------|
| Naive     | 2.65 MB    | 0 MB            | No sorting capability |
| Optimized | 3.12 MB    | 0.73 MB         | +18% for 2-6x faster queries |

---

## Competitive Analysis (vs mcp-redis, mcp-redis-cloud)

### Tool Count Comparison
| Project | Cloud | Enterprise | Database | Total |
|---------|-------|------------|----------|-------|
| **redisctl** | ~50 | ~80 | ~101 | **~231** |
| mcp-redis | 0 | 0 | 46 | 46 |
| mcp-redis-cloud | 23 | 0 | 0 | 23 |

### redisctl's Unique Value Proposition
1. **Only unified solution** - Cloud + Enterprise management + Database access
2. **Bridging scenarios** - Allocate resources via API, then migrate data via commands
3. **Comprehensive** - More tools in each category than competitors combined
4. **Production-ready** - Built in Rust, proper error handling, TLS support

---

## Next Session: Work Chunks Scoped

### Chunk 1: Streams Support (Priority: HIGH)

**Gap:** mcp-redis has 3 basic stream tools; redisctl has 0

**Recommended Tools (10-12):**
| Tool | Description | Priority |
|------|-------------|----------|
| `database_xadd` | Add entry to stream | High |
| `database_xread` | Read entries (blocking/non-blocking) | High |
| `database_xrange` | Range query with start/end | High |
| `database_xrevrange` | Reverse range query | Medium |
| `database_xlen` | Get stream length | High |
| `database_xinfo_stream` | Stream metadata | Medium |
| `database_xinfo_groups` | Consumer group info | Medium |
| `database_xgroup_create` | Create consumer group | High |
| `database_xreadgroup` | Consumer group read | High |
| `database_xack` | Acknowledge messages | High |
| `database_xdel` | Delete entries | Medium |
| `database_xtrim` | Trim stream | Medium |

**Why high priority:** Streams are essential for:
- Real-time data pipelines
- Event sourcing
- Consumer group patterns (production use)
- mcp-redis's implementation is minimal

---

### Chunk 2: Pub/Sub Support (Priority: MEDIUM)

**Gap:** mcp-redis has 3 pub/sub tools; redisctl has 0

**Challenge:** True subscribe doesn't fit MCP request/response model.
mcp-redis's `subscribe()` just returns "Subscribed" but doesn't listen.

**Recommended Tools (3-4, publish-side focus):**
| Tool | Description | Priority |
|------|-------------|----------|
| `database_publish` | Publish message to channel | High |
| `database_pubsub_channels` | List active channels | Medium |
| `database_pubsub_numsub` | Subscriber count per channel | Medium |
| `database_pubsub_numpat` | Pattern subscription count | Low |

**Note:** For actual message consumption, Streams with consumer groups is more appropriate for MCP.

---

### Chunk 3: Vector Search (Priority: LOW - Skip)

**Analysis:** mcp-redis has 5 simplified vector tools, but:
- **No auto-embedding** - requires pre-computed vectors
- Their OpenAI integration is for running the agent, not embeddings
- Limited to HNSW on HASH only

**redisctl already has:**
- `ft_create` with full VECTOR field support (FLAT + HNSW)
- `ft_search` can do KNN queries
- Works with HASH and JSON

**Verdict:** Our existing tools are MORE flexible. No work needed.

---

### Chunk 4: Docs Search (Priority: LOW - Skip)

**mcp-redis implementation:** Calls external RAG API (`MCP_DOCS_SEARCH_URL`)

**Analysis:**
- Not self-contained (requires hosted service)
- Their API is Redis-internal
- Our tool descriptions are comprehensive

**Verdict:** Low ROI. Our tools are self-documenting.

---

### Implementation Priority Order
1. **Streams** - High value, fills major gap
2. **Pub/Sub (publish side)** - Quick win
3. Skip vectors - already covered
4. Skip docs search - low ROI

---

## Additional Future Tasks

### Continue Redis Stack Implementation
- **RedisTimeSeries**: TS.MRANGE, TS.MGET, TS.QUERYINDEX
- **RedisBloom**: CF.* (Cuckoo filters), CMS.* (Count-Min Sketch), TOPK.*
- **RediSearch**: FT.PROFILE (dedicated tool), FT.CURSOR commands

### Update mcp-discovery.json
Add the new pipeline tool and ensure all tools are documented for jpx discovery.

---

## Redis Stack Module Tools Summary

### RediSearch (21 tools)
- `database_ft_search` - Full-text search
- `database_ft_aggregate` - Aggregations
- `database_ft_create` - Create index
- `database_ft_dropindex` - Delete index
- `database_ft_alter` - Add fields
- `database_ft_info` - Index info
- `database_ft_list` - List indexes
- `database_ft_explain` - Query plan
- `database_ft_tagvals` - TAG values
- `database_ft_spellcheck` - Spelling suggestions
- `database_ft_aliasadd/del/update` - Alias management
- `database_ft_sugadd/get/del/len` - Autocomplete
- `database_ft_syndump/synupdate` - Synonyms

### RedisJSON (18 tools)
- `database_json_get/set/del/type/mget` - Core operations
- `database_json_objkeys/objlen` - Object operations
- `database_json_arrappend/arrlen/arrindex/arrpop/arrtrim/arrinsert` - Array operations
- `database_json_numincrby/strlen/clear/toggle` - Other operations

### RedisTimeSeries (5 tools)
- `database_ts_add/get/range/info/create`

### RedisBloom (5 tools)
- `database_bf_reserve/add/madd/exists/mexists/info`

### Core Database (50+ tools)
- String: `database_get/set/incr/decr/incrby`
- Hash: `database_hget/hset/hgetall/hdel/hlen/hset_multiple`
- List: `database_lpush/rpush/lpop/rpop/lrange/llen/lindex/lset`
- Set: `database_sadd/srem/smembers/sismember/scard`
- Sorted Set: `database_zadd/zrem/zrange/zrevrange/zscore/zrank/zcard/zincrby/...`
- Keys: `database_scan/exists/type/ttl/del/expire/persist/rename/memory_usage`
- Server: `database_ping/info/dbsize/slowlog/client_list/config_get/module_list`
- **NEW**: `database_pipeline` - Bulk operations

---

## Key Files

| File | Purpose |
|------|---------|
| `crates/redisctl-mcp/src/server.rs` | MCP tool handlers (200+ tools) |
| `crates/redisctl-mcp/src/database_tools.rs` | Database operations layer |
| `docs/mcp-discovery.json` | jpx discovery spec |
| `.mcp.json` | MCP server config (includes `--allow-writes`) |

---

## Quick Reference

| Category | Count |
|----------|-------|
| Cloud tools | ~50 |
| Enterprise tools | ~80 |
| Database tools | ~101 |
| **Total** | **~231** |
