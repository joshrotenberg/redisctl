# Databases (BDB) REST API Reference

## List All Databases
- **Method**: `GET`
- **Path**: `/v1/bdbs`
- **Authentication**: Basic Auth
- **Response**: `200 OK`

```json
[
  {
    "uid": 1,
    "name": "string",
    "type": "redis",
    "proxy_policy": "single",
    "dns_address_master": "string",
    "memory_size": 1073741824,
    "shard_count": 1,
    "placement": "dense",
    "replication": true,
    "persistence": "disabled",
    "eviction_policy": "noeviction",
    "authentication_redis_pass": "string"
  }
]
```

## Get Database Information
- **Method**: `GET`  
- **Path**: `/v1/bdbs/{uid}`
- **Authentication**: Basic Auth
- **Path Parameters**:
  - `uid`: Database ID (integer)
- **Response**: `200 OK` - Returns database object

## Create Database
- **Method**: `POST`
- **Path**: `/v1/bdbs`
- **Authentication**: Basic Auth
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
  "name": "string (required)",
  "memory_size": 1073741824,
  "type": "redis",
  "proxy_policy": "all-master-shards",
  "port": 12000,
  "authentication_redis_pass": "string",
  "replication": true,
  "replica_ha": true,
  "persistence": "aof",
  "aof_policy": "appendfsync-every-sec",
  "snapshot_policy": [
    {
      "secs": 3600,
      "writes": 100
    }
  ],
  "eviction_policy": "volatile-lru",
  "shard_count": 2,
  "shard_key_regex": [
    {
      "regex": ".*{(.*)}.*"
    }
  ],
  "module_list": [
    {
      "module_name": "search",
      "module_args": "PARTITIONS AUTO"
    }
  ],
  "crdt": false,
  "default_db_config": {
    "max_connections": 10000,
    "max_aof_file_size": 268435456
  }
}
```
- **Response**: `200 OK` - Returns created database object

## Update Database  
- **Method**: `PUT`
- **Path**: `/v1/bdbs/{uid}`
- **Authentication**: Basic Auth
- **Path Parameters**:
  - `uid`: Database ID (integer)
- **Content-Type**: `application/json`
- **Request Body**: Same fields as create (all optional)
- **Response**: `200 OK` - Returns updated database object

## Delete Database
- **Method**: `DELETE`
- **Path**: `/v1/bdbs/{uid}`
- **Authentication**: Basic Auth
- **Path Parameters**:
  - `uid`: Database ID (integer)
- **Response**: `200 OK`

## Get Database Statistics
- **Method**: `GET`
- **Path**: `/v1/bdbs/{uid}/stats`
- **Authentication**: Basic Auth
- **Path Parameters**:
  - `uid`: Database ID (integer)
- **Query Parameters**:
  - `interval`: Time interval (see cluster stats)
  - `stime`: Start time
  - `etime`: End time
- **Response**: `200 OK`

```json
{
  "intervals": [
    {
      "interval": "1min",
      "timestamps": [1234567890],
      "values": {
        "used_memory": [524288000],
        "ops_per_sec": [1500],
        "evicted_objects": [0],
        "expired_objects": [10],
        "hits": [950],
        "misses": [50],
        "incoming_traffic": [1048576],
        "outgoing_traffic": [2097152]
      }
    }
  ]
}
```

## Get Database Metrics
- **Method**: `GET`
- **Path**: `/v1/bdbs/{uid}/metrics`
- **Authentication**: Basic Auth
- **Path Parameters**:
  - `uid`: Database ID (integer)
- **Response**: `200 OK`

```json
{
  "uid": 1,
  "used_memory": 524288000,
  "ops_per_sec": 1500,
  "hit_ratio": 0.95,
  "connections": 25,
  "evicted_objects_per_sec": 0,
  "expired_objects_per_sec": 0.5
}
```

## Database Actions

### Start Database
- **Method**: `POST`
- **Path**: `/v1/bdbs/{uid}/actions/start`
- **Authentication**: Basic Auth
- **Response**: `200 OK`

### Stop Database
- **Method**: `POST`
- **Path**: `/v1/bdbs/{uid}/actions/stop`
- **Authentication**: Basic Auth
- **Response**: `200 OK`

### Restart Database
- **Method**: `POST`
- **Path**: `/v1/bdbs/{uid}/actions/restart`
- **Authentication**: Basic Auth
- **Response**: `200 OK`

### Export Database
- **Method**: `POST`
- **Path**: `/v1/bdbs/{uid}/actions/export`
- **Authentication**: Basic Auth
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
  "export_location": "ftp://user:pass@host/path/backup.rdb"
}
```
- **Response**: `200 OK`

### Import Database
- **Method**: `POST`
- **Path**: `/v1/bdbs/{uid}/actions/import`
- **Authentication**: Basic Auth
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
  "import_location": "ftp://user:pass@host/path/backup.rdb",
  "flush": true
}
```
- **Response**: `200 OK`

## Common Database Fields

### Memory Size
- Format: Integer (bytes)
- Example: `1073741824` (1GB)
- Minimum: `262144` (256KB)

### Persistence Types
- `"disabled"` - No persistence
- `"aof"` - Append-only file
- `"snapshot"` - RDB snapshots

### Eviction Policies
- `"noeviction"` - Return error when memory limit reached
- `"allkeys-lru"` - Evict any key, least recently used first
- `"volatile-lru"` - Evict keys with TTL, LRU first
- `"allkeys-lfu"` - Evict any key, least frequently used first
- `"volatile-lfu"` - Evict keys with TTL, LFU first
- `"allkeys-random"` - Evict random keys
- `"volatile-random"` - Evict random keys with TTL
- `"volatile-ttl"` - Evict keys with shortest TTL

### Proxy Policies
- `"single"` - Single endpoint
- `"all-master-shards"` - All master shards
- `"all-nodes"` - All cluster nodes

## Error Codes
- `400` - Invalid configuration
- `409` - Database name already exists
- `507` - Insufficient memory/resources
- `503` - Database operation in progress
