//! Maintenance window management for Redis Cloud
//!
//! ## Overview
//! - Configure maintenance windows
//! - Set preferred maintenance times

use crate::client::CloudClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Maintenance window configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    pub mode: MaintenanceMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<Vec<Window>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Maintenance mode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MaintenanceMode {
    Automatic,
    Manual,
}

/// Maintenance window details
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Window {
    #[serde(rename = "startHour")]
    pub start_hour: u8,

    #[serde(rename = "durationInHours")]
    pub duration_in_hours: u8,

    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub days: Option<Vec<String>>,
}

/// Update maintenance window request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateMaintenanceWindowRequest {
    pub mode: MaintenanceMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub windows: Option<Vec<Window>>,
}

/// Maintenance window handler
pub struct MaintenanceWindowHandler {
    client: CloudClient,
}

impl MaintenanceWindowHandler {
    pub fn new(client: CloudClient) -> Self {
        MaintenanceWindowHandler { client }
    }

    /// Get maintenance windows for a subscription
    pub async fn get(&self, subscription_id: u32) -> Result<MaintenanceWindow> {
        self.client
            .get(&format!(
                "/subscriptions/{}/maintenance-windows",
                subscription_id
            ))
            .await
    }

    /// Update maintenance windows for a subscription
    pub async fn update(
        &self,
        subscription_id: u32,
        request: UpdateMaintenanceWindowRequest,
    ) -> Result<MaintenanceWindow> {
        self.client
            .put(
                &format!("/subscriptions/{}/maintenance-windows", subscription_id),
                &request,
            )
            .await
    }
}
