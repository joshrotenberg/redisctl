//! Alert configuration and management
//!
//! ## Overview
//! - Configure alert settings
//! - Query alert history
//! - Manage alert thresholds

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Alert information
/// Represents an alert state for a cluster object (database, node, or cluster)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Unique identifier for the alert
    pub uid: String,
    /// Name/type of the alert
    pub name: String,
    /// Alert severity level: DEBUG, INFO, WARNING, ERROR, CRITICAL
    pub severity: String,
    /// Current alert state - true if alert is currently triggered
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Type of entity this alert is associated with (e.g., "bdb", "node", "cluster")
    pub entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the entity this alert is associated with
    pub entity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UID of the entity this alert is associated with
    pub entity_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// String representing an alert threshold when applicable
    pub threshold: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ISO 8601 timestamp when alert state last changed
    pub change_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Object containing data relevant to the evaluation time when the alert went on/off (thresholds/sampled values/etc.)
    pub change_value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Human-readable description of the alert
    pub description: Option<String>,
    /// Error code associated with the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Generic alert settings (legacy - kept for compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSettings {
    /// True if alert is enabled
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Alert threshold value when applicable
    pub threshold: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of email addresses to notify when alert triggers
    pub email_recipients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Webhook URL to call when alert triggers
    pub webhook_url: Option<String>,
}

/// Database alert settings with threshold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BdbAlertSettingsWithThreshold {
    /// True if alert is enabled
    pub enabled: bool,
    /// String representing the alert threshold value
    pub threshold: String,
}

/// Complete database alerts settings object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbAlertsSettings {
    /// Periodic backup has been delayed for longer than specified threshold value \[minutes\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_backup_delayed: Option<BdbAlertSettingsWithThreshold>,

    /// CRDB source - sync lag is higher than specified threshold value \[seconds\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_crdt_src_high_syncer_lag: Option<BdbAlertSettingsWithThreshold>,

    /// CRDB source - sync has connection error while trying to connect replica source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_crdt_src_syncer_connection_error: Option<BdbAlertSettingsWithThreshold>,

    /// CRDB - sync encountered in general error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_crdt_src_syncer_general_error: Option<BdbAlertSettingsWithThreshold>,

    /// Latency is higher than specified threshold value \[micro-sec\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_high_latency: Option<BdbAlertSettingsWithThreshold>,

    /// (Deprecated) Replica of - sync lag is higher than specified threshold value \[seconds\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_high_syncer_lag: Option<BdbAlertSettingsWithThreshold>,

    /// Throughput is higher than specified threshold value \[requests / sec.\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_high_throughput: Option<BdbAlertSettingsWithThreshold>,

    /// An alert for state-machines that are running for too long
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_long_running_action: Option<BdbAlertSettingsWithThreshold>,

    /// Throughput is lower than specified threshold value \[requests / sec.\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_low_throughput: Option<BdbAlertSettingsWithThreshold>,

    /// Dataset RAM overhead of a shard has reached the threshold value \[% of its RAM limit\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_ram_dataset_overhead: Option<BdbAlertSettingsWithThreshold>,

    /// Percent of values kept in a shard's RAM is lower than \[% of its key count\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_ram_values: Option<BdbAlertSettingsWithThreshold>,

    /// Replica-of source - sync lag is higher than specified threshold value \[seconds\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_replica_src_high_syncer_lag: Option<BdbAlertSettingsWithThreshold>,

    /// Replica-of source - sync has connection error while trying to connect replica source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_replica_src_syncer_connection_error: Option<BdbAlertSettingsWithThreshold>,

    /// Replica-of - sync encountered in general error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_replica_src_syncer_general_error: Option<BdbAlertSettingsWithThreshold>,

    /// Number of values kept in a shard's RAM is lower than \[values\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_shard_num_ram_values: Option<BdbAlertSettingsWithThreshold>,

    /// Dataset size has reached the threshold value \[% of the memory limit\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_size: Option<BdbAlertSettingsWithThreshold>,

    /// (Deprecated) Replica of - sync has connection error while trying to connect replica source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_syncer_connection_error: Option<BdbAlertSettingsWithThreshold>,

    /// (Deprecated) Replica of - sync encountered in general error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_syncer_general_error: Option<BdbAlertSettingsWithThreshold>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Cluster alert settings with threshold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAlertSettingsWithThreshold {
    /// True if alert is enabled
    pub enabled: bool,
    /// String representing the alert threshold value
    pub threshold: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of email addresses to notify when alert triggers
    pub email: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Webhook URL to call when alert triggers
    pub webhook_url: Option<String>,
}

