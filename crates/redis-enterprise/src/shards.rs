//! Shards management for Redis Enterprise
//!
//! ## Overview
//! - List and query resources
//! - Create and update configurations
//! - Monitor status and metrics

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Response for a single metric query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricResponse {
    pub interval: String,
    pub timestamps: Vec<i64>,
    pub values: Vec<Value>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Shard information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shard {
    pub uid: String,
    pub bdb_uid: u32,
    pub node_uid: u32,
    pub role: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_memory: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_progress: Option<f64>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Shard stats information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardStats {
    pub uid: String,
    pub intervals: Vec<StatsInterval>,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsInterval {
    pub interval: String,
    pub timestamps: Vec<i64>,
    pub values: Vec<Value>,
}

/// Shard handler for managing shards
pub struct ShardHandler {
    client: RestClient,
}

impl ShardHandler {
    pub fn new(client: RestClient) -> Self {
        ShardHandler { client }
    }

    /// List all shards
    pub async fn list(&self) -> Result<Vec<Shard>> {
        self.client.get("/v1/shards").await
    }

    /// Get specific shard information
    pub async fn get(&self, uid: &str) -> Result<Shard> {
        self.client.get(&format!("/v1/shards/{}", uid)).await
    }

    /// Get shard statistics
    pub async fn stats(&self, uid: &str) -> Result<ShardStats> {
        self.client.get(&format!("/v1/shards/{}/stats", uid)).await
    }

    /// Get shard statistics for a specific metric
    pub async fn stats_metric(&self, uid: &str, metric: &str) -> Result<MetricResponse> {
        self.client
            .get(&format!("/v1/shards/{}/stats/{}", uid, metric))
            .await
    }

    // raw variant removed: use stats_metric()

    /// Get shards for a specific database
    pub async fn list_by_database(&self, bdb_uid: u32) -> Result<Vec<Shard>> {
        self.client
            .get(&format!("/v1/bdbs/{}/shards", bdb_uid))
            .await
    }

    /// Get shards for a specific node
    pub async fn list_by_node(&self, node_uid: u32) -> Result<Vec<Shard>> {
        self.client
            .get(&format!("/v1/nodes/{}/shards", node_uid))
            .await
    }

    // Aggregate raw helpers removed; use StatsHandler for aggregates

    /// Global failover - POST /v1/shards/actions/failover
    pub async fn failover_all(&self, body: ShardActionRequest) -> Result<Action> {
        self.client.post("/v1/shards/actions/failover", &body).await
    }

    /// Global migrate - POST /v1/shards/actions/migrate
    pub async fn migrate_all(&self, body: ShardActionRequest) -> Result<Action> {
        self.client.post("/v1/shards/actions/migrate", &body).await
    }

    /// Per-shard failover - POST /v1/shards/{uid}/actions/failover
    pub async fn failover(&self, uid: &str, body: ShardActionRequest) -> Result<Action> {
        self.client
            .post(&format!("/v1/shards/{}/actions/failover", uid), &body)
            .await
    }

    /// Per-shard migrate - POST /v1/shards/{uid}/actions/migrate
    pub async fn migrate(&self, uid: &str, body: ShardActionRequest) -> Result<Action> {
        self.client
            .post(&format!("/v1/shards/{}/actions/migrate", uid), &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardActionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_uids: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}
