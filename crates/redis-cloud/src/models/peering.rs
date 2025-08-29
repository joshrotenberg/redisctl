//! VPC Peering-related data models

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

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
///
/// # Examples
///
/// ```rust,no_run
/// use redis_cloud::CreatePeeringRequest;
///
/// let request = CreatePeeringRequest::builder()
///     .subscription_id(123)
///     .provider("AWS")
///     .aws_account_id("123456789012")
///     .vpc_id("vpc-12345678")
///     .vpc_cidr("10.0.0.0/16")
///     .region("us-east-1")
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreatePeeringRequest {
    pub subscription_id: u32,
    #[builder(setter(into))]
    pub provider: String,
    #[builder(default, setter(into, strip_option))]
    pub aws_account_id: Option<String>,
    #[builder(setter(into))]
    pub vpc_id: String,
    #[builder(setter(into))]
    pub vpc_cidr: String,
    #[builder(setter(into))]
    pub region: String,
}
