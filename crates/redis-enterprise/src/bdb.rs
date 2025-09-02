//! Database (BDB) management commands for Redis Enterprise
//!
//! Overview
//! - Create, update, delete databases
//! - Operational actions: start/stop/restart, export/import/backup/restore
//! - Insights: endpoints, shards, metrics/stats, availability
//! - Advanced: passwords, modules config/upgrade, traffic control
//!
//! Tip: For time-series metrics, also see the `StatsHandler` for aggregate queries.

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

// Aliases for easier use
pub type Database = DatabaseInfo;
pub type BdbHandler = DatabaseHandler;

/// Response from database action operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseActionResponse {
    /// The action UID for tracking async operations
    pub action_uid: String,
    /// Description of the action
    pub description: Option<String>,
    /// Additional fields from the response
    #[serde(flatten)]
    pub extra: Value,
}

/// Response from backup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupResponse {
    /// The action UID for tracking the backup operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_uid: Option<String>,
    /// Backup UID if available
    pub backup_uid: Option<String>,
    /// Additional fields from the response
    #[serde(flatten)]
    pub extra: Value,
}

/// Response from import operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResponse {
    /// The action UID for tracking the import operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_uid: Option<String>,
    /// Import status
    pub status: Option<String>,
    /// Additional fields from the response
    #[serde(flatten)]
    pub extra: Value,
}

/// Response from export operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResponse {
    /// The action UID for tracking the export operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_uid: Option<String>,
    /// Export status
    pub status: Option<String>,
    /// Additional fields from the response
    #[serde(flatten)]
    pub extra: Value,
}

