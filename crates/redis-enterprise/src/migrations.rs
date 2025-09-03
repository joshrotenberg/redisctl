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
    pub migration_id: String,
    pub source: MigrationEndpoint,
    pub target: MigrationEndpoint,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Migration endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationEndpoint {
    pub endpoint_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_uid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<bool>,
}

/// Create migration request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateMigrationRequest {
    pub source: MigrationEndpoint,
    pub target: MigrationEndpoint,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub migration_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub key_pattern: Option<String>,
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
