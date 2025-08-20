//! Alert management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub uid: String,
    pub name: String,
    pub severity: String,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Alert settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSettings {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_recipients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

/// Alert handler for managing alerts
pub struct AlertHandler {
    client: RestClient,
}

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

    /// Update alert settings
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

    /// Clear/acknowledge an alert
    pub async fn clear(&self, uid: &str) -> Result<()> {
        self.client.delete(&format!("/v1/alerts/{}", uid)).await
    }

    /// Clear all alerts
    pub async fn clear_all(&self) -> Result<()> {
        self.client.delete("/v1/alerts").await
    }
}
