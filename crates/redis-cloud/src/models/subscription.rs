//! Subscription-related data models

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

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
///
/// # Examples
///
/// ```rust,no_run
/// use redis_cloud::{CreateSubscriptionRequest, CloudProviderConfig, CloudRegionConfig};
///
/// let request = CreateSubscriptionRequest::builder()
///     .name("production")
///     .payment_method_id(12345)
///     .memory_storage("ram")
///     .cloud_provider(
///         CloudProviderConfig::builder()
///             .provider("AWS")
///             .regions(vec![
///                 CloudRegionConfig::builder()
///                     .region("us-east-1")
///                     .multiple_availability_zones(true)
///                     .build()
///             ])
///             .build()
///     )
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateSubscriptionRequest {
    #[builder(setter(into))]
    pub name: String,
    pub payment_method_id: u32,
    #[builder(setter(into))]
    pub memory_storage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub persistent_storage_encryption: Option<bool>,
    pub cloud_provider: CloudProviderConfig,
}

#[derive(Debug, Serialize, TypedBuilder)]
pub struct CloudProviderConfig {
    #[builder(setter(into))]
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub cloud_account_id: Option<u32>,
    pub regions: Vec<CloudRegionConfig>,
}

#[derive(Debug, Serialize, TypedBuilder)]
pub struct CloudRegionConfig {
    #[builder(setter(into))]
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub networking_deployment_cidr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub preferred_availability_zones: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub multiple_availability_zones: Option<bool>,
}

/// Update subscription request
///
/// # Examples
///
/// ```rust,no_run
/// use redis_cloud::UpdateSubscriptionRequest;
///
/// let request = UpdateSubscriptionRequest::builder()
///     .name("production-updated")
///     .payment_method_id(54321)
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct UpdateSubscriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_method_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub memory_storage: Option<String>,
}