/// Database information from the REST API - 100% field coverage (152/152 fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    // Core database identification and status
    pub uid: u32,
    pub name: String,
    pub port: Option<u16>,
    pub status: Option<String>,
    pub memory_size: Option<u64>,
    pub memory_used: Option<u64>,

    /// Database type (e.g., "redis", "memcached")
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub version: Option<String>,

    /// Account and action tracking
    pub account_id: Option<u32>,
    pub action_uid: Option<String>,

    // Sharding and placement
    pub shards_count: Option<u32>,
    pub shard_list: Option<Vec<u32>>,
    pub sharding: Option<bool>,
    pub shards_placement: Option<String>,
    pub replication: Option<bool>,

    // Endpoints and networking
    pub endpoints: Option<Vec<EndpointInfo>>,
    pub endpoint: Option<String>,
    pub endpoint_ip: Option<Vec<String>>,
    pub endpoint_node: Option<u32>,
    pub dns_address_master: Option<String>,

    // Data persistence and backup
    pub persistence: Option<String>,
    pub data_persistence: Option<String>,
    pub eviction_policy: Option<String>,

    // Timestamps
    pub created_time: Option<String>,
    pub last_changed_time: Option<String>,
    pub last_backup_time: Option<String>,
    pub last_export_time: Option<String>,

    // Security and authentication
    pub mtls_allow_weak_hashing: Option<bool>,
    pub mtls_allow_outdated_certs: Option<bool>,
    pub authentication_redis_pass: Option<String>,
    pub authentication_admin_pass: Option<String>,
    pub authentication_sasl_pass: Option<String>,
    pub authentication_sasl_uname: Option<String>,
    pub authentication_ssl_client_certs: Option<Vec<Value>>,
    pub authentication_ssl_crdt_certs: Option<Vec<Value>>,
    pub authorized_subjects: Option<Vec<Value>>,
    pub data_internode_encryption: Option<bool>,
    pub ssl: Option<bool>,
    pub tls_mode: Option<String>,
    pub enforce_client_authentication: Option<String>,
    pub default_user: Option<bool>,

    // CRDT/Active-Active fields
    pub crdt: Option<bool>,
    pub crdt_enabled: Option<bool>,
    pub crdt_config_version: Option<u32>,
    pub crdt_replica_id: Option<u32>,
    pub crdt_ghost_replica_ids: Option<String>,
    pub crdt_featureset_version: Option<u32>,
    pub crdt_protocol_version: Option<u32>,
    pub crdt_guid: Option<String>,
    pub crdt_modules: Option<String>,
    pub crdt_replicas: Option<String>,
    pub crdt_sources: Option<Vec<Value>>,
    pub crdt_sync: Option<String>,
    pub crdt_sync_connection_alarm_timeout_seconds: Option<u32>,
    pub crdt_sync_dist: Option<bool>,
    pub crdt_syncer_auto_oom_unlatch: Option<bool>,
    pub crdt_xadd_id_uniqueness_mode: Option<String>,
    pub crdt_causal_consistency: Option<bool>,
    pub crdt_repl_backlog_size: Option<String>,

    // Replication settings
    pub master_persistence: Option<String>,
    pub slave_ha: Option<bool>,
    pub slave_ha_priority: Option<u32>,
    pub replica_read_only: Option<bool>,
    pub replica_sources: Option<Vec<Value>>,
    pub replica_sync: Option<String>,
    pub replica_sync_connection_alarm_timeout_seconds: Option<u32>,
    pub replica_sync_dist: Option<bool>,
    pub repl_backlog_size: Option<String>,

    // Connection and performance settings
    pub max_connections: Option<u32>,
    pub maxclients: Option<u32>,
    pub conns: Option<u32>,
    pub conns_type: Option<String>,
    pub max_client_pipeline: Option<u32>,
    pub max_pipelined: Option<u32>,

    // AOF (Append Only File) settings
    pub aof_policy: Option<String>,
    pub max_aof_file_size: Option<String>,
    pub max_aof_load_time: Option<u32>,

    // Active defragmentation settings
    pub activedefrag: Option<String>,
    pub active_defrag_cycle_max: Option<u32>,
    pub active_defrag_cycle_min: Option<u32>,
    pub active_defrag_ignore_bytes: Option<String>,
    pub active_defrag_max_scan_fields: Option<u32>,
    pub active_defrag_threshold_lower: Option<u32>,
    pub active_defrag_threshold_upper: Option<u32>,

    // Backup settings
    pub backup: Option<bool>,
    pub backup_failure_reason: Option<String>,
    pub backup_history: Option<u32>,
    pub backup_interval: Option<u32>,
    pub backup_interval_offset: Option<u32>,
    pub backup_location: Option<Value>,
    pub backup_progress: Option<f64>,
    pub backup_status: Option<String>,

    // Import/Export settings
    pub dataset_import_sources: Option<Vec<Value>>,
    pub import_failure_reason: Option<String>,
    pub import_progress: Option<f64>,
    pub import_status: Option<String>,
    pub export_failure_reason: Option<String>,
    pub export_progress: Option<f64>,
    pub export_status: Option<String>,
    pub skip_import_analyze: Option<bool>,

    // Monitoring and metrics
    pub metrics_export_all: Option<bool>,
    pub generate_text_monitor: Option<bool>,
    pub email_alerts: Option<bool>,

    // Modules and features
    pub module_list: Option<Vec<Value>>,
    pub search: Option<bool>,
    pub timeseries: Option<bool>,

    // BigStore/Flash storage settings
    pub bigstore: Option<bool>,
    pub bigstore_ram_size: Option<u64>,
    pub bigstore_max_ram_ratio: Option<u32>,
    pub bigstore_ram_weights: Option<Vec<Value>>,
    pub bigstore_version: Option<u32>,

    // Network and proxy settings
    pub proxy_policy: Option<String>,
    pub oss_cluster: Option<bool>,
    pub oss_cluster_api_preferred_endpoint_type: Option<String>,
    pub oss_cluster_api_preferred_ip_type: Option<String>,
    pub oss_sharding: Option<bool>,

    // Redis-specific settings
    pub redis_version: Option<String>,
    pub resp3: Option<bool>,
    pub disabled_commands: Option<String>,

    // Clustering and sharding
    pub hash_slots_policy: Option<String>,
    pub shard_key_regex: Option<Vec<Value>>,
    pub shard_block_crossslot_keys: Option<bool>,
    pub shard_block_foreign_keys: Option<bool>,
    pub implicit_shard_key: Option<bool>,

    // Node placement and rack awareness
    pub avoid_nodes: Option<Vec<String>>,
    pub use_nodes: Option<Vec<String>>,
    pub rack_aware: Option<bool>,

    // Operational settings
    pub auto_upgrade: Option<bool>,
    pub internal: Option<bool>,
    pub db_conns_auditing: Option<bool>,
    pub flush_on_fullsync: Option<bool>,
    pub use_selective_flush: Option<bool>,

    // Sync and replication control
    pub sync: Option<String>,
    pub sync_sources: Option<Vec<Value>>,
    pub sync_dedicated_threads: Option<bool>,
    pub syncer_mode: Option<String>,
    pub syncer_log_level: Option<String>,
    pub support_syncer_reconf: Option<bool>,

    // Gradual sync settings
    pub gradual_src_mode: Option<String>,
    pub gradual_src_max_sources: Option<u32>,
    pub gradual_sync_mode: Option<String>,
    pub gradual_sync_max_shards_per_source: Option<u32>,

    // Slave and buffer settings
    pub slave_buffer: Option<String>,

    // Snapshot settings
    pub snapshot_policy: Option<Vec<Value>>,

    // Scheduling and recovery
    pub sched_policy: Option<String>,
    pub recovery_wait_time: Option<u32>,

    // Performance and optimization
    pub multi_commands_opt: Option<String>,
    pub throughput_ingress: Option<f64>,
    pub tracking_table_max_keys: Option<u32>,
    pub wait_command: Option<bool>,

    // Legacy and deprecated fields
    pub background_op: Option<Vec<Value>>,

    // Advanced configuration
    pub mkms: Option<bool>,
    pub roles_permissions: Option<Vec<Value>>,
    pub tags: Option<Vec<String>>,
    pub topology_epoch: Option<u32>,

    /// Capture any additional fields not explicitly defined
    #[serde(flatten)]
    pub extra: Value,
}

