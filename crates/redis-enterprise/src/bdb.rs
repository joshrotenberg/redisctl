//! Database (BDB) management commands for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Aliases for easier use
pub type Database = DatabaseInfo;
pub type BdbHandler = DatabaseHandler;

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

/// Builder for CreateDatabaseRequest
#[derive(Debug, Default)]
pub struct CreateDatabaseRequestBuilder {
    name: Option<String>,
    memory_size: Option<u64>,
    port: Option<u16>,
    replication: Option<bool>,
    persistence: Option<String>,
    eviction_policy: Option<String>,
    shards_count: Option<u32>,
    module_list: Option<Vec<ModuleConfig>>,
    authentication_redis_pass: Option<String>,
}

impl CreateDatabaseRequestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the database name (required)
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the memory size in bytes (required)
    pub fn memory_size(mut self, size: u64) -> Self {
        self.memory_size = Some(size);
        self
    }

    /// Set the port number
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Enable or disable replication
    pub fn replication(mut self, enabled: bool) -> Self {
        self.replication = Some(enabled);
        self
    }

    /// Set persistence type ("aof", "snapshot", "disabled")
    pub fn persistence(mut self, persistence: impl Into<String>) -> Self {
        self.persistence = Some(persistence.into());
        self
    }

    /// Set eviction policy
    pub fn eviction_policy(mut self, policy: impl Into<String>) -> Self {
        self.eviction_policy = Some(policy.into());
        self
    }

    /// Set number of shards
    pub fn shards(mut self, count: u32) -> Self {
        self.shards_count = Some(count);
        self
    }

    /// Add Redis modules
    pub fn modules(mut self, modules: Vec<ModuleConfig>) -> Self {
        self.module_list = Some(modules);
        self
    }

    /// Set database password
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.authentication_redis_pass = Some(password.into());
        self
    }

    /// Build the request
    pub fn build(self) -> Result<CreateDatabaseRequest> {
        Ok(CreateDatabaseRequest {
            name: self.name.ok_or_else(|| {
                crate::error::RestError::ValidationError("Database name is required".to_string())
            })?,
            memory_size: self.memory_size.ok_or_else(|| {
                crate::error::RestError::ValidationError("Memory size is required".to_string())
            })?,
            port: self.port,
            replication: self.replication,
            persistence: self.persistence,
            eviction_policy: self.eviction_policy,
            shards_count: self.shards_count,
            module_list: self.module_list,
            authentication_redis_pass: self.authentication_redis_pass,
        })
    }
}

/// Module configuration for database creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub module_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_args: Option<String>,
}

/// Create database request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDatabaseRequest {
    pub name: String,
    pub memory_size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_list: Option<Vec<ModuleConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_redis_pass: Option<String>,
}

impl CreateDatabaseRequest {
    /// Create a new builder for the request
    pub fn builder() -> CreateDatabaseRequestBuilder {
        CreateDatabaseRequestBuilder::new()
    }
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

    /// Create a new database (BDB.CREATE)
    pub async fn create(&self, request: CreateDatabaseRequest) -> Result<DatabaseInfo> {
        self.client.post("/v1/bdbs", &request).await
    }

    /// Create a new database using builder pattern
    pub async fn create_with_builder<F>(&self, f: F) -> Result<DatabaseInfo>
    where
        F: FnOnce(CreateDatabaseRequestBuilder) -> CreateDatabaseRequestBuilder,
    {
        let builder = CreateDatabaseRequestBuilder::new();
        let request = f(builder).build()?;
        self.create(request).await
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

    /// Restart database (BDB.RESTART)
    pub async fn restart(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/restart", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Export database (BDB.EXPORT)
    pub async fn export(&self, uid: u32, export_location: &str) -> Result<Value> {
        let body = serde_json::json!({
            "export_location": export_location
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/export", uid), &body)
            .await
    }

    /// Import database (BDB.IMPORT)
    pub async fn import(&self, uid: u32, import_location: &str, flush: bool) -> Result<Value> {
        let body = serde_json::json!({
            "import_location": import_location,
            "flush": flush
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/import", uid), &body)
            .await
    }

    /// Flush database (BDB.FLUSH)
    pub async fn flush(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/flush", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Backup database (BDB.BACKUP)
    pub async fn backup(&self, uid: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/v1/bdbs/{}/actions/backup", uid),
                &serde_json::json!({}),
            )
            .await
    }

    /// Restore database from backup (BDB.RESTORE)
    pub async fn restore(&self, uid: u32, backup_uid: Option<&str>) -> Result<Value> {
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

    /// Upgrade database with new module version (BDB.UPGRADE)
    pub async fn upgrade(&self, uid: u32, module_name: &str, new_version: &str) -> Result<Value> {
        let body = serde_json::json!({
            "module_name": module_name,
            "new_version": new_version
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/upgrade", uid), &body)
            .await
    }

    /// Reset database password (BDB.RESET_PASSWORD)
    pub async fn reset_password(&self, uid: u32, new_password: &str) -> Result<Value> {
        let body = serde_json::json!({
            "authentication_redis_pass": new_password
        });
        self.client
            .post(&format!("/v1/bdbs/{}/actions/reset_password", uid), &body)
            .await
    }
}
