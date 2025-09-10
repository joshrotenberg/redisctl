//! Log management and retrieval
//!
//! ## Overview
//! - Query cluster logs
//! - Configure log levels
//! - Export log data

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Log entry (cluster event)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp when event happened
    pub time: String,

    /// Event type - determines what additional fields are available
    #[serde(rename = "type")]
    pub event_type: String,

    /// Additional fields based on event type
    #[serde(flatten)]
    pub extra: Value,
}

/// Logs query parameters
#[derive(Debug, Serialize, Default)]
pub struct LogsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
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
}
