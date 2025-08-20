//! VPC Peering-related data models

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// VPC Peering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudPeering {
    pub peering_id: String,
    pub subscription_id: u32,
    pub status: String,
    pub provider_peering_id: Option<String>,
    pub aws_account_id: Option<String>,
    pub vpc_id: Option<String>,
    pub vpc_cidr: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

/// Create peering request
#[derive(Debug, Serialize)]
pub struct CreatePeeringRequest {
    pub subscription_id: u32,
    pub provider: String,
    pub aws_account_id: Option<String>,
    pub vpc_id: String,
    pub vpc_cidr: String,
    pub region: String,
}
