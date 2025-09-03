//! Statistics and metrics collection for Redis Enterprise
//!
//! ## Overview
//! - Query cluster, node, database, and shard statistics
//! - Retrieve time-series metrics with configurable intervals
//! - Access both current and historical performance data
//!
//! ## Return Types
//!
//! Stats methods return either typed responses (`StatsResponse`, `LastStatsResponse`)
//! or raw `serde_json::Value` for endpoints with dynamic metric names as keys.
//! The Value returns allow access to all metrics without compile-time knowledge
//! of metric names.
//!
//! ## Examples
//!
//! ### Querying Database Stats
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, StatsHandler};
//! use redis_enterprise::stats::StatsQuery;
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! let stats = StatsHandler::new(client);
//!
//! // Get last interval stats for a database
//! let last_stats = stats.database_last(1).await?;
//! println!("Database stats: {:?}", last_stats);
//!
//! // Query with specific interval (all metrics by default)
//! let query = StatsQuery {
//!     interval: Some("5min".to_string()),
//!     stime: None,
//!     etime: None,
//!     metrics: None,  // None means all metrics
//! };
//! let historical = stats.database(1, Some(query)).await?;
//! println!("5-minute intervals: {:?}", historical.intervals);
//! # Ok(())
//! # }
//! ```
//!
//! ### Cluster-Wide Statistics
//! ```no_run
//! # use redis_enterprise::{EnterpriseClient, StatsHandler};
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! let stats = StatsHandler::new(client);
//!
//! // Get aggregated stats for all nodes
//! let all_nodes = stats.nodes_last().await?;
//! println!("Total stats across cluster: {:?}", all_nodes.stats);
//!
//! // Get aggregated database stats
//! let all_dbs = stats.databases_last().await?;
//! for resource_stats in &all_dbs.stats {
//!     println!("Resource {}: {:?}", resource_stats.uid, resource_stats.intervals);
//! }
//! # Ok(())
//! # }
//! ```

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

    /// Get cluster stats for last interval
    pub async fn cluster_last(&self) -> Result<LastStatsResponse> {
        self.client.get("/v1/cluster/stats/last").await
    }

    // raw variant removed: use cluster_last()

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

    /// Get node stats for last interval
    pub async fn node_last(&self, uid: u32) -> Result<LastStatsResponse> {
        self.client
            .get(&format!("/v1/nodes/{}/stats/last", uid))
            .await
    }

    // raw variant removed: use node_last()

    /// Get all nodes stats
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

    // raw variant removed: use nodes()

    /// Get all nodes last stats
    pub async fn nodes_last(&self) -> Result<AggregatedStatsResponse> {
        self.client.get("/v1/nodes/stats/last").await
    }

    // raw variant removed: use nodes_last()

    /// Get node stats via alternate path form
    pub async fn node_alt(&self, uid: u32) -> Result<StatsResponse> {
        self.client.get(&format!("/v1/nodes/stats/{}", uid)).await
    }

    /// Get node last stats via alternate path form
    pub async fn node_last_alt(&self, uid: u32) -> Result<LastStatsResponse> {
        self.client
            .get(&format!("/v1/nodes/stats/last/{}", uid))
            .await
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

    /// Get database stats for last interval
    pub async fn database_last(&self, uid: u32) -> Result<LastStatsResponse> {
        self.client
            .get(&format!("/v1/bdbs/{}/stats/last", uid))
            .await
    }

    // raw variant removed: use database_last()

    /// Get all databases stats
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

    // raw variant removed: use databases()

    /// Get all databases last stats (aggregate)
    pub async fn databases_last(&self) -> Result<AggregatedStatsResponse> {
        self.client.get("/v1/bdbs/stats/last").await
    }

    // raw variant removed: use databases_last()

    /// Get database stats via alternate path form
    pub async fn database_alt(&self, uid: u32) -> Result<StatsResponse> {
        self.client.get(&format!("/v1/bdbs/stats/{}", uid)).await
    }

    /// Get database last stats via alternate path form
    pub async fn database_last_alt(&self, uid: u32) -> Result<LastStatsResponse> {
        self.client
            .get(&format!("/v1/bdbs/stats/last/{}", uid))
            .await
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

    /// Get all shards stats
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

    // raw variant removed: use shards()
}
