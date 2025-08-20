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
}
