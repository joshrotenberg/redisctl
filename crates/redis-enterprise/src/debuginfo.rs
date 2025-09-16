//! Debuginfo management for Redis Enterprise
//!
//! ## Overview
//! - List and query resources
//! - Create and update configurations
//! - Monitor status and metrics

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Debug info collection request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DebugInfoRequest {
    /// List of node UIDs to collect debug info from (if not specified, collects from all nodes)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub node_uids: Option<Vec<u32>>,
    /// List of database UIDs to collect debug info for (if not specified, collects for all databases)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bdb_uids: Option<Vec<u32>>,
    /// Whether to include log files in the debug info collection
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_logs: Option<bool>,
    /// Whether to include system and database metrics in the debug info
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_metrics: Option<bool>,
    /// Whether to include configuration files and settings
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_configs: Option<bool>,
    /// Time range for collecting historical data and logs
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub time_range: Option<TimeRange>,
}

/// Time range for debug info collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start time for data collection (ISO 8601 format)
    pub start: String,
    /// End time for data collection (ISO 8601 format)
    pub end: String,
}

/// Debug info status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfoStatus {
    /// Unique identifier for the debug info collection task
    pub task_id: String,
    /// Current status of the debug info collection (queued, running, completed, failed)
    pub status: String,
    /// Completion progress as a percentage (0.0-100.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    /// URL for downloading the collected debug info package
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// Error description if the collection task failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Debug info handler
pub struct DebugInfoHandler {
    client: RestClient,
}

impl DebugInfoHandler {
    pub fn new(client: RestClient) -> Self {
        DebugInfoHandler { client }
    }

    /// Start debug info collection
    pub async fn create(&self, request: DebugInfoRequest) -> Result<DebugInfoStatus> {
        self.client.post("/v1/debuginfo", &request).await
    }

    /// Get debug info collection status
    pub async fn status(&self, task_id: &str) -> Result<DebugInfoStatus> {
        self.client.get(&format!("/v1/debuginfo/{}", task_id)).await
    }

    /// List all debug info tasks
    pub async fn list(&self) -> Result<Vec<DebugInfoStatus>> {
        self.client.get("/v1/debuginfo").await
    }

    /// Download debug info package
    pub async fn download(&self, task_id: &str) -> Result<Vec<u8>> {
        self.client
            .get_binary(&format!("/v1/debuginfo/{}/download", task_id))
            .await
    }

    /// Cancel debug info collection
    pub async fn cancel(&self, task_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/debuginfo/{}", task_id))
            .await
    }

    /// Get all debug info across nodes - GET /v1/debuginfo/all (DEPRECATED)
    /// Use cluster_debuginfo_binary() for the new endpoint
    pub async fn all(&self) -> Result<Value> {
        self.client.get("/v1/debuginfo/all").await
    }

    /// Get all debug info for a specific database - GET /v1/debuginfo/all/bdb/{uid} (DEPRECATED)
    /// Use database_debuginfo_binary() for the new endpoint
    pub async fn all_bdb(&self, bdb_uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/debuginfo/all/bdb/{}", bdb_uid))
            .await
    }

    /// Get node debug info - GET /v1/debuginfo/node (DEPRECATED)
    /// Use nodes_debuginfo_binary() for the new endpoint
    pub async fn node(&self) -> Result<Value> {
        self.client.get("/v1/debuginfo/node").await
    }

    /// Get node debug info for a specific database - GET /v1/debuginfo/node/bdb/{uid} (DEPRECATED)
    /// Use database_debuginfo_binary() for the new endpoint
    pub async fn node_bdb(&self, bdb_uid: u32) -> Result<Value> {
        self.client
            .get(&format!("/v1/debuginfo/node/bdb/{}", bdb_uid))
            .await
    }

    // New binary endpoints (current API)

    /// Get cluster debug info package as binary - GET /v1/cluster/debuginfo
    /// Returns a tar.gz file containing all cluster debug information
    pub async fn cluster_debuginfo_binary(&self) -> Result<Vec<u8>> {
        self.client.get_binary("/v1/cluster/debuginfo").await
    }

    /// Get all nodes debug info package as binary - GET /v1/nodes/debuginfo
    /// Returns a tar.gz file containing debug information from all nodes
    pub async fn nodes_debuginfo_binary(&self) -> Result<Vec<u8>> {
        self.client.get_binary("/v1/nodes/debuginfo").await
    }

    /// Get specific node debug info package as binary - GET /v1/nodes/{uid}/debuginfo
    /// Returns a tar.gz file containing debug information from a specific node
    pub async fn node_debuginfo_binary(&self, node_uid: u32) -> Result<Vec<u8>> {
        self.client
            .get_binary(&format!("/v1/nodes/{}/debuginfo", node_uid))
            .await
    }

    /// Get all databases debug info package as binary - GET /v1/bdbs/debuginfo
    /// Returns a tar.gz file containing debug information from all databases
    pub async fn databases_debuginfo_binary(&self) -> Result<Vec<u8>> {
        self.client.get_binary("/v1/bdbs/debuginfo").await
    }

    /// Get specific database debug info package as binary - GET /v1/bdbs/{uid}/debuginfo
    /// Returns a tar.gz file containing debug information from a specific database
    pub async fn database_debuginfo_binary(&self, bdb_uid: u32) -> Result<Vec<u8>> {
        self.client
            .get_binary(&format!("/v1/bdbs/{}/debuginfo", bdb_uid))
            .await
    }

    // Deprecated binary endpoints (for backward compatibility)

    /// Get all debug info as binary - GET /v1/debuginfo/all (DEPRECATED)
    /// Returns a tar.gz file - Use cluster_debuginfo_binary() instead
    pub async fn all_binary(&self) -> Result<Vec<u8>> {
        self.client.get_binary("/v1/debuginfo/all").await
    }

    /// Get all debug info for a specific database as binary - GET /v1/debuginfo/all/bdb/{uid} (DEPRECATED)
    /// Returns a tar.gz file - Use database_debuginfo_binary() instead
    pub async fn all_bdb_binary(&self, bdb_uid: u32) -> Result<Vec<u8>> {
        self.client
            .get_binary(&format!("/v1/debuginfo/all/bdb/{}", bdb_uid))
            .await
    }

    /// Get node debug info as binary - GET /v1/debuginfo/node (DEPRECATED)
    /// Returns a tar.gz file - Use nodes_debuginfo_binary() instead
    pub async fn node_binary(&self) -> Result<Vec<u8>> {
        self.client.get_binary("/v1/debuginfo/node").await
    }

    /// Get node debug info for a specific database as binary - GET /v1/debuginfo/node/bdb/{uid} (DEPRECATED)
    /// Returns a tar.gz file - Use database_debuginfo_binary() instead
    pub async fn node_bdb_binary(&self, bdb_uid: u32) -> Result<Vec<u8>> {
        self.client
            .get_binary(&format!("/v1/debuginfo/node/bdb/{}", bdb_uid))
            .await
    }
}
