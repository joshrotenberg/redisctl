//! Database (BDB) management commands for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Aliases for easier use
pub type Database = DatabaseInfo;
pub type BdbHandler = DatabaseHandler;

/// Database information from the REST API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub uid: u32,
    pub name: String,
    pub port: Option<u16>,
    pub status: Option<String>,
    pub memory_size: Option<u64>,
    pub memory_used: Option<u64>,
    pub type_: Option<String>,
    pub version: Option<String>,
    pub shards_count: Option<u32>,
    pub endpoints: Option<Vec<EndpointInfo>>,
    pub replication: Option<bool>,
    pub persistence: Option<String>,
    pub eviction_policy: Option<String>,

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
            .post(&format!("/v1/bdbs/{}/actions/start", uid), &serde_json::json!({}))
            .await
    }

    /// Stop database (BDB.STOP)
    pub async fn stop(&self, uid: u32) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/{}/actions/stop", uid), &serde_json::json!({}))
            .await
    }

    /// Restart database (BDB.RESTART)
    pub async fn restart(&self, uid: u32) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/{}/actions/restart", uid), &serde_json::json!({}))
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
            .post(&format!("/v1/bdbs/{}/actions/flush", uid), &serde_json::json!({}))
            .await
    }

    /// Backup database (BDB.BACKUP)
    pub async fn backup(&self, uid: u32) -> Result<Value> {
        self.client
            .post(&format!("/v1/bdbs/{}/actions/backup", uid), &serde_json::json!({}))
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
        self.client.get(&format!("/v1/bdbs/{}/endpoints", uid)).await
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