/// Database endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub addr: Option<Vec<String>>,
    pub port: Option<u16>,
    pub dns_name: Option<String>,
}

/// Module configuration for database creation
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ModuleConfig {
    #[builder(setter(into))]
    pub module_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub module_args: Option<String>,
}

/// Create database request
///
/// # Examples
///
/// ```rust,no_run
/// use redis_enterprise::{CreateDatabaseRequest, ModuleConfig};
///
/// let request = CreateDatabaseRequest::builder()
///     .name("my-database")
///     .memory_size(1024 * 1024 * 1024) // 1GB
///     .port(12000)
///     .replication(true)
///     .persistence("aof")
///     .eviction_policy("volatile-lru")
///     .shards_count(2)
///     .authentication_redis_pass("secure-password")
///     .build();
/// ```
#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct CreateDatabaseRequest {
    #[builder(setter(into))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub memory_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub replication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub persistence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub eviction_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sharding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub shards_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "shard_count")]
    #[builder(default, setter(strip_option))]
    pub shard_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub proxy_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub rack_aware: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub module_list: Option<Vec<ModuleConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub crdt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub authentication_redis_pass: Option<String>,
}

/// Database handler for executing database commands
pub struct DatabaseHandler {
    client: RestClient,
}

impl DatabaseHandler {
    pub fn new(client: RestClient) -> Self {
        DatabaseHandler { client }
    }

    /// List all databases (BDB.LIST)
    pub async fn list(&self) -> Result<Vec<DatabaseInfo>> {
        self.client.get("/v1/bdbs").await
    }

    /// Get specific database info (BDB.INFO)
    pub async fn info(&self, uid: u32) -> Result<DatabaseInfo> {
        self.client.get(&format!("/v1/bdbs/{}", uid)).await
    }

    /// Get specific database info (alias for info)
    pub async fn get(&self, uid: u32) -> Result<DatabaseInfo> {
        self.info(uid).await
    }

    /// Create a new database (BDB.CREATE)
    pub async fn create(&self, request: CreateDatabaseRequest) -> Result<DatabaseInfo> {
        self.client.post("/v1/bdbs", &request).await
    }

    /// Update database configuration (BDB.UPDATE)
    pub async fn update(&self, uid: u32, updates: Value) -> Result<DatabaseInfo> {
        self.client
            .put(&format!("/v1/bdbs/{}", uid), &updates)
            .await
    }

    /// Delete a database (BDB.DELETE)
    pub async fn delete(&self, uid: u32) -> Result<()> {
        self.client.delete(&format!("/v1/bdbs/{}", uid)).await
    }

    /// Get database stats (BDB.STATS)
    pub async fn stats(&self, uid: u32) -> Result<Value> {
        self.client.get(&format!("/v1/bdbs/{}/stats", uid)).await
    }

