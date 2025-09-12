//! Proxies management for Redis Enterprise
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

    // Additional fields from API audit
    /// Backlog size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlog: Option<u32>,

    /// Client eviction enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_eviction: Option<bool>,

    /// Client TCP keepalive count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_keepcnt: Option<u32>,

    /// Client TCP keepalive idle time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_keepidle: Option<u32>,

    /// Client TCP keepalive interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_keepintvl: Option<u32>,

    /// Number of connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conns: Option<u32>,

    /// Core file generation enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corefile: Option<bool>,

    /// Duration usage threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_usage_threshold: Option<u32>,

    /// Dynamic threads scaling enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_threads_scaling: Option<bool>,

    /// Ignore BDB client connection limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_bdb_cconn_limit: Option<bool>,

    /// Ignore BDB client connection output buffer limits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_bdb_cconn_output_buff_limits: Option<bool>,

    /// Incoming connections capacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_connections_capacity: Option<u32>,

    /// Incoming connections minimum capacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_connections_min_capacity: Option<u32>,

    /// Incoming connections rate limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_connections_rate_limit: Option<u32>,

    /// Log level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,

    /// Maximum listeners
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_listeners: Option<u32>,

    /// Maximum servers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_servers: Option<u32>,

    /// Maximum threads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_threads: Option<u32>,

    /// Maximum worker client connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_worker_client_conns: Option<u32>,

    /// Maximum worker server connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_worker_server_conns: Option<u32>,

    /// Maximum worker transactions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_worker_txns: Option<u32>,

    /// Maximum memory for clients
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmemory_clients: Option<u32>,

    /// Threads usage threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads_usage_threshold: Option<u32>,

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

    /// Get proxy statistics for a specific metric
    pub async fn stats_metric(&self, uid: u32, metric: &str) -> Result<MetricResponse> {
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
