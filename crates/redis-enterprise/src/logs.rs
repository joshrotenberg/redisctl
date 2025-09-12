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
    /// Timestamp when event happened (ISO 8601 format)
    pub time: String,

    /// Event type - determines what additional fields are available
    /// (e.g., "bdb_name_updated", "node_status_changed", etc.)
    #[serde(rename = "type")]
    pub event_type: String,

    /// Additional fields based on event type
    /// May include severity, bdb_uid, old_val, new_val, and other event-specific fields
    #[serde(flatten)]
    pub extra: Value,
}

/// Logs query parameters
#[derive(Debug, Serialize, Default)]
pub struct LogsQuery {
    /// Optional start time before which we don't want events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stime: Option<String>,
    /// Optional end time after which we don't want events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etime: Option<String>,
    /// Order of events: "desc" (descending) or "asc" (ascending, default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// Optional maximum number of events to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Optional offset - skip this many events before returning results (for pagination)
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
