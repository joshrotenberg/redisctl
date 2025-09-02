//! Region models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionInfo {
    // Common top-level fields
    #[serde(alias = "region")]
    pub id: String,
    pub name: Option<String>,
    pub provider: Option<String>,
    #[serde(alias = "isAvailable")]
    pub available: Option<bool>,

    // Arrays and details
    pub zones: Option<Vec<String>>,

    // Nested sections frequently returned by the API
    pub networking: Option<RegionNetworking>,
    pub pricing: Option<RegionPricing>,
    pub compliance: Option<Vec<String>>,

    // Limits and capabilities
    #[serde(rename = "maxInstances")]
    pub max_instances: Option<u32>,
    #[serde(rename = "diskTypes")]
    pub disk_types: Option<Vec<String>>,

    // Additional reason/details when unavailable
    pub reason: Option<String>,

    // Preserve any additional fields
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionNetworking {
    pub vpc: Option<bool>,
    #[serde(rename = "privateServiceConnect")]
    pub private_service_connect: Option<bool>,
    #[serde(rename = "transitGateway")]
    pub transit_gateway: Option<bool>,
    #[serde(rename = "supportedFeatures")]
    pub supported_features: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionPricing {
    pub currency: Option<String>,
    #[serde(rename = "dataTransfer")]
    pub data_transfer: Option<DataTransferPricing>,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransferPricing {
    // Inbound/outbound can be string/number depending on backend, keep flexible
    pub inbound: Option<Value>,
    pub outbound: Option<Value>,

    #[serde(flatten)]
    pub extra: Value,
}
