//! Usage reporting and telemetry
//!
//! ## Overview
//! - Generate usage reports
//! - Configure telemetry settings
//! - Export usage data

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Usage report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageReport {
    /// Unique identifier for this usage report
    pub report_id: String,
    /// Timestamp when the report was generated
    pub timestamp: String,
    /// Start time of the reporting period
    pub period_start: String,
    /// End time of the reporting period
    pub period_end: String,
    /// Name of the cluster
    pub cluster_name: String,
    /// Usage information for individual databases
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<DatabaseUsage>>,
    /// Usage information for cluster nodes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<NodeUsage>>,
    /// Summary of overall usage across the cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<UsageSummary>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Database usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseUsage {
    /// Database unique identifier
    pub bdb_uid: u32,
    /// Name of the database
    pub name: String,
    /// Average memory usage during the reporting period (bytes)
    pub memory_used_avg: u64,
    /// Peak memory usage during the reporting period (bytes)
    pub memory_used_peak: u64,
    /// Average operations per second
    pub ops_per_sec_avg: f64,
    /// Average bandwidth usage (bytes per second)
    pub bandwidth_avg: u64,
    /// Number of shards in the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Node usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeUsage {
    /// Node unique identifier
    pub node_uid: u32,
    /// Average CPU usage as a percentage (0.0-1.0)
    pub cpu_usage_avg: f32,
    /// Average memory usage during the reporting period (bytes)
    pub memory_usage_avg: u64,
    /// Persistent storage usage (bytes)
    pub persistent_storage_usage: u64,
    /// Ephemeral storage usage (bytes)
    pub ephemeral_storage_usage: u64,

    #[serde(flatten)]
    pub extra: Value,
}

/// Usage summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSummary {
    /// Total memory usage across the cluster (GB)
    pub total_memory_gb: f64,
    /// Total number of operations across the cluster
    pub total_ops: u64,
    /// Total bandwidth usage across the cluster (GB)
    pub total_bandwidth_gb: f64,
    /// Total number of databases in the cluster
    pub database_count: u32,
    /// Total number of nodes in the cluster
    pub node_count: u32,
    /// Total number of shards in the cluster
    pub shard_count: u32,
}

/// Usage report configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageReportConfig {
    /// Whether usage reporting is enabled
    pub enabled: bool,
    /// Email addresses to send usage reports to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_recipients: Option<Vec<String>>,
    /// Frequency of report generation (e.g., "daily", "weekly", "monthly")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// Whether to include database usage information in reports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_databases: Option<bool>,
    /// Whether to include node usage information in reports
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