    /// Get database metrics (BDB.METRICS)
    pub async fn metrics(&self, uid: u32) -> Result<Value> {
        self.client.get(&format!("/v1/bdbs/{}/metrics", uid)).await
    }

    /// Start database (BDB.START)
    pub async fn start(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/start", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Stop database (BDB.STOP)
    pub async fn stop(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/stop", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Restart database (BDB.RESTART) - typed version
    pub async fn restart(&self, uid: u32) -> Result<DatabaseActionResponse> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/restart", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Restart database (BDB.RESTART) - raw version
    pub async fn restart_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/restart", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Export database (BDB.EXPORT) - typed version
    pub async fn export(&self, uid: u32, export_location: &str) -> Result<ExportResponse> {
        let body = serde_json::json!({
            "export_location": export_location
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/export", uid), &body)
            .await
    }

    /// Export database (BDB.EXPORT) - raw version
    pub async fn export_raw(&self, uid: u32, export_location: &str) -> Result<Value> {
        let body = serde_json::json!({
            "export_location": export_location
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/export", uid), &body)
            .await
    }

    /// Import database (BDB.IMPORT) - typed version
    pub async fn import(
        &self,
        uid: u32,
        import_location: &str,
        flush: bool,
    ) -> Result<ImportResponse> {
        let body = serde_json::json!({
            "import_location": import_location,
            "flush": flush
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/import", uid), &body)
            .await
    }

    /// Import database (BDB.IMPORT) - raw version
    pub async fn import_raw(&self, uid: u32, import_location: &str, flush: bool) -> Result<Value> {
        let body = serde_json::json!({
            "import_location": import_location,
            "flush": flush
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/import", uid), &body)
            .await
    }

    /// Flush database (BDB.FLUSH) - typed version
    pub async fn flush(&self, uid: u32) -> Result<DatabaseActionResponse> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/flush", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Flush database (BDB.FLUSH) - raw version
    pub async fn flush_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/flush", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Backup database (BDB.BACKUP) - typed version
    pub async fn backup(&self, uid: u32) -> Result<BackupResponse> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/backup", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Backup database (BDB.BACKUP) - raw version
    pub async fn backup_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/backup", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Restore database from backup (BDB.RESTORE) - typed version
    pub async fn restore(
        &self,
        uid: u32,
        backup_uid: Option<&str>,
    ) -> Result<DatabaseActionResponse> {
        let body = if let Some(backup_id) = backup_uid {
            serde_json::json!({ "backup_uid": backup_id })
        } else {
            serde_json::json!({})
        };
        self.client
            .post(&format!("/v1/bdbs/{}/actions/restore", uid), &body)
            .await
    }

    /// Restore database from backup (BDB.RESTORE) - raw version
    pub async fn restore_raw(&self, uid: u32, backup_uid: Option<&str>) -> Result<Value> {
        let body = if let Some(backup_id) = backup_uid {
            serde_json::json!({ "backup_uid": backup_id })
        } else {
            serde_json::json!({})
        };
        self.client
            .post(&format!("/v1/bdbs/{}/actions/restore", uid), &body)
            .await
    }

    /// Get database shards (BDB.SHARDS)
    pub async fn shards(&self, uid: u32) -> Result<Value> {
        self.client.get(&format!("/v1/bdbs/{}/shards", uid)).await
    }

    /// Get database endpoints (BDB.ENDPOINTS)
    pub async fn endpoints(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/endpoints", uid))
            .await
    }

    /// Optimize shards placement (status) - GET
    pub async fn optimize_shards_placement(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/v1/bdbs/{}/actions/optimize_shards_placement",
                uid
            ))
            .await
    }

    /// Recover database (status) - GET
    pub async fn recover_status(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/actions/recover", uid))
            .await
    }

    /// Recover database - POST (typed)
    pub async fn recover(&self, uid: u32) -> Result<DatabaseActionResponse> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/recover", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Recover database - POST (raw)
    pub async fn recover_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/recover", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Resume traffic - POST (typed)
    pub async fn resume_traffic(&self, uid: u32) -> Result<DatabaseActionResponse> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/resume_traffic", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Resume traffic - POST (raw)
    pub async fn resume_traffic_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/resume_traffic", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Stop traffic - POST (typed)
    pub async fn stop_traffic(&self, uid: u32) -> Result<DatabaseActionResponse> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/stop_traffic", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Stop traffic - POST (raw)
    pub async fn stop_traffic_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/stop_traffic", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Rebalance database - PUT (typed)
    pub async fn rebalance(&self, uid: u32) -> Result<DatabaseActionResponse> {
        self.client
            .put(
                &format!("/v1/bdbs/{}/actions/rebalance", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Rebalance database - PUT (raw)
    pub async fn rebalance_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .put(
                &format!("/v1/bdbs/{}/actions/rebalance", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Revamp database - PUT (typed)
    pub async fn revamp(&self, uid: u32) -> Result<DatabaseActionResponse> {
        self.client
            .put(
                &format!("/v1/bdbs/{}/actions/revamp", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Revamp database - PUT (raw)
    pub async fn revamp_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .put(
                &format!("/v1/bdbs/{}/actions/revamp", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Reset backup status - PUT
    pub async fn backup_reset_status(&self, uid: u32) -> Result<Value> {
        self.client
            .put(
                &format!("/v1/bdbs/{}/actions/backup_reset_status", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Reset export status - PUT
    pub async fn export_reset_status(&self, uid: u32) -> Result<Value> {
        self.client
            .put(
                &format!("/v1/bdbs/{}/actions/export_reset_status", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Reset import status - PUT
    pub async fn import_reset_status(&self, uid: u32) -> Result<Value> {
        self.client
            .put(
                &format!("/v1/bdbs/{}/actions/import_reset_status", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Peer stats for a database - GET
    pub async fn peer_stats(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/peer_stats", uid))
            .await
    }

    /// Peer stats for a specific peer - GET
    pub async fn peer_stats_for(&self, uid: u32, peer_uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/peer_stats/{}", uid, peer_uid))
            .await
    }

    /// Sync source stats for a database - GET
    pub async fn sync_source_stats(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/sync_source_stats", uid))
            .await
    }

    /// Sync source stats for a specific source - GET
    pub async fn sync_source_stats_for(&self, uid: u32, src_uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/sync_source_stats/{}", uid, src_uid))
            .await
    }

    /// Syncer state (all) - GET
    pub async fn syncer_state(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/syncer_state", uid))
            .await
    }

    /// Syncer state for CRDT - GET
    pub async fn syncer_state_crdt(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/syncer_state/crdt", uid))
            .await
    }

    /// Syncer state for replica - GET
    pub async fn syncer_state_replica(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/syncer_state/replica", uid))
            .await
    }

    /// Generic database command passthrough - POST
    pub async fn command_raw(&self, uid: u32, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/{}/command", uid), &body)
            .await
    }

    /// Database passwords create - POST (raw)
    pub async fn passwords_create_raw(&self, uid: u32, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/{}/passwords", uid), &body)
            .await
    }

    /// Database passwords update - PUT (raw)
    pub async fn passwords_update_raw(&self, uid: u32, body: Value) -> Result<Value> {
        self.client
            .put(&format!("/v1/bdbs/{}/passwords", uid), &body)
            .await
    }

    /// Database passwords delete - DELETE
    pub async fn passwords_delete(&self, uid: u32) -> Result<()> {
        self.client
            .delete(&format!("/v1/bdbs/{}/passwords", uid))
            .await
    }

    /// Post database alert operation - POST (raw)
    pub async fn alerts_post_raw(&self, uid: u32, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/alerts/{}", uid), &body)
            .await
    }

    /// List all database alerts - GET
    pub async fn alerts_all(&self) -> Result<Value> {
        self.client.get("/v1/bdbs/alerts").await
    }

    /// List alerts for a specific database - GET
    pub async fn alerts_for(&self, uid: u32) -> Result<Value> {
        self.client.get(&format!("/v1/bdbs/alerts/{}", uid)).await
    }

    /// Get a specific alert for a database - GET
    pub async fn alert_detail(&self, uid: u32, alert: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/alerts/{}/{}", uid, alert))
            .await
    }

    /// CRDT source alerts - GET
    pub async fn crdt_source_alerts_all(&self) -> Result<Value> {
        self.client.get("/v1/bdbs/crdt_sources/alerts").await
    }

    /// CRDT source alerts for DB - GET
    pub async fn crdt_source_alerts_for(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/crdt_sources/alerts/{}", uid))
            .await
    }

    /// CRDT source alerts for specific source - GET
    pub async fn crdt_source_alerts_source(&self, uid: u32, source_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/v1/bdbs/crdt_sources/alerts/{}/{}",
                uid, source_id
            ))
            .await
    }

    /// CRDT source alert detail - GET
    pub async fn crdt_source_alert_detail(
        &self,
        uid: u32,
        source_id: u32,
        alert: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/v1/bdbs/crdt_sources/alerts/{}/{}/{}",
                uid, source_id, alert
            ))
            .await
    }

    /// Replica source alerts - GET
    pub async fn replica_source_alerts_all(&self) -> Result<Value> {
        self.client.get("/v1/bdbs/replica_sources/alerts").await
    }

    /// Replica source alerts for DB - GET
    pub async fn replica_source_alerts_for(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/replica_sources/alerts/{}", uid))
            .await
    }

    /// Replica source alerts for specific source - GET
    pub async fn replica_source_alerts_source(&self, uid: u32, source_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/v1/bdbs/replica_sources/alerts/{}/{}",
                uid, source_id
            ))
            .await
    }

    /// Replica source alert detail - GET
    pub async fn replica_source_alert_detail(
        &self,
        uid: u32,
        source_id: u32,
        alert: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/v1/bdbs/replica_sources/alerts/{}/{}/{}",
                uid, source_id, alert
            ))
            .await
    }

    /// Modules config for a database - POST (raw)
    pub async fn modules_config_raw(&self, uid: u32, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/{}/modules/config", uid), &body)
            .await
    }

    /// Modules upgrade for a database - POST (raw)
    pub async fn modules_upgrade_raw(&self, uid: u32, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/{}/modules/upgrade", uid), &body)
            .await
    }

    /// Engine upgrade shortcut (non-actions) - POST (raw)
    pub async fn upgrade_direct_raw(&self, uid: u32, body: Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/{}/upgrade", uid), &body)
            .await
    }

    /// Update a single field via path - PUT (raw)
    pub async fn update_field_raw(&self, uid: u32, field: &str, body: Value) -> Result<Value> {
        self.client
            .put(&format!("/v1/bdbs/{}/{}", uid, field), &body)
            .await
    }

    /// Upgrade database with new module version (BDB.UPGRADE) - typed version
    pub async fn upgrade(
        &self,
        uid: u32,
        module_name: &str,
        new_version: &str,
    ) -> Result<DatabaseActionResponse> {
        let body = serde_json::json!({
            "module_name": module_name,
            "new_version": new_version
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/upgrade", uid), &body)
            .await
    }

    /// Upgrade database with new module version (BDB.UPGRADE) - raw version
    pub async fn upgrade_raw(
        &self,
        uid: u32,
        module_name: &str,
        new_version: &str,
    ) -> Result<Value> {
        let body = serde_json::json!({
            "module_name": module_name,
            "new_version": new_version
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/upgrade", uid), &body)
            .await
    }

    /// Reset database password (BDB.RESET_PASSWORD) - typed version
    pub async fn reset_password(
        &self,
        uid: u32,
        new_password: &str,
    ) -> Result<DatabaseActionResponse> {
        let body = serde_json::json!({
            "authentication_redis_pass": new_password
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/reset_password", uid), &body)
            .await
    }

    /// Reset database password (BDB.RESET_PASSWORD) - raw version
    pub async fn reset_password_raw(&self, uid: u32, new_password: &str) -> Result<Value> {
        let body = serde_json::json!({
            "authentication_redis_pass": new_password
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/reset_password", uid), &body)
            .await
    }

    /// Check database availability
    pub async fn availability(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/availability", uid))
            .await
    }

    /// Check local database endpoint availability
    pub async fn endpoint_availability(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/local/bdbs/{}/endpoint/availability", uid))
            .await
    }

    /// Create database using v2 API (supports recovery plan)
    pub async fn create_v2(&self, request: Value) -> Result<DatabaseInfo> {
        self.client.post("/v2/bdbs", &request).await
    }

    /// Create database using v2 API - raw version
    pub async fn create_v2_raw(&self, request: Value) -> Result<Value> {
        self.client.post("/v2/bdbs", &request).await
    }
}
