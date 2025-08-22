//! Usage reporting for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Usage report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageReport {
    pub report_id: String,
    pub timestamp: String,
    pub period_start: String,
    pub period_end: String,
    pub cluster_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<DatabaseUsage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<NodeUsage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<UsageSummary>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Database usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseUsage {
    pub bdb_uid: u32,
    pub name: String,
    pub memory_used_avg: u64,
    pub memory_used_peak: u64,
    pub ops_per_sec_avg: f64,
    pub bandwidth_avg: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Node usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeUsage {
    pub node_uid: u32,
    pub cpu_usage_avg: f32,
    pub memory_usage_avg: u64,
    pub persistent_storage_usage: u64,
    pub ephemeral_storage_usage: u64,

    #[serde(flatten)]
    pub extra: Value,
}

/// Usage summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummary {
    pub total_memory_gb: f64,
    pub total_ops: u64,
    pub total_bandwidth_gb: f64,
    pub database_count: u32,
    pub node_count: u32,
    pub shard_count: u32,
}

/// Usage report configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageReportConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_recipients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_databases: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_nodes: Option<bool>,
}

/// Usage report handler
pub struct UsageReportHandler {
    client: RestClient,
}

impl UsageReportHandler {
    pub fn new(client: RestClient) -> Self {
        UsageReportHandler { client }
    }

    /// Get latest usage report
    pub async fn latest(&self) -> Result<UsageReport> {
        self.client.get("/v1/usage_report/latest").await
    }

    /// List all usage reports
    pub async fn list(&self) -> Result<Vec<UsageReport>> {
        self.client.get("/v1/usage_report").await
    }

    /// Get specific usage report
    pub async fn get(&self, report_id: &str) -> Result<UsageReport> {
        self.client
            .get(&format!("/v1/usage_report/{}", report_id))
            .await
    }

    /// Generate new usage report
    pub async fn generate(&self) -> Result<UsageReport> {
        self.client
            .post("/v1/usage_report/generate", &Value::Null)
            .await
    }

    /// Get usage report configuration
    pub async fn get_config(&self) -> Result<UsageReportConfig> {
        self.client.get("/v1/usage_report/config").await
    }

    /// Update usage report configuration
    pub async fn update_config(&self, config: UsageReportConfig) -> Result<UsageReportConfig> {
        self.client.put("/v1/usage_report/config", &config).await
    }

    /// Download usage report as CSV
    pub async fn download_csv(&self, report_id: &str) -> Result<String> {
        self.client
            .get_text(&format!("/v1/usage_report/{}/csv", report_id))
            .await
    }
}
