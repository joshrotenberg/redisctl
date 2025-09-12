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
    /// Maximum number of pending connections in the listen queue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlog: Option<u32>,

    /// Whether automatic client eviction is enabled when limits are reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_eviction: Option<bool>,

    /// Number of TCP keepalive probes before connection is dropped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_keepcnt: Option<u32>,

    /// Time in seconds before TCP keepalive probes start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_keepidle: Option<u32>,

    /// Interval in seconds between TCP keepalive probes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_keepintvl: Option<u32>,

    /// Current number of active connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conns: Option<u32>,

    /// Whether core dump files are generated on crash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corefile: Option<bool>,

    /// Threshold in milliseconds for slow operation logging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_usage_threshold: Option<u32>,

    /// Whether proxy can dynamically adjust thread count based on load
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_threads_scaling: Option<bool>,

    /// Whether to bypass database connection limit checks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_bdb_cconn_limit: Option<bool>,

    /// Whether to bypass database output buffer limit checks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_bdb_cconn_output_buff_limits: Option<bool>,

    /// Maximum capacity for incoming connection handling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_connections_capacity: Option<u32>,

    /// Minimum reserved capacity for incoming connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_connections_min_capacity: Option<u32>,

    /// Maximum rate of new incoming connections per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoming_connections_rate_limit: Option<u32>,

    /// Logging level for proxy (e.g., 'debug', 'info', 'warning', 'error')
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,

    /// Maximum number of listener sockets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_listeners: Option<u32>,

    /// Maximum number of backend server connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_servers: Option<u32>,

    /// Maximum number of worker threads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_threads: Option<u32>,

    /// Maximum client connections per worker thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_worker_client_conns: Option<u32>,

    /// Maximum server connections per worker thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_worker_server_conns: Option<u32>,

    /// Maximum concurrent transactions per worker thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_worker_txns: Option<u32>,

    /// Maximum memory in bytes allocated for client connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxmemory_clients: Option<u32>,

    /// CPU usage threshold percentage for thread scaling decisions
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