/// Complete cluster alerts settings object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAlertsSettings {
    /// CA certificate about to expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ca_cert_about_to_expire: Option<ClusterAlertSettingsWithThreshold>,

    /// Cluster certificates about to expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_certs_about_to_expire: Option<ClusterAlertSettingsWithThreshold>,

    /// License about to expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_license_about_to_expire: Option<ClusterAlertSettingsWithThreshold>,

    /// Node CPU utilization above threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cpu_utilization: Option<ClusterAlertSettingsWithThreshold>,

    /// Node ephemeral storage below threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ephemeral_storage: Option<ClusterAlertSettingsWithThreshold>,

    /// Node free flash below threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_free_flash: Option<ClusterAlertSettingsWithThreshold>,

    /// Node internal certificates about to expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_internal_certs_about_to_expire: Option<ClusterAlertSettingsWithThreshold>,

    /// Node memory below threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_memory: Option<ClusterAlertSettingsWithThreshold>,

    /// Node network throughput above threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_net_throughput: Option<ClusterAlertSettingsWithThreshold>,

    /// Node persistent storage below threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_persistent_storage: Option<ClusterAlertSettingsWithThreshold>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Alert handler for managing alerts
pub struct AlertHandler {
    client: RestClient,
}

/// Alias for backwards compatibility and intuitive plural naming
pub type AlertsHandler = AlertHandler;

impl AlertHandler {
    pub fn new(client: RestClient) -> Self {
        AlertHandler { client }
    }

    /// List all alerts
    pub async fn list(&self) -> Result<Vec<Alert>> {
        self.client.get("/v1/alerts").await
    }

    /// Get specific alert
    pub async fn get(&self, uid: &str) -> Result<Alert> {
        self.client.get(&format!("/v1/alerts/{}", uid)).await
    }

    /// List alerts for a specific database
    pub async fn list_by_database(&self, bdb_uid: u32) -> Result<Vec<Alert>> {
        self.client
            .get(&format!("/v1/bdbs/{}/alerts", bdb_uid))
            .await
    }

    /// List alerts for a specific node
    pub async fn list_by_node(&self, node_uid: u32) -> Result<Vec<Alert>> {
        self.client
            .get(&format!("/v1/nodes/{}/alerts", node_uid))
            .await
    }

    /// List alerts for the cluster
    pub async fn list_cluster_alerts(&self) -> Result<Vec<Alert>> {
        self.client.get("/v1/cluster/alerts").await
    }

    /// Get alert settings for a specific alert type
    pub async fn get_settings(&self, alert_name: &str) -> Result<AlertSettings> {
        self.client
            .get(&format!("/v1/cluster/alert_settings/{}", alert_name))
            .await
    }

    /// Update alert settings (generic/legacy)
    pub async fn update_settings(
        &self,
        alert_name: &str,
        settings: AlertSettings,
    ) -> Result<AlertSettings> {
        self.client
            .put(
                &format!("/v1/cluster/alert_settings/{}", alert_name),
                &settings,
            )
            .await
    }

    /// Get database alert settings
    pub async fn get_database_alert_settings(&self, bdb_uid: u32) -> Result<DbAlertsSettings> {
        self.client
            .get(&format!("/v1/bdbs/{}/alert_settings", bdb_uid))
            .await
    }

    /// Update database alert settings
    pub async fn update_database_alert_settings(
        &self,
        bdb_uid: u32,
        settings: &DbAlertsSettings,
    ) -> Result<DbAlertsSettings> {
        self.client
            .put(&format!("/v1/bdbs/{}/alert_settings", bdb_uid), settings)
            .await
    }

    /// Get cluster alert settings
    pub async fn get_cluster_alert_settings(&self) -> Result<ClusterAlertsSettings> {
        self.client.get("/v1/cluster/alert_settings").await
    }

    /// Update cluster alert settings
    pub async fn update_cluster_alert_settings(
        &self,
        settings: &ClusterAlertsSettings,
    ) -> Result<ClusterAlertsSettings> {
        self.client
            .put("/v1/cluster/alert_settings", settings)
            .await
    }

    /// Clear/acknowledge an alert
    pub async fn clear(&self, uid: &str) -> Result<()> {
        self.client.delete(&format!("/v1/alerts/{}", uid)).await
    }

    /// Clear all alerts
    pub async fn clear_all(&self) -> Result<()> {
        self.client.delete("/v1/alerts").await
    }
}
