//! Database migration operations
//!
//! ## Overview
//! - Perform database migrations
//! - Track migration status
//! - Manage migration plans

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Migration task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    /// Unique identifier for this migration task
    pub migration_id: String,
    /// Source endpoint configuration
    pub source: MigrationEndpoint,
    /// Target endpoint configuration
    pub target: MigrationEndpoint,
    /// Sync status of this migration (e.g., "syncing", "in-sync", "out-of-sync")
    pub status: String,
    /// Migration progress as a percentage (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    /// Timestamp when migration started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Timestamp when migration completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Error message if migration failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Migration endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationEndpoint {
    /// Type of endpoint (e.g., "redis", "cluster", "azure-cache")
    pub endpoint_type: String,
    /// Hostname or IP address of the endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Port number of the endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Database UID (for internal cluster migrations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_uid: Option<u32>,
    /// Authentication password for the endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Whether to use SSL/TLS for the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<bool>,
}

/// Create migration request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateMigrationRequest {
    /// Source endpoint configuration
    pub source: MigrationEndpoint,
    /// Target endpoint configuration
    pub target: MigrationEndpoint,
    /// Type of migration to perform (e.g., "full", "incremental")
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub migration_type: Option<String>,
    /// Redis key pattern to migrate (supports wildcards)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub key_pattern: Option<String>,
    /// Whether to flush the target database before migration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub flush_target: Option<bool>,
}

/// Migrations handler
pub struct MigrationsHandler {
    client: RestClient,
}

impl MigrationsHandler {
    pub fn new(client: RestClient) -> Self {
        MigrationsHandler { client }
    }

    /// List all migrations
    pub async fn list(&self) -> Result<Vec<Migration>> {
        self.client.get("/v1/migrations").await
    }

    /// Get specific migration
    pub async fn get(&self, migration_id: &str) -> Result<Migration> {
        self.client
            .get(&format!("/v1/migrations/{}", migration_id))
            .await
    }

    /// Create a new migration
    pub async fn create(&self, request: CreateMigrationRequest) -> Result<Migration> {
        self.client.post("/v1/migrations", &request).await
    }

    /// Start a migration
    pub async fn start(&self, migration_id: &str) -> Result<Migration> {
        self.client
            .post(
                &format!("/v1/migrations/{}/start", migration_id),
                &Value::Null,
            )
            .await
    }

    /// Pause a migration
    pub async fn pause(&self, migration_id: &str) -> Result<Migration> {
        self.client
            .post(
                &format!("/v1/migrations/{}/pause", migration_id),
                &Value::Null,
            )
            .await
    }

    /// Resume a migration
    pub async fn resume(&self, migration_id: &str) -> Result<Migration> {
        self.client
            .post(
                &format!("/v1/migrations/{}/resume", migration_id),
                &Value::Null,
            )
            .await
    }

    /// Cancel a migration
    pub async fn cancel(&self, migration_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/migrations/{}", migration_id))
            .await
    }
}
