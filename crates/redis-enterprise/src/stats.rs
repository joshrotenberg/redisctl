//! Statistics and metrics for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Stats query parameters
#[derive(Debug, Serialize)]
pub struct StatsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>, // "1min", "5min", "1hour", "1day"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stime: Option<String>, // Start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etime: Option<String>, // End time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<String>, // Comma-separated metrics
}

/// Generic stats response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub intervals: Vec<StatsInterval>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Stats interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsInterval {
    pub time: String,
    pub metrics: Value,
}

/// Last stats response for single resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastStatsResponse {
    pub time: String,
    pub metrics: Value,
    #[serde(flatten)]
    pub extra: Value,
}

/// Aggregated stats response for multiple resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedStatsResponse {
    pub stats: Vec<ResourceStats>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Stats for a single resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStats {
    pub uid: u32,
    pub intervals: Vec<StatsInterval>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Stats handler for retrieving metrics
pub struct StatsHandler {
    client: RestClient,
}

impl StatsHandler {
    pub fn new(client: RestClient) -> Self {
        StatsHandler { client }
    }

    /// Get cluster stats
    pub async fn cluster(&self, query: Option<StatsQuery>) -> Result<StatsResponse> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/cluster/stats?{}", query_str))
                .await
        } else {
            self.client.get("/v1/cluster/stats").await
        }
    }

    /// Get cluster stats for last interval - typed version
    pub async fn cluster_last(&self) -> Result<LastStatsResponse> {
        self.client.get("/v1/cluster/stats/last").await
    }

    /// Get cluster stats for last interval - raw version
    pub async fn cluster_last_raw(&self) -> Result<Value> {
        self.client.get("/v1/cluster/stats/last").await
    }

    /// Get node stats
    pub async fn node(&self, uid: u32, query: Option<StatsQuery>) -> Result<StatsResponse> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/nodes/{}/stats?{}", uid, query_str))
                .await
        } else {
            self.client.get(&format!("/v1/nodes/{}/stats", uid)).await
        }
    }

    /// Get node stats for last interval - typed version
    pub async fn node_last(&self, uid: u32) -> Result<LastStatsResponse> {
        self.client
            .get(&format!("/v1/nodes/{}/stats/last", uid))
            .await
    }

    /// Get node stats for last interval - raw version
    pub async fn node_last_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/nodes/{}/stats/last", uid))
            .await
    }

    /// Get all nodes stats - typed version
    pub async fn nodes(&self, query: Option<StatsQuery>) -> Result<AggregatedStatsResponse> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/nodes/stats?{}", query_str))
                .await
        } else {
            self.client.get("/v1/nodes/stats").await
        }
    }

    /// Get all nodes stats - raw version
    pub async fn nodes_raw(&self, query: Option<StatsQuery>) -> Result<Value> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/nodes/stats?{}", query_str))
                .await
        } else {
            self.client.get("/v1/nodes/stats").await
        }
    }

    /// Get database stats
    pub async fn database(&self, uid: u32, query: Option<StatsQuery>) -> Result<StatsResponse> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/bdbs/{}/stats?{}", uid, query_str))
                .await
        } else {
            self.client.get(&format!("/v1/bdbs/{}/stats", uid)).await
        }
    }

    /// Get database stats for last interval - typed version
    pub async fn database_last(&self, uid: u32) -> Result<LastStatsResponse> {
        self.client
            .get(&format!("/v1/bdbs/{}/stats/last", uid))
            .await
    }

    /// Get database stats for last interval - raw version
    pub async fn database_last_raw(&self, uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/bdbs/{}/stats/last", uid))
            .await
    }

    /// Get all databases stats - typed version
    pub async fn databases(&self, query: Option<StatsQuery>) -> Result<AggregatedStatsResponse> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/bdbs/stats?{}", query_str))
                .await
        } else {
            self.client.get("/v1/bdbs/stats").await
        }
    }

    /// Get all databases stats - raw version
    pub async fn databases_raw(&self, query: Option<StatsQuery>) -> Result<Value> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/bdbs/stats?{}", query_str))
                .await
        } else {
            self.client.get("/v1/bdbs/stats").await
        }
    }

    /// Get shard stats
    pub async fn shard(&self, uid: u32, query: Option<StatsQuery>) -> Result<StatsResponse> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/shards/{}/stats?{}", uid, query_str))
                .await
        } else {
            self.client.get(&format!("/v1/shards/{}/stats", uid)).await
        }
    }

    /// Get all shards stats - typed version
    pub async fn shards(&self, query: Option<StatsQuery>) -> Result<AggregatedStatsResponse> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/shards/stats?{}", query_str))
                .await
        } else {
            self.client.get("/v1/shards/stats").await
        }
    }

    /// Get all shards stats - raw version
    pub async fn shards_raw(&self, query: Option<StatsQuery>) -> Result<Value> {
        if let Some(q) = query {
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client
                .get(&format!("/v1/shards/stats?{}", query_str))
                .await
        } else {
            self.client.get("/v1/shards/stats").await
        }
    }
}
