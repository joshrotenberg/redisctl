//! Region models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Cloud region information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionInfo {
    pub region: String,
    pub provider: Option<String>,
    pub multiple_availability_zones: Option<bool>,
    pub preferred_availability_zones: Option<Vec<String>>,
    pub networking_deployment_cidr: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

