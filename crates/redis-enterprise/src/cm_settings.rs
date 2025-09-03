//! Cluster Manager configuration
//!
//! ## Overview
//! - Configure CM settings
//! - Manage cluster-wide parameters
//! - Query configuration status

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Cluster Manager settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cm_port: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cm_session_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_recovery: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_failover: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_ha: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_ha_grace_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_simultaneous_backups: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Cluster Manager settings handler
pub struct CmSettingsHandler {
    client: RestClient,
}

impl CmSettingsHandler {
    pub fn new(client: RestClient) -> Self {
        CmSettingsHandler { client }
    }

    /// Get Cluster Manager settings
    pub async fn get(&self) -> Result<CmSettings> {
        self.client.get("/v1/cm_settings").await
    }

    /// Update Cluster Manager settings
    pub async fn update(&self, settings: CmSettings) -> Result<CmSettings> {
        self.client.put("/v1/cm_settings", &settings).await
    }

    /// Reset Cluster Manager settings to defaults
    pub async fn reset(&self) -> Result<()> {
        self.client.delete("/v1/cm_settings").await
    }
}
