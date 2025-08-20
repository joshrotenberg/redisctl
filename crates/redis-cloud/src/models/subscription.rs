//! Subscription-related data models

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Cloud subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudSubscription {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub payment_method_id: Option<u32>,
    pub memory_storage: Option<String>,
    pub persistent_storage_encryption: Option<bool>,
    pub cloud_provider: Option<CloudProvider>,
    pub region: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudProvider {
    pub provider: String,
    pub cloud_account_id: Option<u32>,
    pub regions: Option<Vec<CloudRegion>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudRegion {
    pub region: String,
    pub networking_deployment_cidr: Option<String>,
    pub preferred_availability_zones: Option<Vec<String>>,
    pub multiple_availability_zones: Option<bool>,
}

/// Create subscription request
#[derive(Debug, Serialize)]
pub struct CreateSubscriptionRequest {
    pub name: String,
    pub payment_method_id: u32,
    pub memory_storage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_storage_encryption: Option<bool>,
    pub cloud_provider: CloudProviderConfig,
}

#[derive(Debug, Serialize)]
pub struct CloudProviderConfig {
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_account_id: Option<u32>,
    pub regions: Vec<CloudRegionConfig>,
}

#[derive(Debug, Serialize)]
pub struct CloudRegionConfig {
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking_deployment_cidr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zones: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_availability_zones: Option<bool>,
}

/// Update subscription request
#[derive(Debug, Serialize)]
pub struct UpdateSubscriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_storage: Option<String>,
}
