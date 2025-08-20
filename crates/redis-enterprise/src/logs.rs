//! Event log management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: u64,
    pub time: String,
    pub level: String,
    pub component: Option<String>,
    pub message: String,
    pub node_uid: Option<u32>,
    pub bdb_uid: Option<u32>,
    pub user: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Logs query parameters
#[derive(Debug, Serialize)]
pub struct LogsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_uid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_uid: Option<u32>,
}

/// Logs handler for querying event logs
pub struct LogsHandler {
    client: RestClient,
}

impl LogsHandler {
    pub fn new(client: RestClient) -> Self {
        LogsHandler { client }
    }

    /// Get event logs
    pub async fn list(&self, query: Option<LogsQuery>) -> Result<Vec<LogEntry>> {
        if let Some(q) = query {
            // Build query string from LogsQuery
            let query_str = serde_urlencoded::to_string(&q).unwrap_or_default();
            self.client.get(&format!("/v1/logs?{}", query_str)).await
        } else {
            self.client.get("/v1/logs").await
        }
    }

    /// Get specific log entry
    pub async fn get(&self, id: u64) -> Result<LogEntry> {
        self.client.get(&format!("/v1/logs/{}", id)).await
    }
}
