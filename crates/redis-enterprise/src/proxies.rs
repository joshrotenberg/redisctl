//! Proxy management for Redis Enterprise
//!
//! Overview
//! - List/get proxies
//! - Per-proxy metrics (interval/timestamps/values)
//! - Bulk and per-proxy updates via typed `ProxyUpdate`

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

/// Proxy information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    pub uid: u32,
    pub bdb_uid: u32,
    pub node_uid: u32,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Proxy stats information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStats {
    pub uid: u32,
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

/// Proxy handler for managing proxies
pub struct ProxyHandler {
    client: RestClient,
}

impl ProxyHandler {
    pub fn new(client: RestClient) -> Self {
        ProxyHandler { client }
    }

    /// List all proxies
    pub async fn list(&self) -> Result<Vec<Proxy>> {
        self.client.get("/v1/proxies").await
    }

    /// Get specific proxy information
    pub async fn get(&self, uid: u32) -> Result<Proxy> {
        self.client.get(&format!("/v1/proxies/{}", uid)).await
    }

    /// Get proxy statistics
    pub async fn stats(&self, uid: u32) -> Result<ProxyStats> {
        self.client.get(&format!("/v1/proxies/{}/stats", uid)).await
    }

    /// Get proxy statistics for a specific metric - typed version
    pub async fn stats_metric(&self, uid: u32, metric: &str) -> Result<MetricResponse> {
        self.client
            .get(&format!("/v1/proxies/{}/stats/{}", uid, metric))
            .await
    }

    /// Get proxy statistics for a specific metric - raw version
    pub async fn stats_metric_raw(&self, uid: u32, metric: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/proxies/{}/stats/{}", uid, metric))
            .await
    }

    /// Get proxies for a specific database
    pub async fn list_by_database(&self, bdb_uid: u32) -> Result<Vec<Proxy>> {
        self.client
            .get(&format!("/v1/bdbs/{}/proxies", bdb_uid))
            .await
    }

    /// Get proxies for a specific node
    pub async fn list_by_node(&self, node_uid: u32) -> Result<Vec<Proxy>> {
        self.client
            .get(&format!("/v1/nodes/{}/proxies", node_uid))
            .await
    }

    /// Reload proxy configuration
    pub async fn reload(&self, uid: u32) -> Result<()> {
        self.client
            .post_action(&format!("/v1/proxies/{}/actions/reload", uid), &Value::Null)
            .await
    }

    /// Update proxies (bulk) - PUT /v1/proxies
    pub async fn update_all(&self, update: ProxyUpdate) -> Result<Vec<Proxy>> {
        self.client.put("/v1/proxies", &update).await
    }

    /// Update specific proxy - PUT /v1/proxies/{uid}
    pub async fn update(&self, uid: u32, update: ProxyUpdate) -> Result<Proxy> {
        self.client
            .put(&format!("/v1/proxies/{}", uid), &update)
            .await
    }
}

/// Proxy update body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<u32>,
    #[serde(flatten)]
    pub extra: Value,
}
