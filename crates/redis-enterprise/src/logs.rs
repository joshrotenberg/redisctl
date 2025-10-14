//! Log management and retrieval
//!
//! ## Overview
//! - Query cluster logs
//! - Configure log levels
//! - Export log data
//! - Stream logs in real-time (via polling)

use crate::client::RestClient;
use crate::error::Result;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;

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

    /// Stream logs in real-time by polling
    ///
    /// Since Redis Enterprise API doesn't support native streaming, this polls
    /// the logs endpoint at regular intervals and yields new log entries.
    ///
    /// # Arguments
    /// * `poll_interval` - Time to wait between polls (default: 2 seconds)
    /// * `limit` - Maximum number of logs to fetch per poll (default: 100)
    ///
    /// # Returns
    /// A stream of log entries that can be consumed with `while let Some(entry) = stream.next().await`
    pub fn stream_logs(
        &self,
        poll_interval: Duration,
        limit: Option<u32>,
    ) -> Pin<Box<dyn Stream<Item = Result<LogEntry>> + Send + '_>> {
        Box::pin(async_stream::stream! {
            let mut last_time: Option<String> = None;

            loop {
                // Build query - get logs after the last timestamp we saw
                let query = LogsQuery {
                    stime: last_time.clone(),
                    etime: None,
                    order: Some("asc".to_string()), // Ascending so we get chronological order
                    limit,
                    offset: None,
                };

                // Fetch logs
                match self.list(Some(query)).await {
                    Ok(entries) => {
                        if !entries.is_empty() {
                            // Update last_time to the timestamp of the last entry
                            if let Some(last_entry) = entries.last() {
                                last_time = Some(last_entry.time.clone());
                            }

                            // Yield each entry
                            for entry in entries {
                                yield Ok(entry);
                            }
                        }
                    }
                    Err(e) => {
                        yield Err(e);
                        break;
                    }
                }

                // Wait before next poll
                sleep(poll_interval).await;
            }
        })
    }
}
