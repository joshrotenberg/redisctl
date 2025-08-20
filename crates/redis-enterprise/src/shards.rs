//! Shard management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub async fn stats_metric(&self, uid: &str, metric: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/shards/{}/stats/{}", uid, metric))
            .await
    }

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
}
