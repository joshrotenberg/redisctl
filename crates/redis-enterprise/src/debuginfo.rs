//! Debug information collection for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Debug info collection request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DebugInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub node_uids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bdb_uids: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_logs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub include_configs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub time_range: Option<TimeRange>,
}

/// Time range for debug info collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: String,
    pub end: String,
}

/// Debug info status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInfoStatus {
    pub task_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
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
            .get(&format!("/v1/debuginfo/{}/download", task_id))
            .await
    }

    /// Cancel debug info collection
    pub async fn cancel(&self, task_id: &str) -> Result<()> {
        self.client
            .delete(&format!("/v1/debuginfo/{}", task_id))
            .await
    }
}
