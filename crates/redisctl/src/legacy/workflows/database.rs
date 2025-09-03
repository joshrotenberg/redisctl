//! Database creation workflows

use anyhow::Result;
use redis_enterprise::EnterpriseClient;
use serde_json::Value;
use tracing::info;

/// Create a database with best practices for the specified type
pub async fn create_database(
    client: &EnterpriseClient,
    name: String,
    db_type: String,
) -> Result<Value> {
    info!("Creating {} database '{}'", db_type, name);
    
    let config = match db_type.as_str() {
        "cache" => create_cache_config(name.clone()),
        "persistent" => create_persistent_config(name.clone()),
        "search" => create_search_config(name.clone()),
        "timeseries" => create_timeseries_config(name.clone()),
        "json" => create_json_config(name.clone()),
        "graph" => create_graph_config(name.clone()),
        _ => anyhow::bail!("Unknown database type: {}. Supported types: cache, persistent, search, timeseries, json, graph", db_type)
    };
    
    let result = client.post_raw("/v1/bdbs", config).await?;
    
    Ok(serde_json::json!({
        "success": true,
        "message": format!("Database '{}' created with {} configuration", name, db_type),
        "database": result
    }))
}

/// Create configuration for a cache database
fn create_cache_config(name: String) -> Value {
    serde_json::json!({
        "name": name,
        "memory_size": 536870912, // 512MB
        "eviction_policy": "allkeys-lru",
        "persistence": "disabled",
        "replication": false,
        "sharding": false,
        "port": 0, // Auto-assign
        "proxy_policy": "single",
        "oss_cluster": false
    })
}

/// Create configuration for a persistent database
fn create_persistent_config(name: String) -> Value {
    serde_json::json!({
        "name": name,
        "memory_size": 1073741824, // 1GB
        "persistence": "aof",
        "aof_policy": "appendfsync-every-sec",
        "snapshot_policy": [{
            "hours": 23,
            "minutes": 0
        }],
        "replication": true,
        "eviction_policy": "noeviction",
        "port": 0, // Auto-assign
        "proxy_policy": "single",
        "oss_cluster": false
    })
}

/// Create configuration for a RediSearch database
fn create_search_config(name: String) -> Value {
    serde_json::json!({
        "name": name,
        "memory_size": 2147483648u64, // 2GB
        "module_list": [{
            "module_args": "PARTITIONS AUTO",
            "module_id": "search",
            "module_name": "search"
        }],
        "persistence": "aof",
        "aof_policy": "appendfsync-every-sec",
        "replication": true,
        "eviction_policy": "noeviction",
        "port": 0, // Auto-assign
        "proxy_policy": "single"
    })
}

/// Create configuration for a RedisTimeSeries database
fn create_timeseries_config(name: String) -> Value {
    serde_json::json!({
        "name": name,
        "memory_size": 1073741824, // 1GB
        "module_list": [{
            "module_args": "",
            "module_id": "timeseries",
            "module_name": "timeseries"
        }],
        "persistence": "aof",
        "aof_policy": "appendfsync-every-sec",
        "replication": true,
        "eviction_policy": "noeviction",
        "port": 0, // Auto-assign
        "proxy_policy": "single"
    })
}

/// Create configuration for a RedisJSON database
fn create_json_config(name: String) -> Value {
    serde_json::json!({
        "name": name,
        "memory_size": 1073741824, // 1GB
        "module_list": [{
            "module_args": "",
            "module_id": "ReJSON",
            "module_name": "ReJSON"
        }],
        "persistence": "aof",
        "aof_policy": "appendfsync-every-sec",
        "replication": true,
        "eviction_policy": "noeviction",
        "port": 0, // Auto-assign
        "proxy_policy": "single"
    })
}

/// Create configuration for a RedisGraph database
fn create_graph_config(name: String) -> Value {
    serde_json::json!({
        "name": name,
        "memory_size": 2147483648u64, // 2GB
        "module_list": [{
            "module_args": "",
            "module_id": "graph",
            "module_name": "graph"
        }],
        "persistence": "aof",
        "aof_policy": "appendfsync-every-sec",
        "replication": true,
        "eviction_policy": "noeviction",
        "port": 0, // Auto-assign
        "proxy_policy": "single"
    })
}